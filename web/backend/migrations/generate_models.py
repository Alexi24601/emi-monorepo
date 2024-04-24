"""Python script to run a patched version of the diesel extended CLI to generate models.

Implementation details
----------------------
The diesel extended CLI can be used to generated the structs associated to the database tables. However, the
generated structs are not complete most commonly, as it does not come equipped with some of the postgres types.
Fortunately, this can be easily patched with some replace statements.

We start by running the extended CLI command:

```bash
diesel_ext --model --add-table-name > src/models.rs
```

Then, we need to handle the following replacements, plus adding on the top of the file the associated imports.
The imports need to be added solely when the replacements are effective, i.e. there is actually a change in the file,
otherwise we would add unnecessary imports and cause compilation warnings.

The replacements are defined in the file `replacements.json` and are applied to the generated file `src/models.rs`.
The structure of the JSON file is as follows:

```json
[
    {
        "search": "search_string",
        "replace": "replace_string",
        "imports": ["import1", "import2"]
    }
]
```

With a more concrete example:

```json
[
    {
        "search": "Option</* TODO: unknown type Nullable<Numrange> */>",
        "replace": "Option<Range<Numeric>>",
        "imports": [
            "use diesel::sql_types::Numeric;",
            "use diesel::sql_types::Range;"
        ]
    }
]
```

Since we also need to interface with the Frontend database which are NOT based
on Postgres, we also need to duplicate the code in the web commons and generate
the From implementations for the structs in the `src/models.rs` file. The `web_common`
structs will be generated in the `src/database/tables.rs` file in the `web_common` crate.
Since these structs are field-by-field identical, we can simply copy the structs while
ignoring the `#[derive(...)]` and `#[table_name = "..."]` attributes which would not be
applicable in the `web_common` crate. The `From` implementations will be generated in the
`src/models.rs` file in the `backend` crate, below each of the diesel-generated structs and
will make use of the full path to the struct in the `web_common` crate so to avoid conflicts.

"""

from typing import List, Tuple, Optional, Union
import psycopg2
import compress_json
import os
import re
import pandas as pd
import shutil
from tqdm.auto import tqdm
from densify_directory_counter import densify_directory_counter
from dotenv import load_dotenv
from retrieve_ncbi_taxon import retrieve_ncbi_taxon

def struct_name_from_table_name(table_name: str) -> str:
    """Convert the table name to the struct name."""
    if table_name.endswith("s"):
        table_name = table_name[:-1]
    return "".join(word.capitalize() for word in table_name.split("_"))


class PGIndex:

    def __init__(self, name: str, table_name: str, columns: List[str]):
        self.name = name
        self.table_name = table_name
        self.columns = columns

    def human_readable_columns(self) -> str:
        """Return the columns in a human-readable format."""
        last_column = self.columns[-1]
        if len(self.columns) == 1:
            return last_column
        return f"{', '.join(self.columns[:-1])} and {last_column}"


class PGIndices:

    def __init__(self, indices: List[PGIndex]):
        self.indices = indices
        self.foreign_keys_information = find_foreign_keys()

    def has_table(self, table_name: str) -> bool:
        if self.foreign_keys_information.is_view(table_name):
            view_columns = self.foreign_keys_information.extract_view_columns(
                table_name
            )
            # We seek an "id" column in the view columns
            for column in view_columns:
                if column.alias_name == "id":
                    return self.has_table(column.table_name)

        for index in self.indices:
            if index.table_name == table_name:
                return True
        return False

    def get_table(self, table_name: str) -> PGIndex:
        if self.foreign_keys_information.is_view(table_name):
            view_columns = self.foreign_keys_information.extract_view_columns(
                table_name
            )
            # We seek an "id" column in the view columns
            for column in view_columns:
                if column.alias_name == "id":
                    return self.get_table(column.table_name)
        
        for index in self.indices:
            if index.table_name == table_name:
                return index
        return None


class AttributeMetadata:

    def __init__(
        self,
        original_name: str,
        name: str,
        data_type: Union[str, "StructMetadata"],
        optional: bool,
    ):
        self.original_name = original_name
        self.name = name
        self._data_type = data_type
        self.optional = optional

    def is_undefined_nested_dependencies(self) -> bool:
        return not self.has_struct_data_type() and self.data_type().startswith("Nested")

    def has_struct_data_type(self) -> bool:
        return isinstance(self._data_type, StructMetadata)

    def format_data_type(self) -> str:
        data_type = self.data_type()

        if self.optional:
            return f"Option<{data_type}>"
        return data_type

    def data_type(self) -> str:
        if isinstance(self._data_type, StructMetadata):
            return self._data_type.name
        elif isinstance(self._data_type, str):
            return self._data_type

        raise ValueError("The data type must be either a string or a StructMetadata.")

    def implements_eq(self) -> bool:
        return (
            self._data_type not in ["f32", "f64"]
            or isinstance(self._data_type, StructMetadata)
            and self._data_type.can_implement_eq()
        )

    def __eq__(self, other: "AttributeMetadata") -> bool:
        return (
            self.name == other.name
            and self._data_type == other._data_type
            and self.optional == other.optional
        )

    def implements_clone(self) -> bool:
        return (
            self._data_type
            in [
                "bool",
                "i8",
                "i16",
                "i32",
                "i64",
                "i128",
                "u8",
                "u16",
                "u32",
                "u64",
                "u128",
                "f32",
                "f64",
                "Uuid",
                "String",
                "NaiveDateTime",
            ]
            or isinstance(self._data_type, StructMetadata)
            and self._data_type.can_implement_clone()
        )


class StructMetadata:

    def __init__(self, struct_name: str, table_name: str):
        self.name = struct_name
        self.table_name = table_name
        self.attributes: List[AttributeMetadata] = []
        self._derives: List[str] = []

    def has_undefined_nested_dependencies(self) -> bool:
        """Returns whether the struct has undefined nested dependencies.

        Implementative details
        -----------------------
        This method checks if any of the attributes of the struct
        is not a struct and starts with the word `Nested`.
        """
        return any(
            attribute.is_undefined_nested_dependencies()
            for attribute in self.attributes
        )

    def get_attribute_by_name(self, attribute_name: str) -> Optional[AttributeMetadata]:
        for attribute in self.attributes:
            if attribute.name == attribute_name:
                return attribute
        return None

    def capitalized_table_name(self) -> str:
        return "".join(word.capitalize() for word in self.table_name.split("_"))

    def is_nested(self) -> bool:
        return any(
            isinstance(attribute._data_type, StructMetadata)
            for attribute in self.attributes
        )

    def add_attribute(self, attribute_metadata: AttributeMetadata):
        self.attributes.append(attribute_metadata)

    def add_derive(self, derive: str):
        self._derives.append(derive)

    def contains_optional_fields(self) -> bool:
        return any(attribute.optional for attribute in self.attributes)

    def contains_only_optional_fields(self) -> bool:
        return all(attribute.optional for attribute in self.attributes)

    def derives(self) -> List[str]:
        derives = self._derives.copy()
        if self.can_implement_eq() and "Eq" not in self._derives:
            derives.append("Eq")
        if self.can_implement_clone() and "Clone" not in self._derives:
            derives.append("Clone")

        return derives

    def can_implement_clone(self) -> bool:
        return all(attribute.implements_clone() for attribute in self.attributes)

    def can_implement_eq(self) -> bool:
        return all(attribute.implements_eq() for attribute in self.attributes)

    def has_attribute(self, attribute: AttributeMetadata) -> bool:
        """Returns the type of the attribute"""
        return any(
            attribute == existing_attribute for existing_attribute in self.attributes
        )

    def is_nested(self) -> bool:
        return any(
            isinstance(attribute._data_type, StructMetadata)
            for attribute in self.attributes
        )


def get_cursor():
    """Get the cursor to the database."""
    dbname = os.getenv("POSTGRES_DB")
    user = os.getenv("POSTGRES_USER")
    password = os.getenv("POSTGRES_PASSWORD")
    port = os.getenv("PGPORT")
    # url = os.getenv("POSTGRES_URL")

    # Establishing a connection to the PostgreSQL database
    conn = psycopg2.connect(
        dbname=dbname,
        user=user,
        password=password,
        host="localhost",
        port=port,
    )

    return conn, conn.cursor()


def find_pg_trgm_indices() -> PGIndices:
    """Returns the list of indices that are of type `pg_trgm`."""
    conn, cursor = get_cursor()
    cursor.execute(
        """
        SELECT
            indexname AS index_name,
            tablename AS table_name,
            substring(indexdef from '\((.*)\)') AS columns_involved
        FROM
            pg_indexes
        WHERE
            indexdef ILIKE '%using gin%'
            AND indexdef ILIKE '%gin_trgm_ops%';
        """
    )
    indices = cursor.fetchall()
    pg_indices = []
    for index in indices:
        sanitized_coumn_names = []
        for column in index[2].split(", "):
            sanitized_coumn_names.append(column.split(" ")[0])

        pg_indices.append(PGIndex(index[0], index[1], sanitized_coumn_names))

    cursor.close()
    conn.close()

    return PGIndices(pg_indices)


def sql_type_to_rust_type(sql_type: str) -> str:
    """Convert the SQL type to the Rust type."""
    if sql_type == "uuid":
        return "uuid::Uuid"
    if sql_type == "integer":
        return "i32"
    raise NotImplementedError(f"The SQL type {sql_type} is not supported.")


class ViewColumn:
    
    def __init__(self, column_name: str, data_type: str, alias_name: str, table_name: str, nullable: bool):
        self.column_name = column_name
        self.data_type = data_type
        self.alias_name = alias_name
        self.table_name = table_name
        self.nullable = nullable


class TableMetadata:

    def __init__(self, table_metadata: pd.DataFrame):
        self.table_metadata = table_metadata
        self._view_names: List[str] = []
        self._table_names: List[str] = []

    def is_table(self, table_name: str) -> bool:
        """Returns whether the table is a table."""
        if table_name in self._view_names:
            return False
        if table_name in self._table_names:
            return True
        conn, cursor = get_cursor()
        cursor.execute(
            f"""
            SELECT
                table_name
            FROM
                information_schema.tables
            WHERE
                table_name = '{table_name}'
                AND table_type = 'BASE TABLE';
            """
        )
        is_table = cursor.fetchone() is not None
        cursor.close()
        conn.close()

        if is_table:
            self._table_names.append(table_name)

        return is_table

    def is_view(self, table_name: str) -> bool:
        """Returns whether the table is a view."""
        if table_name in self._view_names:
            return True
        if table_name in self._table_names:
            return False
        conn, cursor = get_cursor()
        cursor.execute(
            f"""
            SELECT
                table_name
            FROM
                information_schema.tables
            WHERE
                table_name = '{table_name}'
                AND table_type = 'VIEW';
            """
        )
        is_view = cursor.fetchone() is not None
        cursor.close()
        conn.close()

        if is_view:
            self._view_names.append(table_name)

        return is_view

    def extract_view_columns(self, view_name: str) -> List[ViewColumn]:
        """Returns list of columns, their alias and the original table name from a provided view name.

        # Example
        Suppose you have a simple view creation statement like this:

        ```sql
        CREATE VIEW view_name AS
        SELECT
            table_name.column_name AS alias_name
        FROM
            table_name;
        ```

        This function will return a list of tuples like this:

            ```python
            [("column_name", "alias_name", "table_name")]
            ```
        """
        conn, cursor = get_cursor()
        cursor.execute(
            f"""
            SELECT
                view_definition
            FROM
                information_schema.views
            WHERE
                table_name = '{view_name}';
            """
        )
        view_definition = cursor.fetchall()
        cursor.close()
        conn.close()

        if len(view_definition) == 0:
            raise ValueError(f"The view {view_name} does not exist.")

        view_definition = view_definition[0][0]
        view_definition = view_definition.replace("\n", " ")

        # Extract the columns from the view definition
        columns = re.findall(r"SELECT (.*?) FROM", view_definition)[0]
        columns = columns.split(", ")

        # For each column, we need to identify the original table name.
        table_name_mappings = {
            "thumbnail_documents": "documents",
            "profile_picture_documents": "documents",
        }

        extracted_columns: List[ViewColumn] = []
        for column in columns:
            if " AS " in column:
                original_column_name, alias_column_name = column.split(" AS ")
                if "." in original_column_name:
                    original_table_name, original_column_name = (
                        original_column_name.split(".")
                    )
                    original_table_name = original_table_name.strip()
                else:
                    continue
                
                remapped = False

                if not self.is_table(original_table_name) and not self.is_view(original_table_name):
                    original_table_name = table_name_mappings.get(original_table_name)
                    remapped = True
                    if original_table_name is None:
                        raise ValueError(f"The table {original_table_name} does not exist.")

                extracted_columns.append(ViewColumn(
                    column_name=original_column_name.strip(),
                    data_type=self.get_column_data_type(original_table_name, original_column_name.strip()),
                    alias_name=alias_column_name.strip(),
                    table_name=original_table_name.strip(),
                    nullable=remapped or self.is_nullable(original_table_name, original_column_name.strip())
                ))
            else:
                if "." in column:
                    original_table_name, original_column_name = column.split(".")
                    original_table_name = original_table_name.strip()
                else:
                    continue
                
                remapped = False

                if not self.is_table(original_table_name) and not self.is_view(original_table_name):
                    original_table_name = table_name_mappings.get(original_table_name)
                    remapped = True
                    if original_table_name is None:
                        raise ValueError(f"The table {original_table_name} does not exist.")

                extracted_columns.append(
                    ViewColumn(
                        column_name=original_column_name.strip(),
                        data_type=self.get_column_data_type(original_table_name, original_column_name.strip()),
                        alias_name=original_column_name.strip(),
                        table_name=original_table_name.strip(),
                        nullable=remapped or self.is_nullable(original_table_name, original_column_name.strip())
                    )
                )

        return extracted_columns

    def has_foreign_keys(self, table_name: str) -> bool:
        """Returns whether the table has foreign keys.

        Parameters
        ----------
        table_name : str
            The name of the table.

        Implementation details
        ----------------------
        This method checks if any of the columns in the table metadata
        associated with the table name have a non-null value in the
        `referenced_table` column. If any of the columns have a non-null
        value, then the table has foreign keys.
        """
        if self.is_view(table_name):
            for column in self.extract_view_columns(table_name):
                if column.column_name in self.get_foreign_keys(column.table_name):
                    return True
            return False

        primary_key_name, _ = self.get_primary_key_name_and_type(table_name)
        foreign_keys = self.get_foreign_keys(table_name)
        return any(foreign_key != primary_key_name for foreign_key in foreign_keys)

    def get_foreign_keys(self, table_name: str) -> List[str]:
        """Returns the foreign keys of the table.

        Parameters
        ----------
        table_name : str
            The name of the table.

        Implementation details
        ----------------------
        This method returns the list of columns in the table metadata
        associated with the table name that have a non-null value in the
        `referenced_table` column. These columns are the foreign keys
        of the table.
        """
        if self.is_view(table_name):
            foreign_keys = []
            for column in self.extract_view_columns(table_name):
                if column.column_name in self.get_foreign_keys(column.table_name) or self.is_primary_key(column.table_name, column.column_name):
                    foreign_keys.append(column.alias_name)
            return foreign_keys

        table_columns = self.table_metadata[
            self.table_metadata["referencing_table"] == table_name
        ]
        return table_columns[table_columns["referenced_table"].notnull()][
            "referencing_column"
        ].tolist()

    def get_foreign_key_table_name(self, table_name: str, column_name: str) -> str:
        """Returns the table that the foreign key references.

        Parameters
        ----------
        table_name : str
            The name of the table.
        column_name : str
            The name of the column.

        Implementation details
        ----------------------
        This method returns the value in the `referenced_table` column
        in the table metadata associated with the table name and column
        name. This value is the table that the foreign key references.
        """
        if self.is_view(table_name):
            for column in self.extract_view_columns(table_name):
                if column.alias_name == column_name:
                    return self.get_foreign_key_table_name(
                        column.table_name, column.column_name
                    )
            raise ValueError(
                f"The column {column_name} does not exist in the view {table_name}."
            )

        if self.is_primary_key(table_name, column_name):
            return table_name

        table_columns = self.table_metadata[
            (self.table_metadata["referencing_table"] == table_name)
            & (self.table_metadata["referencing_column"] == column_name)
        ]
        return table_columns["referenced_table"].values[0]

    def foreign_key_table_has_foreign_keys(
        self, table_name: str, foreign_key_column: str
    ) -> bool:
        """Returns whether the foreign key table has foreign keys.

        Parameters
        ----------
        table_name : str
            The name of the table.
        foreign_key_column : str
            The name of the foreign key column.

        Implementation details
        ----------------------
        This method checks if the foreign key table has foreign keys.
        """
        return self.has_foreign_keys(
            self.get_foreign_key_table_name(table_name, foreign_key_column)
        )

    def has_primary_key(self, table_name: str) -> bool:
        """Returns whether the table has a primary key.

        Parameters
        ----------
        table_name : str
            The name of the table.

        Implementation details
        ----------------------
        This method returns whether the table metadata associated with
        the table name has a non-null value in the `column_key` column.
        """
        return self.get_primary_key_name_and_type(table_name) is not None

    def is_primary_key(self, table_name: str, candidate_key: str) -> bool:
        """Returns whether the candidate key is the primary key of the table.

        Parameters
        ----------
        table_name : str
            The name of the table.
        candidate_key : str
            The candidate key.

        Implementation details
        ----------------------
        This method returns whether the candidate key is the primary key
        of the table metadata associated with the table name.
        """
        primary_key_name, _ = self.get_primary_key_name_and_type(table_name)
        return primary_key_name == candidate_key

    def get_column_data_type(self, table_name: str, column_name: str) -> str:
        """Returns the data type of the column.

        Parameters
        ----------
        table_name : str
            The name of the table.
        column_name : str
            The name of the column.
        """
        if self.is_view(table_name):
            for view_column in self.extract_view_columns(table_name):
                if view_column.alias_name == column_name:
                    return view_column.data_type

        _conn, cursor = get_cursor()

        cursor.execute(
            f"""
            SELECT
                data_type
            FROM
                information_schema.columns
            WHERE
                table_name = '{table_name}'
                AND column_name = '{column_name}';
            """
        )

        data_type = cursor.fetchone()[0]

        cursor.close()

        return data_type

    def is_nullable(self, table_name: str, column_name: str) -> bool:
        """Returns whether the column is nullable.

        Parameters
        ----------
        table_name : str
            The name of the table.
        column_name : str
            The name of the column.
        """
        if self.is_view(table_name):
            for view_column in self.extract_view_columns(table_name):
                if view_column.alias_name == column_name:
                    return view_column.nullable
        
        _conn, cursor = get_cursor()

        cursor.execute(
            f"""
            SELECT
                is_nullable
            FROM
                information_schema.columns
            WHERE
                table_name = '{table_name}'
                AND column_name = '{column_name}';
            """
        )

        is_nullable = cursor.fetchone()[0]

        cursor.close()

        return is_nullable == "YES"


    def get_primary_key_name_and_type(
        self, table_name: str
    ) -> Optional[Tuple[str, str]]:
        """Returns the name and type of the primary key of the table.

        Parameters
        ----------
        table_name : str
            The name of the table.

        Implementation details
        ----------------------
        This method returns the name and data type of the column in the
        table metadata associated with the table name that has the value
        `PRI` in the `column_key` column. This column is the primary key
        of the table.
        """
        if self.is_view(table_name):
            # We check if the view has an "id" column and if it does,
            # we return the primary key of the associated table.
            view_columns = self.extract_view_columns(table_name)
            for column in view_columns:
                if column.alias_name == "id":
                    return self.get_primary_key_name_and_type(column.table_name)
            return None
        _conn, cursor = get_cursor()

        cursor.execute(
            f"""
            SELECT
                kcu.column_name,
                data_type
            FROM
                information_schema.table_constraints AS tc
            JOIN
                information_schema.key_column_usage AS kcu
            ON
                tc.constraint_name = kcu.constraint_name
            JOIN
                information_schema.columns AS cols
            ON
                kcu.table_name = cols.table_name
                AND kcu.column_name = cols.column_name
            WHERE
                tc.table_name = '{table_name}'
                AND tc.constraint_type = 'PRIMARY KEY';
            """
        )

        primary_key = cursor.fetchone()

        cursor.close()

        return primary_key

    def get_columns(self, table_name: str) -> List[str]:
        """Returns the columns of the table."""
        if self.is_view(table_name):
            return [
                column[1]
                for column in self.extract_view_columns(table_name)
            ]
        
        _conn, cursor = get_cursor()
        cursor.execute(
            f"""
            SELECT
                column_name
            FROM
                information_schema.columns
            WHERE
                table_name = '{table_name}';
            """
        )

        columns = cursor.fetchall()
        cursor.close()

        return [column[0] for column in columns]


def find_foreign_keys() -> TableMetadata:
    """Returns the list of indices that are of type `pg_trgm`."""
    conn, cursor = get_cursor()
    cursor.execute(
        """
        SELECT
            tc.constraint_name AS constraint_name,
            tc.table_name AS referencing_table,
            kcu.column_name AS referencing_column,
            ccu.table_name AS referenced_table,
            ccu.column_name AS referenced_column,
            CASE
                WHEN tc.constraint_type = 'FOREIGN KEY' THEN 'NO'
                ELSE 'YES'
            END AS is_optional
        FROM
            information_schema.table_constraints AS tc
        JOIN information_schema.key_column_usage AS kcu
        ON
            tc.constraint_name = kcu.constraint_name
        JOIN information_schema.constraint_column_usage AS ccu
        ON
            tc.constraint_name = ccu.constraint_name
        WHERE
            tc.constraint_type = 'FOREIGN KEY';
        """
    )
    table_metadata = cursor.fetchall()

    columns = [desc[0] for desc in cursor.description]
    table_metadata = pd.DataFrame(table_metadata, columns=columns)

    cursor.close()
    conn.close()

    return TableMetadata(table_metadata)


def write_from_impls(
    path: str, table_type: str, struct_metadatas: List[StructMetadata]
):
    """Write the `From` implementations for the structs in the `src/models.rs` file."""

    if len(struct_metadatas) == 0:
        return

    if table_type not in ["tables", "views"]:
        raise ValueError("The table type must be either 'tables' or 'views'.")

    def get_struct_metadata(struct_metadata: str) -> StructMetadata:
        for struct in struct_metadatas:
            if struct.name == struct_metadata:
                return struct
        raise ValueError(
            f"The provided struct {struct_metadata} could " "not be found."
        )

    similarity_indices: PGIndices = find_pg_trgm_indices()
    table_metadatas = find_foreign_keys()

    with open(path, "r", encoding="utf8") as file:
        content = file.read()

    # After each struct ends, as defined by the `}` character, after
    # we have found a `struct` keyword, we write the `From` implementation
    # for the struct where we implement the conversion to the struct in the
    # `web_common` crate.

    impl_from_line = "impl From<{struct_name}> for web_common::database::{table_type}::{struct_name} {{\n"
    reverse_from = "impl From<web_common::database::{table_type}::{struct_name}> for {struct_name} {{\n"

    struct_name = None
    struct_field_names = []
    new_content = ""

    for line in content.split("\n"):
        new_content += line + "\n"

        if "struct" in line:
            # We have found a line such as "pub struct Archivable {"
            # and we need to extract the struct name.
            struct_name = line.split(" ")[2]

        # We are currently inside a struct
        if struct_name is not None:
            # And we have found the end of the struct
            if "}" in line:
                # We have found the end of the struct, and we write the
                # `From` implementation.
                new_content += "\n"
                new_content += impl_from_line.format(
                    struct_name=struct_name, table_type=table_type
                )
                new_content += f"    fn from(item: {struct_name}) -> Self {{\n"
                new_content += "        Self {\n"
                for field_name in struct_field_names:
                    new_content += f"            {field_name}: item.{field_name},\n"
                new_content += "        }\n"
                new_content += "    }\n"
                new_content += "}\n\n"

                new_content += reverse_from.format(
                    struct_name=struct_name, table_type=table_type
                )
                new_content += f"    fn from(item: web_common::database::{table_type}::{struct_name}) -> Self {{\n"
                new_content += "        Self {\n"
                for field_name in struct_field_names:
                    new_content += f"            {field_name}: item.{field_name},\n"
                new_content += "        }\n"
                new_content += "    }\n"
                new_content += "}\n\n"

                # We now generate the `get` method for the diesel struct.
                # This method receives the ID of the struct and returns the
                # struct from the database.
                #
                # ```rust
                # pub fn get(
                #     id: Uuid,
                #     connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
                # ) -> Result<Self, diesel::Error> {
                #     login_providers::dsl::login_providers
                #         .filter(login_providers::dsl::id.eq(provider_id))
                #         .first::<Self>(&mut conn)
                # }
                # ```

                struct = get_struct_metadata(struct_name)
                new_content += f"impl {struct.name} {{\n"

                # For all tables we implement a `all` method that retrieves all of
                # the rows in the table structured as a vector of the struct.

                new_content += "    /// Get all of the structs from the database.\n"
                new_content += "    ///\n"
                new_content += "    /// # Arguments\n"
                new_content += (
                    "    /// * `connection` - The connection to the database.\n"
                )
                new_content += "    ///\n"
                new_content += f"    pub fn all(\n"
                new_content += "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                new_content += "    ) -> Result<Vec<Self>, diesel::result::Error> {\n"
                if table_type == "tables":
                    new_content += f"        use crate::schema::{struct.table_name};\n"
                else:
                    new_content += (
                        f"        use crate::views::schema::{struct.table_name};\n"
                    )
                new_content += (
                    f"        {struct.table_name}::dsl::{struct.table_name}\n"
                )
                new_content += "            .load::<Self>(connection)\n"
                new_content += "    }\n"

                if table_metadatas.has_primary_key(struct.table_name):
                    primary_key_name, _ = table_metadatas.get_primary_key_name_and_type(
                        struct.table_name
                    )
                    primary_key = struct.get_attribute_by_name(primary_key_name)
                elif table_metadatas.is_view(struct.table_name):
                    primary_key = struct.get_attribute_by_name("id")

                if primary_key is not None:
                    new_content += (
                        "    /// Get the struct from the database by its ID.\n"
                    )
                    new_content += "    ///\n"
                    new_content += "    /// # Arguments\n"
                    new_content += f"    /// * `{primary_key.name}` - The ID of the struct to get.\n"
                    new_content += (
                        "    /// * `connection` - The connection to the database.\n"
                    )
                    new_content += "    ///\n"
                    new_content += "    pub fn get(\n"
                    new_content += (
                        f"        {primary_key.name}: {primary_key.data_type()},\n"
                    )
                    new_content += "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                    new_content += "    ) -> Result<Self, diesel::result::Error> {\n"
                    if table_type == "tables":
                        new_content += (
                            f"        use crate::schema::{struct.table_name};\n"
                        )
                    else:
                        new_content += (
                            f"        use crate::views::schema::{struct.table_name};\n"
                        )
                    new_content += (
                        f"        {struct.table_name}::dsl::{struct.table_name}\n"
                    )
                    new_content += f"            .filter({struct.table_name}::dsl::{primary_key.name}.eq({primary_key.name}))\n"
                    new_content += "            .first::<Self>(connection)\n"
                    new_content += "    }\n"

                # If this table implements the `pg_trgm` index, we also
                # provide the `search` method to search for the struct
                # by a given string. The method also receives a limit
                # parameter to limit the number of results and a threshold
                # parameter to set the similarity threshold.
                if similarity_indices.has_table(struct.table_name):
                    similarity_index: PGIndex = similarity_indices.get_table(
                        struct.table_name
                    )
                    new_content += "    /// Search for the struct by a given string.\n"
                    new_content += "    ///\n"
                    new_content += "    /// # Arguments\n"
                    new_content += "    /// * `query` - The string to search for.\n"
                    new_content += "    /// * `limit` - The maximum number of results, by default `10`.\n"
                    new_content += "    /// * `threshold` - The similarity threshold, by default `0.6`.\n"
                    new_content += (
                        "    /// * `connection` - The connection to the database.\n"
                    )
                    new_content += "    ///\n"
                    new_content += "    pub fn search(\n"
                    new_content += "        query: &str,\n"
                    new_content += "        limit: Option<i32>,\n"
                    new_content += "        threshold: Option<f64>,\n"
                    new_content += "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                    new_content += (
                        "    ) -> Result<Vec<Self>, diesel::result::Error> {\n"
                    )

                    new_content += "        let limit = limit.unwrap_or(10);\n"
                    new_content += "        let threshold = threshold.unwrap_or(0.3);\n"

                    # Since Diesel does not support the `similarity` Postgres function natively
                    # as part of the DSL query builder, we are forced to build the query manually
                    # in raw SQL. We use the `sql_query` function to execute the raw SQL query.
                    # Since the `sql_query` function needs to run a raw SQL query, we need to
                    # sanitize the input to avoid SQL injection attacks.

                    joined_field_names = ", ".join([
                        f"{struct.table_name}.{attribute.name}"
                        for attribute in struct.attributes
                    ])

                    # Since the similarity function only takes two arguments, we need to combine
                    # the scores of all of the columns. We do this by summing the scores of each
                    # column:
                    similarity_function = " + ".join(
                        f"similarity({similarity_index.table_name}.{column}, $1)"
                        for column in similarity_index.columns
                    )

                    if table_type == "views":
                        new_content += "        let similarity_query = concat!(\n"
                        new_content += f'            "WITH selected_ids AS (",\n'
                        new_content += f'            "SELECT {similarity_index.table_name}.id FROM {similarity_index.table_name} ",\n'
                        new_content += f'            "ORDER BY {similarity_function} DESC LIMIT $3",\n'
                        new_content += "         \")\",\n"
                        new_content += f'            "SELECT {joined_field_names} FROM {struct.table_name} ",\n'
                        new_content += (
                            f'            "JOIN selected_ids ON selected_ids.id = {struct.table_name}.id"\n'
                        )
                        new_content += "        );\n"
                    else:
                        new_content += "        let similarity_query = concat!(\n"
                        new_content += f'            "SELECT {joined_field_names} FROM {struct.table_name} ",\n'
                        new_content += f'            "ORDER BY {similarity_function} DESC LIMIT $3;"\n'
                        new_content += "        );\n"

                    new_content += "        diesel::sql_query(similarity_query)\n"
                    new_content += (
                        "            .bind::<diesel::sql_types::Text, _>(query)\n"
                    )
                    new_content += (
                        "            .bind::<diesel::sql_types::Float8, _>(threshold)\n"
                    )
                    new_content += (
                        "            .bind::<diesel::sql_types::Integer, _>(limit)\n"
                    )
                    new_content += "            .load(connection)\n"
                    new_content += "}\n"

                # Finally, we cluse the struct implementation.
                new_content += "}\n"

                struct_name = None
                struct_field_names = []

        # We are currently inside a struct, and we are looking for
        # the field names.
        if struct_name is not None and "pub" in line and ":" in line:
            # We have found a line such as `pub name: String,`
            # and we need to extract the field name.
            field_name = line.strip().split(" ")[1].strip(":")
            struct_field_names.append(field_name)

    with open(path, "w", encoding="utf8") as file:
        file.write(new_content)


def write_web_common_structs(
    path: str, target: str, enumeration: str
) -> List[StructMetadata]:
    """Write the structs in the target file in the `web_common` crate.

    Parameters
    ----------
    path : str
        The path from where to load the structs for the tables or views.
    target : str
        The path where to write the structs in the `web_common` crate.
    enumeration : str
        The name of the enumeration to write in the target file.
    """
    # The derive statements to include in the `src/database/tables.rs` document
    imports = [
        "use serde::Deserialize;",
        "use serde::Serialize;",
        "use uuid::Uuid;",
        "use chrono::NaiveDateTime;",
    ]

    table_metadatas = find_foreign_keys()

    # The derives to apply to the structs in the `src/database/tables.rs` document
    derives = ["Deserialize", "Serialize", "Clone", "Debug", "PartialEq"]

    # We check that we are currently executing in the `backend` crate
    # so to make sure that the relative path to the `web_common` crate
    # is correct.
    if not os.getcwd().endswith("backend"):
        raise Exception("This script must be executed in the `backend` crate.")

    tables = open(f"../web_common/src/database/{target}.rs", "w", encoding="utf8")

    similarity_indices: PGIndices = find_pg_trgm_indices()

    with open(path, "r", encoding="utf8") as file:
        models = file.read()

    for import_statement in imports:
        struct_imported = import_statement.rsplit(":", maxsplit=1)[0].strip(";")
        if struct_imported not in models:
            continue
        tables.write(f"{import_statement}\n")

    inside_struct = False

    # A dictionary to store the table names and their
    # respective structs.
    struct_metadatas: List[StructMetadata] = []
    last_table_name = None
    struct_has_just_finished = False

    for line in models.split("\n"):
        # We skip all lines beginning with `//!` as they are comments
        if line.startswith("//!"):
            continue

        # We find the table name by searching lines like
        # #[diesel(table_name = item_continuous_quantities)]
        if "table_name =" in line:
            last_table_name = line.split("=")[1].strip(" )]").split(":")[-1]

        # We determine whether a new struct has started
        # by checking if the `struct` keyword is present
        # in the line.
        if "struct" in line:
            struct_name = line.split(" ")[2]

            struct_metadata = StructMetadata(
                table_name=last_table_name,
                struct_name=struct_name,
            )

            inside_struct = True

        if inside_struct:
            # If the current line contains the id field,
            # we store the type of the id field.
            if "pub" in line and ":" in line:
                field_name = line.strip().split(" ")[1].strip(":")
                field_type = line.split(":")[1].strip(", ")
                option = False
                if field_type.startswith("Option<"):
                    option = True
                    field_type = field_type[7:-1]
                struct_metadata.add_attribute(
                    AttributeMetadata(
                        original_name=field_name,
                        name=field_name,
                        data_type=field_type,
                        optional=option,
                    )
                )

            # We determine whether the struct has ended
            # by checking if the `}` keyword is present
            # in the line.
            if "}" in line:
                inside_struct = False
                struct_has_just_finished = True

        if struct_has_just_finished:
            struct_has_just_finished = False
            # If the struct has finished, we can now begin with the
            # implementations for this struct.
            for derive in derives:
                struct_metadata.add_derive(derive)

            tables.write("#[derive(")
            tables.write(", ".join(struct_metadata.derives()))
            tables.write(")]\n")
            # We also write conditional derives for the frontend feature
            # that ask for the `frontend` feature to be enabled and derive
            # the yew::html::Properties trait for the struct.
            tables.write(
                '#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]\n'
            )

            tables.write(f"pub struct {struct_metadata.name} {{\n")
            for attribute in struct_metadata.attributes:
                tables.write(
                    f"    pub {attribute.name}: {attribute.format_data_type()},\n"
                )
            tables.write("}\n")

            struct_metadatas.append(struct_metadata)

            if enumeration == "Table":
                # This variant of the struct implementation is only
                # available when in the web_common is enabled the frontend
                # feature. It provides several methods including the use
                # of GlueSQL. Fortunately, it does not force us like Diesel
                # to create yet again another duplicate of the struct.
                tables.write('#[cfg(feature = "frontend")]\n')
                tables.write(f"impl {struct_metadata.name} {{\n")
                columns = ", ".join(
                    [attribute.name for attribute in struct_metadata.attributes]
                )

                # As first thing, we implement the `into_row` method for the struct. This method
                # converts the struct into a vector of `gluesql::core::ast_builder::ExprList`
                # variants, which are used to insert the struct into the GlueSQL database.
                types_and_methods = {
                    "i8": "gluesql::core::ast_builder::num({})",
                    "i16": "gluesql::core::ast_builder::num({})",
                    "i32": "gluesql::core::ast_builder::num({})",
                    "i64": "gluesql::core::ast_builder::num({})",
                    "i128": "gluesql::core::ast_builder::num({})",
                    "u8": "gluesql::core::ast_builder::num({})",
                    "u16": "gluesql::core::ast_builder::num({})",
                    "u32": "gluesql::core::ast_builder::num({})",
                    "u64": "gluesql::core::ast_builder::num({})",
                    "u128": "gluesql::core::ast_builder::num({})",
                    "f32": "gluesql::core::ast_builder::num({})",
                    "f64": "gluesql::core::ast_builder::num({})",
                    "String": "gluesql::core::ast_builder::text({})",
                    "Uuid": "gluesql::core::ast_builder::uuid({}.to_string())",
                    "bool": "({}.into())",
                    "NaiveDateTime": "gluesql::core::ast_builder::timestamp({}.to_string())",
                    "DateTime<Utc>": "gluesql::core::ast_builder::timestamp({}.to_string())",
                }

                update_types_and_methods = types_and_methods.copy()
                update_types_and_methods["bool"] = "{}"

                tables.write(
                    "    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {\n"
                )

                tables.write("        vec![\n")
                for attribute in struct_metadata.attributes:

                    if attribute.optional:
                        if attribute.data_type() in types_and_methods:
                            tables.write(
                                f"            match self.{attribute.name} {{\n"
                            )
                            tables.write(
                                f"                Some({attribute.name}) => {types_and_methods[attribute.data_type()].format(attribute.name)},\n"
                            )
                            tables.write(
                                "                None => gluesql::core::ast_builder::null(),\n"
                            )
                            tables.write("            },\n")
                        else:
                            raise NotImplementedError(
                                f"The type {attribute.data_type()} is not supported. "
                                f"The struct {struct_metadata.name} contains an {attribute.data_type()}. "
                            )
                    elif attribute.data_type() in types_and_methods:
                        tables.write(
                            f"            {types_and_methods[attribute.data_type()].format(f'self.{attribute.name}')},\n"
                        )
                    else:
                        raise NotImplementedError(
                            f"The type {attribute.data_type()} is not supported."
                        )

                tables.write("        ]\n")

                tables.write("    }\n\n")

                # We implement the `insert` method for the struct. This method
                # receives a connection to the GlueSQL database and inserts the
                # struct into the database.
                tables.write(
                    f"    /// Insert the {struct_metadata.name} into the database.\n"
                )
                tables.write("    ///\n")
                tables.write("    /// # Arguments\n")
                tables.write(
                    "    /// * `connection` - The connection to the database.\n"
                )
                tables.write("    ///\n")
                tables.write("    /// # Returns\n")
                tables.write(
                    f"    /// The number of rows inserted in table {struct_metadata.name}\n"
                )
                tables.write("    pub async fn insert<C>(\n")
                tables.write("        self,\n")
                tables.write("        connection: &mut gluesql::prelude::Glue<C>,\n")
                tables.write("    ) -> Result<usize, gluesql::prelude::Error> where\n")
                tables.write(
                    "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
                )
                tables.write("    {\n")
                tables.write("        use gluesql::core::ast_builder::*;\n")
                # We use the AST builder as much as possible so to avoid SQL injection attacks.
                tables.write(f'        table("{struct_metadata.table_name}")\n')
                tables.write("            .insert()\n")
                tables.write(f'            .columns("{columns}")\n')
                tables.write("            .values(vec![self.into_row()])\n")
                tables.write("            .execute(connection)\n")
                tables.write("            .await\n")
                tables.write("             .map(|payload| match payload {\n")
                tables.write(
                    "                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,\n"
                )
                tables.write(
                    '                 _ => unreachable!("Payload must be an Insert"),\n'
                )
                tables.write("             })\n")
                tables.write("    }\n\n")

                # We implement the `get` method for the struct. This method
                # receives the ID of the struct and a connection to the GlueSQL
                # database. The method returns the struct from the database.
                tables.write(
                    f"    /// Get {struct_metadata.name} from the database by its ID.\n"
                )
                tables.write("    ///\n")
                tables.write("    /// # Arguments\n")
                primary_key_name, primary_key_type = (
                    table_metadatas.get_primary_key_name_and_type(
                        struct_metadata.table_name
                    )
                )
                rust_primary_key_type = sql_type_to_rust_type(primary_key_type)

                tables.write(
                    f"    /// * `{primary_key_name}` - The ID of {struct_metadata.name} to get.\n"
                )
                tables.write(
                    "    /// * `connection` - The connection to the database.\n"
                )
                tables.write("    ///\n")
                tables.write("    pub async fn get<C>(\n")
                tables.write(f"        {primary_key_name}: {rust_primary_key_type},\n")
                tables.write("        connection: &mut gluesql::prelude::Glue<C>,\n")
                tables.write(
                    "    ) -> Result<Option<Self>, gluesql::prelude::Error> where\n"
                )
                tables.write(
                    "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
                )
                tables.write("    {\n")
                tables.write("        use gluesql::core::ast_builder::*;\n")
                # We use the AST builder as much as possible so to avoid SQL injection attacks.
                tables.write(
                    f'        let select_row = table("{struct_metadata.table_name}")\n'
                )
                tables.write("            .select()\n")
                tables.write(
                    f'            .filter(col("id").eq({primary_key_name}.to_string()))\n'
                )
                tables.write(f'            .project("{columns}")\n')
                tables.write("            .limit(1)\n")
                tables.write("            .execute(connection)\n")
                tables.write("            .await?;\n")
                tables.write("         Ok(select_row.select()\n")
                tables.write("            .unwrap()\n")
                tables.write("            .map(Self::from_row)\n")
                tables.write("            .collect::<Vec<_>>()\n")
                tables.write("            .pop())\n")
                tables.write("    }\n\n")

                # We implement the `delete` method for the struct. This method deletes
                # the struct from the GlueSQL database.
                tables.write(
                    f"    /// Delete {struct_metadata.name} from the database.\n"
                )
                tables.write("    ///\n")
                tables.write("    /// # Arguments\n")
                tables.write(
                    f"    /// * `{primary_key_name}` - The ID of the struct to delete.\n"
                )
                tables.write(
                    "    /// * `connection` - The connection to the database.\n"
                )
                tables.write("    ///\n")
                tables.write("    /// # Returns\n")
                tables.write("    /// The number of rows deleted.\n")
                tables.write("    pub async fn delete_from_id<C>(\n")
                tables.write(f"        {primary_key_name}: {rust_primary_key_type},\n")
                tables.write("        connection: &mut gluesql::prelude::Glue<C>,\n")
                tables.write("    ) -> Result<usize, gluesql::prelude::Error> where\n")
                tables.write(
                    "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
                )
                tables.write("    {\n")
                tables.write("        use gluesql::core::ast_builder::*;\n")
                # We use the AST builder as much as possible so to avoid SQL injection attacks.
                tables.write(f'        table("{struct_metadata.table_name}")\n')
                tables.write("            .delete()\n")
                tables.write('            .filter(col("id").eq(id.to_string()))\n')
                tables.write("            .execute(connection)\n")
                tables.write("            .await\n")
                tables.write("             .map(|payload| match payload {\n")
                tables.write(
                    "                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,\n"
                )
                tables.write(
                    '                 _ => unreachable!("Payload must be a Delete"),\n'
                )
                tables.write("             })\n")
                tables.write("    }\n\n")

                # We implement the `delete` method for the struct. This method deletes
                # the current instance of the struct from the GlueSQL database.
                tables.write(
                    f"    /// Delete the current instance of {struct_metadata.name} from the database.\n"
                )
                tables.write("    ///\n")
                tables.write("    /// # Arguments\n")
                tables.write(
                    "    /// * `connection` - The connection to the database.\n"
                )
                tables.write("    ///\n")
                tables.write("    /// # Returns\n")
                tables.write("    /// The number of rows deleted.\n")
                tables.write("    pub async fn delete<C>(\n")
                tables.write("        self,\n")
                tables.write("        connection: &mut gluesql::prelude::Glue<C>,\n")
                tables.write("    ) -> Result<usize, gluesql::prelude::Error> where\n")
                tables.write(
                    "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
                )
                tables.write("    {\n")
                tables.write(
                    "        Self::delete_from_id(self.id, connection).await\n"
                )
                tables.write("    }\n")

                # We implement the `update` method for the struct. This method updates
                # the struct in the GlueSQL database.
                tables.write("    /// Update the struct in the database.\n")
                tables.write("    ///\n")
                tables.write("    /// # Arguments\n")
                tables.write(
                    "    /// * `connection` - The connection to the database.\n"
                )
                tables.write("    ///\n")
                tables.write("    /// # Returns\n")
                tables.write("    /// The number of rows updated.\n")
                tables.write("    pub async fn update<C>(\n")
                tables.write("        self,\n")
                tables.write("        connection: &mut gluesql::prelude::Glue<C>,\n")
                tables.write("    ) -> Result<usize, gluesql::prelude::Error> where\n")
                tables.write(
                    "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
                )
                tables.write("    {\n")
                tables.write("        use gluesql::core::ast_builder::*;\n")
                # We use the AST builder as much as possible so to avoid SQL injection attacks.

                # First, we determine whether the current struct has at least an optional field.

                if struct_metadata.contains_optional_fields():
                    tables.write(
                        f'        let mut update_row = table("{struct_metadata.table_name}")\n'
                    )
                else:
                    tables.write(f'        table("{struct_metadata.table_name}")\n')
                tables.write("            .update()")

                if struct_metadata.contains_only_optional_fields():
                    raise NotImplementedError(
                        f"The struct {struct_metadata.name} does not contain any non-optional attributes. "
                        "It is not well defined how to update a struct without any attributes."
                    )

                for attribute in struct_metadata.attributes:
                    if attribute.optional:
                        # We handle this in the next loop
                        continue
                    if attribute.data_type() in update_types_and_methods:
                        conversion = update_types_and_methods[
                            attribute.data_type()
                        ].format(f"self.{attribute.name}")
                        tables.write(
                            f'        \n.set("{attribute.name}", {conversion})'
                        )
                    else:
                        raise NotImplementedError(
                            f"The type {attribute.data_type()} is not supported."
                            f"The struct {struct_metadata.name} contains an {attribute.data_type()}."
                        )

                if struct_metadata.contains_optional_fields():
                    tables.write(";\n")

                # After all of the non-optional fields, we handle the optional fields.
                for attribute in struct_metadata.attributes:
                    if not attribute.optional:
                        continue
                    conversion = update_types_and_methods[attribute.data_type()].format(
                        f"self.{attribute.name}"
                    )
                    if attribute.data_type() in update_types_and_methods:
                        tables.write(
                            f"        if let Some({attribute.name}) = self.{attribute.name} {{\n"
                        )
                        tables.write(
                            f'            update_row = update_row.set("{attribute.name}", {update_types_and_methods[attribute.data_type()].format(attribute.name)});\n'
                        )
                        tables.write("        }\n")
                    else:
                        raise NotImplementedError(
                            f"The type {attribute.data_type()} is not supported. "
                            f"The struct {attribute.name} contains an {attribute.data_type()}. "
                        )

                if struct_metadata.contains_optional_fields():
                    tables.write("            update_row.execute(connection)\n")
                else:
                    tables.write("            .execute(connection)\n")
                tables.write("            .await\n")
                tables.write("             .map(|payload| match payload {\n")
                tables.write(
                    "                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,\n"
                )
                tables.write(
                    '                 _ => unreachable!("Expected Payload::Update")\n'
                )
                tables.write("})\n")
                tables.write("    }\n\n")

                # Next, we implement the `update_or_insert` method for the struct. This method
                # inserts the struct into the GlueSQL database if it does not exist, otherwise
                # it updates the struct in the database.
                tables.write(
                    "    /// Update the struct in the database if it exists, otherwise insert it.\n"
                )
                tables.write("    ///\n")
                tables.write("    /// # Arguments\n")
                tables.write(
                    "    /// * `connection` - The connection to the database.\n"
                )
                tables.write("    ///\n")
                tables.write("    /// # Returns\n")
                tables.write("    /// The number of rows updated or inserted.\n")
                tables.write("    pub async fn update_or_insert<C>(\n")
                tables.write("        self,\n")
                tables.write("        connection: &mut gluesql::prelude::Glue<C>,\n")
                tables.write("    ) -> Result<usize, gluesql::prelude::Error> where\n")
                tables.write(
                    "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
                )
                tables.write("    {\n")
                tables.write(
                    "        let number_of_rows = self.clone().update(connection).await?;\n"
                )
                tables.write("        if number_of_rows == 0 {\n")
                tables.write("            self.insert(connection).await\n")
                tables.write("        } else {\n")
                tables.write("            Ok(number_of_rows)\n")
                tables.write("        }\n")
                tables.write("    }\n")

                # We implement the `all` method for the struct. This method returns all of the
                # structs in the GlueSQL database.
                tables.write(
                    f"    /// Get all {struct_metadata.name} from the database.\n"
                )
                tables.write("    ///\n")
                tables.write("    /// # Arguments\n")
                tables.write(
                    "    /// * `connection` - The connection to the database.\n"
                )
                tables.write("    ///\n")
                tables.write("    pub async fn all<C>(\n")
                tables.write("        connection: &mut gluesql::prelude::Glue<C>,\n")
                tables.write(
                    "    ) -> Result<Vec<Self>, gluesql::prelude::Error> where\n"
                )
                tables.write(
                    "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
                )
                tables.write("    {\n")
                tables.write("        use gluesql::core::ast_builder::*;\n")
                tables.write(
                    f'        let select_row = table("{struct_metadata.table_name}")\n'
                )
                tables.write("            .select()\n")
                tables.write(f'            .project("{columns}")\n')
                tables.write("            .execute(connection)\n")
                tables.write("            .await?;\n")
                tables.write("        Ok(select_row.select()\n")
                tables.write("            .unwrap()\n")
                tables.write("            .map(Self::from_row)\n")
                tables.write("            .collect::<Vec<_>>())\n")
                tables.write("    }\n")

                # We implement the `from_row` method for the struct. This method
                # receives a row from the GlueSQL database, which is a `HashMap<&str, &&Value>`.
                # The method returns the struct from the row.
                tables.write(
                    "    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {\n"
                )
                tables.write("        Self {\n")

                clonables = {
                    "bool": "Bool",
                    "i8": "I8",
                    "i16": "I16",
                    "i32": "I32",
                    "i64": "I64",
                    "i128": "I128",
                    "u8": "U8",
                    "u16": "U16",
                    "u32": "U32",
                    "u64": "U64",
                    "u128": "U128",
                    "f32": "F32",
                    "f64": "F64",
                    "String": "Str",
                    "NaiveDateTime": "Timestamp",
                }

                for attribute in struct_metadata.attributes:
                    if attribute.format_data_type() == "Uuid":
                        tables.write(
                            f'            {attribute.name}: match row.get("{attribute.name}").unwrap() {{\n'
                        )
                        tables.write(
                            f"                gluesql::prelude::Value::Uuid({attribute.name}) => Uuid::from_u128(*{attribute.name}),\n"
                        )
                        tables.write(
                            '                _ => unreachable!("Expected Uuid"),\n'
                        )
                        tables.write("            },\n")
                    elif attribute.format_data_type() == "Option<Uuid>":
                        tables.write(
                            f'            {attribute.name}: match row.get("{attribute.name}").unwrap() {{\n'
                        )
                        tables.write(
                            "                gluesql::prelude::Value::Null => None,\n"
                        )
                        tables.write(
                            f"                gluesql::prelude::Value::Uuid({attribute.name}) => Some(Uuid::from_u128(*{attribute.name})),\n"
                        )
                        tables.write(
                            '                _ => unreachable!("Expected Uuid"),\n'
                        )
                        tables.write("            },\n")
                    elif attribute.implements_clone():
                        if attribute.optional:
                            tables.write(
                                f'            {attribute.name}: match row.get("{attribute.name}").unwrap() {{\n'
                            )
                            tables.write(
                                "                gluesql::prelude::Value::Null => None,\n"
                            )
                            tables.write(
                                f"                gluesql::prelude::Value::{clonables[attribute.data_type()]}({attribute.name}) => Some({attribute.name}.clone()),\n"
                            )
                            tables.write(
                                f'                _ => unreachable!("Expected {clonables[attribute.data_type()]}")\n'
                            )
                            tables.write("            },\n")
                        else:
                            tables.write(
                                f'            {attribute.name}: match row.get("{attribute.name}").unwrap() {{\n'
                            )
                            tables.write(
                                f"                gluesql::prelude::Value::{clonables[attribute.data_type()]}({attribute.name}) => {attribute.name}.clone(),\n"
                            )
                            tables.write(
                                f'                _ => unreachable!("Expected {clonables[attribute.data_type()]}")\n'
                            )
                            tables.write("            },\n")
                    else:
                        raise NotImplementedError(
                            f"Found an unsupported attribute type for the struct {struct_name}: {attribute.data_type()} "
                            f"for the attribute {attribute.name}."
                        )
                tables.write("        }\n")
                tables.write("    }\n")

                # And finally we close the struct implementation
                tables.write("}\n")

    tables.close()

    return struct_metadatas


def get_view_names() -> List[str]:
    """Returns list of view names.

    Implementative details
    -------------------------
    The view names are extracted from the `migrations` directory. The view names are extracted
    from the `up.sql` file in each of the directories. The view names are extracted by searching
    for the `CREATE VIEW` statements in the SQL file.
    """
    view_names = []
    for directory in os.listdir("migrations"):
        if not os.path.isdir(f"migrations/{directory}"):
            continue
        with open(f"migrations/{directory}/up.sql", "r", encoding="utf8") as file:
            content = file.read()
        for line in content.split("\n"):
            if "CREATE VIEW" in line or "CREATE MATERIALIZED VIEW" in line:
                view_name = line.rsplit(" ", maxsplit=2)[1]
                view_names.append(view_name)
    return view_names


def get_views(cursor) -> List[str]:
    """Return list with the view names"""
    cursor.execute(
        "SELECT table_name FROM information_schema.views WHERE table_schema = 'public';"
    )
    views = cursor.fetchall()
    return views


def map_postgres_to_rust_type(pg_type):
    pg_to_rust_types = {
        "uuid": "diesel::sql_types::Uuid",
        "text": "diesel::sql_types::Text",
        "timestamp without time zone": "diesel::sql_types::Timestamp",
        "character varying": "diesel::sql_types::Text",
        "integer": "diesel::sql_types::Integer",
    }

    if pg_type in pg_to_rust_types:
        return pg_to_rust_types[pg_type]

    raise NotImplementedError(f'Postgres type "{pg_type}" is not supported.')


def generate_diesel_schema(view_name: str, columns: List[ViewColumn]) -> str:
    schema_code = "diesel::table! {\n"
    schema_code += f"    {view_name} (id) {{\n"
    for column in columns:
        if column.nullable:
            schema_code += (
                f"        {column.alias_name} -> diesel::sql_types::Nullable<{map_postgres_to_rust_type(column.data_type)}>,\n"
            )
        else:
            schema_code += (
                f"        {column.alias_name} -> {map_postgres_to_rust_type(column.data_type)},\n"
            )
    schema_code += "    }\n"
    schema_code += "}\n"
    return schema_code


def generate_view_schema():
    """Generate the view schema.

    Implementative details
    -------------------------
    We generate the views by connecting to the database and querying the `information_schema`
    tables. We then write the views to the file `src/views/schema.rs`. The database is a postgres
    database, and the connection string is read from the environment variable `DATABASE_URL`.
    """
    # We load the data from the environment variables from the `.env` file
    # at `../.env`.
    conn, cursor = get_cursor()

    # Getting the list of views
    views = get_views(cursor)
    table_metadata = find_foreign_keys()

    # We open the file to write the schema
    schema_file = open("src/views/schema.rs", "w", encoding="utf8")

    # Generating Diesel schema for each view
    for view in views:
        view_name = view[0]
        columns = table_metadata.extract_view_columns(view_name)
        schema_code = generate_diesel_schema(view_name, columns)
        schema_file.write(schema_code + "\n")

    # Closing the cursor and connection
    cursor.close()
    conn.close()


def check_schema_completion():
    """Check the view schema completion.

    Implementative details
    -------------------------
    Diesel does not support the `CREATE VIEW` statements, and as such we need to manually
    create the views in the schema file `src/views/schema.rs`. This script check that all
    of the view names are present in the schema file.
    """
    view_names = get_view_names()
    with open("src/views/schema.rs", "r", encoding="utf8") as file:
        content = file.read()
    for view_name in view_names:
        if view_name not in content:
            raise NotImplementedError(
                f"View {view_name} is not present in the schema file."
            )


def generate_view_structs():
    """Generate the view structs.

    Implementative details
    -------------------------
    Since Diesel solely supports the generation of the table structs, we need to use
    this custom script to generate the view structs. The view structs are generated
    starting from the schema file `src/views/schema.rs` and are written to the file
    `src/views/views.rs`. The view structs are generated by copying the views structs
    defined in the views schema, with the data types appropriately changed to match the
    view schema.

    An example of a schema entry is:

    ```rust
    diesel::table! {
        users_view (id) {
            id -> Uuid,
            first_name -> Nullable<Text>,
            middle_name -> Nullable<Text>,
            last_name -> Nullable<Text>,
            created_at -> Timestamp,
            updated_at -> Timestamp,
        }
    }
    ```
    """

    with open("src/views/schema.rs", "r", encoding="utf8") as file:
        schema = file.read()

    views = open("src/views/views.rs", "w", encoding="utf8")

    if len(schema) == 0:
        views.close()
        return

    data_types = {
        "diesel::sql_types::Uuid": "Uuid",
        "diesel::sql_types::Text": "String",
        "diesel::sql_types::Timestamp": "NaiveDateTime",
        "diesel::sql_types::Integer": "i32",
    }

    imports = [
        "use serde::Deserialize;",
        "use serde::Serialize;",
        "use diesel::Queryable;",
        "use diesel::QueryableByName;",
        "use uuid::Uuid;",
        "use chrono::NaiveDateTime;",
        "use diesel::r2d2::PooledConnection;",
        "use diesel::r2d2::ConnectionManager;",
        "use diesel::prelude::*;",
    ]

    derives = [
        "Deserialize",
        "Serialize",
        "Debug",
        "PartialEq",
        "Queryable",
        "QueryableByName",
    ]

    for import_statement in imports:
        views.write(f"{import_statement}\n")

    view_structs = []
    last_line_was_table = False
    brackets_count = 0

    for line in schema.split("\n"):
        if "{" in line:
            brackets_count += 1
        if "}" in line:
            brackets_count -= 1

        if last_line_was_table:
            view_name = line.split("(")[0].strip(" ")
            view_struct = StructMetadata(
                struct_name=struct_name_from_table_name(view_name),
                table_name=view_name,
            )
            view_structs.append(view_struct)

        if "diesel::table! {" in line:
            last_line_was_table = True
            continue
        else:
            last_line_was_table = False

        if "->" in line:
            (attribute, data_type) = line.strip(" ,").split(" -> ")
            optional = False
            if "Nullable<" in data_type:
                optional = True
                data_type = data_type.split("Nullable<", maxsplit=1)[1].strip(">")
            view_struct.add_attribute(
                AttributeMetadata(
                    original_name=attribute,
                    name=attribute,
                    data_type=data_types[data_type],
                    optional=optional
                )
            )

        # If we have found the end of the struct, we write the struct
        # to the file.
        if brackets_count == 0 and view_name is not None:
            # First, we generate the derives.
            for derive in derives:
                view_struct.add_derive(derive)
            views.write("#[derive(")
            views.write(", ".join(view_struct.derives()))
            views.write(")]\n")

            # Then, we write the table_name attribute to link
            # the struct to the view.
            views.write(
                f"#[diesel(table_name = crate::views::schema::{view_struct.table_name})]\n"
            )

            # We write the struct definition
            views.write(f"pub struct {view_struct.name} {{\n")
            for attribute in view_struct.attributes:
                views.write(f"    pub {attribute.name}: {attribute.format_data_type()},\n")
            views.write("}\n\n")

        if brackets_count == 0:
            attributes = {}
            view_name = None

    view_names_from_sql = get_view_names()
    for view_struct in view_structs:
        assert (
            view_struct.table_name in view_names_from_sql
        ), f'View "{view_struct.table_name}" is not present in the "schema.rs" file.'

    views.close()


def generate_nested_structs(
    path: str, struct_metadatas: List[StructMetadata]
) -> List[StructMetadata]:
    """Generate the nested structs.

    Implementative details
    -------------------------
    Normally, a table struct is generated from a row in the database. However, in some cases,
    a table row may contain a reference id to another table. In this case, we generate a nested
    struct for the referenced table. Depending on whether this referenced row contains also a
    reference to another table, we may generate the nested struct version of the referenced row
    or the flat version, i.e. the row itself.

    For each table, we query the postgres to get the foreign keys. We then generate the nested
    structs for the referenced tables. The nested structs are written to the file `src/models.rs`.
    """
    tables_metadata = find_foreign_keys()
    similarity_indices: PGIndices = find_pg_trgm_indices()

    # We open the file to write the nested structs
    tables = open(path, "w", encoding="utf8")

    # Preliminarly, we write a docstring at the very head
    # of this submodule to explain what it does and warn the
    # reader not to write anything in this file as it is
    # automatically generated.
    tables.write(
        "//! This module contains the nested structs for the database tables.\n"
    )
    tables.write("//!\n")
    tables.write(
        "//! This file is automatically generated. Do not write anything here.\n"
    )
    tables.write("\n")

    # We start with the necessary imports.
    imports = [
        "use serde::Deserialize;",
        "use serde::Serialize;",
        "use diesel::r2d2::ConnectionManager;",
        "use diesel::r2d2::PooledConnection;",
        "use crate::models::*;",
        "use crate::views::views::*;",
    ]

    for import_statement in imports:
        tables.write(f"{import_statement}\n")

    def get_struct_by_table_name(table_name: str) -> StructMetadata:
        for struct in struct_metadatas:
            if struct.table_name == table_name:
                return struct
        raise ValueError(f"Table name {table_name} not found in the struct metadata.")

    new_struct_metadatas = []

    # For each of the struct, we generated the Nested{struct_name} version
    # if the struct contains a reference to another struct.
    for struct in tqdm(
        struct_metadatas,
        desc="Generating nested structs",
        leave=False
    ):
        # If the struct does not have any foreign keys, we skip it
        if not tables_metadata.has_foreign_keys(
            struct.table_name
        ) and not tables_metadata.is_view(struct.table_name):
            continue

        foreign_keys = tables_metadata.get_foreign_keys(struct.table_name)

        primary_key_attribute = None
        if tables_metadata.has_primary_key(struct.table_name):
            # We implement the `get` method, which returns the nested struct
            # from a provided row primary key.
            primary_key_attribute, primary_key_type = (
                tables_metadata.get_primary_key_name_and_type(struct.table_name)
            )
            rust_primary_key_type = sql_type_to_rust_type(primary_key_type)

            # If all of the foreign keys are equal to the primary key, we skip
            # the struct as it is a self-referencing struct.
            if all(fk == primary_key_attribute for fk in foreign_keys):
                continue

        else:
            # If the table does not have a primary key, as may happen in the context
            # of a view, we use the attribyte `id` as the primary key.
            for attribute in struct.attributes:
                if attribute.name == "id":
                    primary_key_attribute = "id"
                    primary_key_type = attribute.data_type()
                    rust_primary_key_type = primary_key_type
                    break
            if primary_key_attribute is None:
                raise ValueError(
                    f"Table {struct.table_name} does not have a primary key nor an `id` attribute. "
                    f"It has the following attributes: {struct.attributes}"
                )

        if primary_key_attribute not in foreign_keys:
            foreign_keys.append(primary_key_attribute)

        nested_struct_name = f"Nested{struct.name}"
        new_struct_metadata = StructMetadata(nested_struct_name, struct.table_name)

        # We write the Nested{struct_name} struct
        tables.write("#[derive(")
        new_struct_metadata.add_derive("Debug")
        new_struct_metadata.add_derive("Serialize")
        new_struct_metadata.add_derive("Deserialize")
        new_struct_metadata.add_derive("PartialEq")
        tables.write(", ".join(new_struct_metadata.derives()))
        tables.write(")]\n")

        tables.write(f"pub struct {nested_struct_name} {{\n")
        for attribute in struct.attributes:

            # If the current attribute is among the foreign keys, we replace it
            # with the foreign struct. This struct may be also nested if the foreign
            # table has foreign keys, which we check by using the `has_foreign_keys`
            # method of the `tables_metadata` object.
            if attribute.name in foreign_keys:
                if attribute.name == primary_key_attribute:
                    foreign_struct = struct
                    normalized_attribute_name = "inner"
                else:
                    foreign_key_table_name = tables_metadata.get_foreign_key_table_name(
                        struct.table_name, attribute.name
                    )
                    normalized_attribute_name = attribute.name
                    foreign_struct = get_struct_by_table_name(foreign_key_table_name)

                if normalized_attribute_name.endswith("_id"):
                    normalized_attribute_name = normalized_attribute_name[:-3]
                if attribute.name != primary_key_attribute and (
                    tables_metadata.foreign_key_table_has_foreign_keys(
                        struct.table_name, attribute.name
                    )
                    or tables_metadata.is_view(struct.table_name)
                    and primary_key_attribute == "id"
                ):
                    # If the nested version of the foreign struct is already present,
                    # we cannot use it save risking the struct be extremely nested.
                    # Think for instance a leaf taxon struct containing its parent taxon
                    # and the parent taxon containing its parent taxon and so on.
                    if struct.name == foreign_struct.name:
                        new_attribute = AttributeMetadata(
                            original_name=attribute.name,
                            name=normalized_attribute_name,
                            data_type=foreign_struct,
                            optional=attribute.optional,
                        )
                    else:
                        new_attribute = AttributeMetadata(
                            original_name=attribute.name,
                            name=normalized_attribute_name,
                            data_type=f"Nested{foreign_struct.name}",
                            optional=attribute.optional,
                        )
                    tables.write(
                        f"    pub {new_attribute.name}: {new_attribute.format_data_type()},\n"
                    )
                    new_struct_metadata.add_attribute(new_attribute)
                else:
                    new_attribute = AttributeMetadata(
                        original_name=attribute.name,
                        name=normalized_attribute_name,
                        data_type=foreign_struct,
                        optional=attribute.optional,
                    )
                    tables.write(
                        f"    pub {new_attribute.name}: {new_attribute.format_data_type()},\n"
                    )
                    new_struct_metadata.add_attribute(new_attribute)
            else:
                continue
        tables.write("}\n\n")

        # We implement the all for the nested structs
        tables.write(
            f"impl {nested_struct_name} {{\n"
            "    /// Get all the nested structs from the database.\n"
            "    ///\n"
            "    /// # Arguments\n"
            "    /// * `connection` - The database connection.\n"
            "    pub fn all(\n"
            "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,\n"
            "    ) -> Result<Vec<Self>, diesel::result::Error> {\n"
            f"        let flat_structs = {struct.name}::all(connection)?;\n"
            "        let mut nested_structs = Vec::new();\n"
            "        for flat_struct in flat_structs {\n"
            "            nested_structs.push(Self {\n"
        )
        for attribute in new_struct_metadata.attributes:
            if attribute.name == "inner":
                continue
            if (
                attribute.data_type() == new_struct_metadata.name
                or struct.has_attribute(attribute)
            ):
                tables.write(
                    f"            {attribute.name}: flat_struct.{attribute.name},\n"
                )
                continue
            if attribute.optional:
                tables.write(
                    f"                {attribute.name}: flat_struct.{attribute.original_name}.map(|flat_struct| {attribute.data_type()}::get(flat_struct, connection)).transpose()?,\n"
                )
            else:
                tables.write(
                    f"                {attribute.name}: {attribute.data_type()}::get(flat_struct.{attribute.original_name}, connection)?,\n"
                )
        if any(
            attribute.name == primary_key_attribute for attribute in struct.attributes
        ):
            tables.write(f"                inner: flat_struct,\n")

        tables.write(
            "            });\n"
            "        }\n"
            "        Ok(nested_structs)\n"
            "    }\n"
            "}\n"
        )

        tables.write(
            f"impl {new_struct_metadata.name} {{\n"
            "    /// Get the nested struct from the provided primary key.\n"
            "    ///\n"
            "    /// # Arguments\n"
            "    /// * `id` - The primary key of the row.\n"
            "    /// * `connection` - The database connection.\n"
            "    pub fn get(\n"
            f"        id: {rust_primary_key_type},\n"
            "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,\n"
            "    ) -> Result<Self, diesel::result::Error>\n"
            "    {\n"
            f"        let flat_struct = {struct.name}::get(id, connection)?;\n"
            "        Ok(Self {\n"
        )
        for attribute in new_struct_metadata.attributes:
            if (
                attribute.data_type() == new_struct_metadata.name
                or struct.has_attribute(attribute)
            ):
                tables.write(
                    f"            {attribute.name}: flat_struct.{attribute.name},\n"
                )
                continue
            if attribute.optional:
                tables.write(
                    f"            {attribute.name}: flat_struct.{attribute.original_name}.map(|flat_struct| {attribute.data_type()}::get(flat_struct, connection)).transpose()?,\n"
                )
            else:
                tables.write(
                    f"            {attribute.name}: {attribute.data_type()}::get(flat_struct.{attribute.original_name}, connection)?,\n"
                )
        tables.write("        })\n" "    }\n" "}\n")

        # If there is an index on the table, we implement the search method that
        # calls search on the flat version of the struct and then iterates on the
        # primary keys of the results and constructs the nested structs by calling
        # the `get` method several times.
        if similarity_indices.has_table(struct.table_name):
            tables.write(
                f"impl Nested{struct.name} {{\n"
                "    /// Search the table by the query.\n"
                "    ///\n"
                "    /// # Arguments\n"
                "    /// * `query` - The string to search for.\n"
                "    /// * `limit` - The maximum number of results, by default `10`.\n"
                "    /// * `threshold` - The similarity threshold, by default `0.6`.\n"
                "    pub fn search(\n"
                "        query: &str,\n"
                "        limit: Option<i32>,\n"
                "        threshold: Option<f64>,\n"
                "        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,\n"
                "    ) -> Result<Vec<Self>, diesel::result::Error> {\n"
                f"        let flat_structs = {struct.name}::search(query, limit, threshold, connection)?;\n"
                "        let mut nested_structs = Vec::new();\n"
                "        for flat_struct in flat_structs {\n"
                "            nested_structs.push(Self::get(flat_struct.id, connection)?);\n"
                "        }\n"
                "        Ok(nested_structs)\n"
                "    }\n"
                "}\n"
            )

        # We implement the bidirectional From methods for the nested struct
        # present in the web_common crate, which does not use Diesel or its
        # structs, but the web_common version of the structs.
        tables.write(
            f"impl From<web_common::database::nested_models::Nested{struct.name}> for Nested{struct.name} {{\n"
            f"    fn from(item: web_common::database::nested_models::Nested{struct.name}) -> Self {{\n"
            "        Self {\n"
        )
        for attribute in new_struct_metadata.attributes:
            if attribute.optional:
                tables.write(
                    f"            {attribute.name}: item.{attribute.name}.map(|item| item.into()),\n"
                )
            else:
                tables.write(
                    f"            {attribute.name}: item.{attribute.name}.into(),\n"
                )
        tables.write("        }\n" "    }\n" "}\n")

        tables.write(
            f"impl From<Nested{struct.name}> for web_common::database::nested_models::Nested{struct.name} {{\n"
            f"    fn from(item: Nested{struct.name}) -> Self {{\n"
            "        Self {\n"
        )
        for attribute in new_struct_metadata.attributes:
            if attribute.optional:
                tables.write(
                    f"            {attribute.name}: item.{attribute.name}.map(|item| item.into()),\n"
                )
            else:
                tables.write(
                    f"            {attribute.name}: item.{attribute.name}.into(),\n"
                )
        tables.write("        }\n" "    }\n" "}\n")

        new_struct_metadatas.append(new_struct_metadata)

    tables.close()

    # We replace until convergence the data type of the structs with the structs themselves.
    # This is necessary as the nested structs may contain references to other structs, which
    # in turn may contain references to other structs and so on.

    changed = True

    while changed:
        changed = False
        updated_struct_metadatas = []
        for new_struct in new_struct_metadatas:
            if not new_struct.has_undefined_nested_dependencies():
                updated_struct_metadatas.append(new_struct)
                continue
            new_attributes = []
            converged = True
            for attribute in new_struct.attributes:
                if attribute.is_undefined_nested_dependencies():
                    for struct in new_struct_metadatas + struct_metadatas:
                        if struct.name == attribute.data_type():
                            if struct.has_undefined_nested_dependencies():
                                converged = False
                                continue
                            new_attributes.append(
                                AttributeMetadata(
                                    original_name=attribute.original_name,
                                    name=attribute.name,
                                    data_type=struct,
                                    optional=attribute.optional,
                                )
                            )
                            changed = True
                            break
                else:
                    new_attributes.append(attribute)
            if converged:
                new_struct.attributes = new_attributes
            updated_struct_metadatas.append(new_struct)
        new_struct_metadatas = updated_struct_metadatas

    return new_struct_metadatas


def write_web_common_nested_structs(path: str, nested_structs: List[StructMetadata]):
    """Writes the nested structs to the web_common crate."""

    # We open the file to write the nested structs
    tables = open(f"../web_common/src/database/{path}", "w", encoding="utf8")

    # Preliminarly, we write a docstring at the very head
    # of this submodule to explain what it does and warn the
    # reader not to write anything in this file as it is
    # automatically generated.
    tables.write(
        "//! This module contains the nested structs for the database tables.\n"
    )
    tables.write("//!\n")
    tables.write(
        "//! This file is automatically generated. Do not write anything here.\n"
    )
    tables.write("\n")

    # We start with the necessary imports.
    imports = [
        "use serde::Deserialize;",
        "use serde::Serialize;",
        "use super::tables::*;",
        "use super::views::*;",
    ]

    for import_statement in imports:
        tables.write(f"{import_statement}\n")

    for struct_metadata in nested_structs:
        if not struct_metadata.can_implement_clone():
            print(f"# {struct_metadata.name}")
            for attribute in struct_metadata.attributes:
                print(f"* {attribute.name} {attribute.implements_clone()}")

        tables.write("#[derive(" + ", ".join(struct_metadata.derives()) + ")]\n")
        tables.write(f"pub struct {struct_metadata.name} {{\n")
        for attribute in struct_metadata.attributes:
            tables.write(f"    pub {attribute.name}: {attribute.format_data_type()},\n")
        tables.write("}\n")

    tables.close()


def write_table_names_enumeration(struct_metadatas: List[StructMetadata]):
    imports = [
        "use serde::Deserialize;",
        "use serde::Serialize;",
    ]

    # The derives to apply to the structs in the `src/database/tables.rs` document
    derives = ["Deserialize", "Serialize", "Clone", "Debug", "PartialEq", "Eq", "Copy"]

    # We check that we are currently executing in the `backend` crate
    # so to make sure that the relative path to the `web_common` crate
    # is correct.
    if not os.getcwd().endswith("backend"):
        raise Exception("This script must be executed in the `backend` crate.")

    tables = open(f"../web_common/src/database/table_names.rs", "w", encoding="utf8")

    # Preliminarly, we write a docstring at the very head
    # of this submodule to explain what it does and warn the
    # reader not to write anything in this file as it is
    # automatically generated.

    tables.write(
        "//! This module contains the table names enumeration.\n"
        "//!\n"
        "//! This module is automatically generated. Do not write anything here.\n\n"
    )

    for import_statement in imports:
        tables.write(f"{import_statement}\n")

    unique_table_names = {
        (struct_metadata.table_name, struct_metadata.capitalized_table_name())
        for struct_metadata in struct_metadatas
    }

    unique_table_names = list(unique_table_names)

    sorted(unique_table_names, key=lambda x: x[0])

    tables.write("#[derive(" + ", ".join(derives) + ")]\n")
    tables.write("pub enum Table {\n")
    for _, table_name in unique_table_names:
        tables.write(f"    {table_name},\n")
    tables.write("}\n\n")

    # We implement the `AsRef` trait for the `Table` enum
    # to convert it into &str.
    tables.write("impl AsRef<str> for Table {\n")
    tables.write("    fn as_ref(&self) -> &str {\n")
    tables.write("        match self {\n")
    for table_name, capitalized_table_name in unique_table_names:
        tables.write(
            f'            Table::{capitalized_table_name} => "{table_name}",\n'
        )
    tables.write("        }\n")
    tables.write("    }\n")
    tables.write("}\n")

    # We implement display

    tables.write("impl std::fmt::Display for Table {\n")
    tables.write(
        "    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n"
    )
    tables.write('        write!(f, "{}", self.as_ref())\n')
    tables.write("    }\n")
    tables.write("}\n")

    tables.close()


def write_web_common_search_trait_implementations(
    struct_metadatas: List[StructMetadata],
):
    # We check that we are currently executing in the `backend` crate
    # so to make sure that the relative path to the `web_common` crate
    # is correct.
    if not os.getcwd().endswith("backend"):
        raise Exception("This script must be executed in the `backend` crate.")

    tables = open(f"../web_common/src/database/search_tables.rs", "w", encoding="utf8")
    similarity_indices: PGIndices = find_pg_trgm_indices()

    imports = [
        "use crate::database::*;",
    ]

    # Preliminarly, we write a docstring at the very head
    # of this submodule to explain what it does and warn the
    # reader not to write anything in this file as it is
    # automatically generated.

    tables.write(
        "//! This module contains the table names enumeration.\n"
        "//!\n"
        "//! This module is automatically generated. Do not write anything here.\n\n"
    )

    for import_statement in imports:
        tables.write(f"{import_statement}\n")

    # First, we create the Searchable trait that will be implemented by all the structs
    # that are searchable.

    tables.write("pub trait Searchable {\n")
    tables.write("    fn search_task(query: String, limit: u32) -> super::Select;\n")
    tables.write("}\n")

    for struct in struct_metadatas:
        if similarity_indices.has_table(struct.table_name):
            tables.write(f"impl Searchable for {struct.name} {{\n")
            tables.write(
                "    fn search_task(query: String, limit: u32) -> super::Select {\n"
            )
            tables.write(f"        super::Select::search(\n")
            tables.write(f"             Table::{struct.capitalized_table_name()},\n")
            tables.write("              query,\n")
            tables.write("              limit,\n")
            tables.write("        )\n")
            tables.write("    }\n")
            tables.write("}\n")

    tables.close()


def main():
    # Read the replacements from the JSON file
    replacements = compress_json.local_load("replacements.json")

    # We make sure the migrations were fully executed
    status = os.system("diesel migration run")

    if status != 0:
        raise Exception("The migrations were not fully executed.")

    # We run the diesel extended CLI command
    status = os.system("diesel_ext --model --add-table-name > src/models.rs")

    if status != 0:
        raise Exception("The diesel_ext command failed.")

    # Read the generated file
    with open("src/models.rs", "r", encoding="utf8") as file:
        content = file.read()

    # Imports to always add to the file
    imports = [
        "use diesel::Queryable;",
        "use diesel::QueryableByName;",
        "use diesel::Identifiable;",
        "use diesel::Insertable;",
        "use crate::schema::*;",
        "use diesel::Selectable;",
        "use serde::Deserialize;",
        "use serde::Serialize;",
        "use diesel::r2d2::ConnectionManager;",
        "use diesel::r2d2::PooledConnection;",
        "use diesel::prelude::*;",
    ]

    for import_statement in imports:
        if import_statement not in content:
            content = content.replace(
                "#![allow(clippy::all)]", f"#![allow(clippy::all)]\n{import_statement}"
            )

    # We need to add some extra derives to the structs
    derives = ["Selectable", "Clone", "PartialEq"]

    for derive in derives:
        content = content.replace("#[derive(", f"#[derive({derive}, ")

    # Apply the replacements
    for replacement in replacements:
        if replacement["search"] in content:
            content = content.replace(replacement["search"], replacement["replace"])
            for import_statement in replacement["imports"]:
                if import_statement not in content:

                    # The import statements must be added on the top of the file,
                    # but below the eventual macros and attributes. We can refer
                    # to the following macros as an anchor point, which are characterized
                    # by the `#![...]` syntax.
                    #
                    # ```rust
                    # #![allow(unused)]
                    # #![allow(clippy::all)]
                    # ```
                    #
                    # We then insert the import statements right after the anchor point.
                    content = content.replace(
                        "#![allow(clippy::all)]",
                        f"#![allow(clippy::all)]\n{import_statement}",
                    )

    complex_derives = [
        "Serialize",
        "Deserialize",
        "Eq",
        "Insertable",
        "QueryableByName",
    ]

    deny_list = ["Interval", "Range<Numeric>", "Money"]

    unequables = [
        "f32",
        "f64",
    ]

    # Some derives are more complex, and we only add them if within
    # the struct we are currently processing there is NOT in the deny list.
    # A struct is defined from where the `struct` keyword is found until the
    # next `}` is found.
    new_content = ""
    currently_in_struct = False

    for line_number, line in enumerate(content.split("\n")):
        if "#[derive(" in line:
            currently_in_struct = True
        if currently_in_struct:
            # We execute a look ahead to see if we find any of the
            # types in the deny list in the next lines up until
            # we find the closing bracket of the struct.
            found_deny = False
            found_unequable = False

            for look_ahead_line in content.split("\n")[line_number:]:
                if "}" in look_ahead_line:
                    currently_in_struct = False
                    break
                for deny in deny_list:
                    if deny in look_ahead_line:
                        found_deny = True
                        break
                for unequable in unequables:
                    if unequable in look_ahead_line:
                        found_unequable = True
                        break
            if not found_deny:
                for derive in complex_derives:
                    if derive == "Eq" and found_unequable:
                        continue
                    if f", {derive}," not in line and f"({derive}," not in line:
                        line = line.replace("#[derive(", f"#[derive({derive}, ")

        new_content += line + "\n"
    content = new_content

    # Write the file back
    with open("src/models.rs", "w", encoding="utf8") as file:
        file.write(content)


if __name__ == "__main__":
    # Load dotenv file
    load_dotenv()

    # Due to git collisions, it may happen that a migration has been renamed
    # but the old directory is still there. For each directory in the migrations
    # that is composed of the [0-9]+_[a-z_]+ pattern, we check if the directory
    # as a twin with a different code but the same name. If so, we remove the
    # empty version of these directories.
    for directory in os.listdir("migrations"):
        if not os.path.isdir(f"migrations/{directory}"):
            continue
        if re.match(r"^[0-9]+_[a-z_]+$", directory):
            # We check whether the current directory DOES NOT
            # contain either a down.sql or an up.sql file.
            if not os.path.exists(
                f"migrations/{directory}/down.sql"
            ) or not os.path.exists(f"migrations/{directory}/up.sql"):
                code, migration_name = directory.split("_", maxsplit=1)
                twin_found = False
                for directory2 in os.listdir("migrations"):
                    if not os.path.isdir(f"migrations/{directory2}"):
                        continue
                    if re.match(r"^[0-9]+_[a-z_]+$", directory2):
                        code2, migration_name2 = directory2.split("_", maxsplit=1)
                        if code != code2 and migration_name == migration_name2:
                            print(f"Removing {directory}")
                            shutil.rmtree(f"migrations/{directory}")
                            twin_found = True
                            break
                if not twin_found:
                    raise Exception(
                        f"Directory {directory} does not contain either a `down.sql` or an `up.sql` file "
                        "and there is no twin directory with a different code."
                    )

    densify_directory_counter()

    # We make sure that the ./db_data/taxons.tsv file is present
    # or otherwise we run the script to generate it.
    if not os.path.exists("./db_data/taxons.tsv"):
        retrieve_ncbi_taxon()

    # If there is a "__pycache__" directory, we remove it as Diesel
    # seems to be keen to try and run it as a migration, and it will
    # fail.
    if os.path.exists("__pycache__"):
        shutil.rmtree("__pycache__")

    if os.path.exists("migrations/__pycache__"):
        shutil.rmtree("migrations/__pycache__")

    main()
    print("Generated models.")
    generate_view_schema()
    print("Generated view schema.")
    check_schema_completion()
    print("Checked schema completion.")
    generate_view_structs()
    print("Generated view structs.")
    table_structs: List[StructMetadata] = write_web_common_structs(
        "src/models.rs", "tables", "Table"
    )
    view_structs: List[StructMetadata] = write_web_common_structs(
        "src/views/views.rs", "views", "View"
    )
    print("Generated web common structs.")
    write_from_impls("src/models.rs", "tables", table_structs)
    write_from_impls("src/views/views.rs", "views", view_structs)
    print("Generated From implementations for backend.")

    write_table_names_enumeration(table_structs + view_structs)
    print("Generated table names enumeration for web_common.")

    nested_structs: List[StructMetadata] = generate_nested_structs(
        "src/nested_models.rs", table_structs + view_structs
    )
    print("Generated nested structs for backend.")
    write_web_common_nested_structs("nested_models.rs", nested_structs)
    print("Generated nested structs for web_common.")

    write_web_common_search_trait_implementations(
        nested_structs + table_structs + view_structs
    )
    print("Generated search trait implementations for web_common.")
