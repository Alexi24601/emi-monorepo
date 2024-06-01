"""This module contains the function to write the nested structs to the web_common crate."""
from typing import List
from constraint_checkers.struct_metadata import StructMetadata
from constraint_checkers.rust_implementation_check import trait_implementation_exist

def write_web_common_nested_structs(path: str, nested_structs: List[StructMetadata]):
    """Writes the nested structs to the web_common crate."""

    # We open the file to write the nested structs
    document = open(f"../web_common/src/database/{path}", "w", encoding="utf8")

    # Preliminarly, we write a docstring at the very head
    # of this submodule to explain what it does and warn the
    # reader not to write anything in this file as it is
    # automatically generated.
    document.write(
        "//! This module contains the nested structs for the database tables.\n"
        "//!\n"
        "//! This file is automatically generated. Do not write anything here.\n\n"
    )

    # We start with the necessary imports.
    imports = [
        "use serde::Deserialize;",
        "use serde::Serialize;",
        "use super::*;",
    ]

    document.write("\n".join(imports) + "\n\n")

    for nested_struct in nested_structs:
        nested_struct.write_to(document)

        document.write(
            f"impl Tabular for {nested_struct.name} {{\n"
            f"    const TABLE: Table = Table::{nested_struct.capitalized_table_name()};\n"
            "}\n"
        )

        if nested_struct.has_filter_variant():
            filter_variant = nested_struct.get_filter_variant()

            document.write(
                f"impl Filtrable for {nested_struct.name} {{\n"
                f"    type Filter = {filter_variant.name};\n"
                "}\n"
            )

        if not trait_implementation_exist(
            "Describable",
            nested_struct.name,
            deny_file_list=(f"database/{path}.rs",),
            root="webcommon",
        ):
            inner = nested_struct.get_inner_attribute()

            document.write(
                f"impl Describable for {nested_struct.name} {{\n"
                "    fn description(&self) -> Option<&str> {\n"
            )
            if inner is not None:
                document.write(
                    "        self.inner.description()\n"
                )
            else:
                document.write("        None\n")
            document.write("    }\n}\n")

        if not trait_implementation_exist(
            "Colorable",
            nested_struct.name,
            deny_file_list=(f"database/{path}.rs",),
            root="webcommon",
        ):
            color_attribute = nested_struct.get_color_attribute()

            document.write(
                f"impl Colorable for {nested_struct.name} {{\n"
                "    fn color(&self) -> Option<&str> {\n"
            )
            if color_attribute:
                if color_attribute.optional:
                    document.write(f"        self.{color_attribute.name}.as_ref().map(|color| color.name.as_str())\n")
                else:
                    document.write(f"        Some(self.{color_attribute.name}.name.as_str())\n")
            else:
                document.write("        None\n")
            document.write("    }\n}\n")

        # We implement the `get` method for the struct when the frontend feature
        # is enabled using GlueSQL. This method will be extremely similar to the
        # `get` method for the Diesel-based approach of the backend.

        flat_variant = nested_struct.get_flat_variant()

        document.write(f'#[cfg(feature = "frontend")]\nimpl {nested_struct.name} {{\n')

        # First, we implement the `from_flat` method that will be used to convert
        # the flat struct to the nested struct. This method receives the flat struct
        # and the connection to the database and returns the nested struct.
        document.write(
            "    /// Convert the flat struct to the nested struct.\n"
            "    ///\n"
            "    /// # Arguments\n"
            "    /// * `flat_variant` - The flat struct.\n"
            "    /// * `connection` - The database connection.\n"
            "    pub async fn from_flat(\n"
            f"        flat_variant: {flat_variant.name},\n"
            "        connection: &mut gluesql::prelude::Glue<impl gluesql::core::store::GStore + gluesql::core::store::GStoreMut>,\n"
            "    ) -> Result<Self, gluesql::prelude::Error> {\n"
            "        Ok(Self {\n"
        )
        for attribute in nested_struct.attributes:
            if attribute.name == "inner":
                continue
            if (
                attribute.data_type() == nested_struct.name
                or flat_variant.has_attribute(attribute)
            ):
                document.write(
                    f"            {attribute.name}: flat_variant.{attribute.name},\n"
                )
                continue
            if attribute.optional:
                document.write(
                    f"            {attribute.name}: if let Some({attribute.original_name}) = flat_variant.{attribute.original_name} {{ {attribute.data_type()}::get({attribute.original_name}, connection).await? }} else {{ None }},\n"
                )
            else:
                document.write(
                    f"            {attribute.name}: {attribute.data_type()}::get(flat_variant.{attribute.original_name}, connection).await?.unwrap(),\n"
                )

        if any(attribute.name == "inner" for attribute in nested_struct.attributes):
            document.write("            inner: flat_variant,\n")

        document.write("        })\n    }\n")

        document.write(
            "    /// Get the nested struct from the provided primary key.\n"
            "    ///\n"
            "    /// # Arguments\n"
            f"    /// * `{flat_variant.get_formatted_primary_keys(include_prefix=False)}` - The primary key(s) of the row.\n"
            "    /// * `connection` - The database connection.\n"
            "    pub async fn get<C>(\n"
            f"        {flat_variant.get_formatted_primary_keys(include_prefix=False)}: {flat_variant.get_formatted_primary_key_data_types()},\n"
            "        connection: &mut gluesql::prelude::Glue<C>,\n"
            "    ) -> Result<Option<Self>, gluesql::prelude::Error> where\n"
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
            "    {\n"
            f"       let flat_variant = {flat_variant.name}::get({flat_variant.get_formatted_primary_keys(include_prefix=False)}, connection).await?;"
            "        match flat_variant {\n"
            "            Some(flat_variant) => Ok(Some(Self::from_flat(flat_variant, connection).await?)),\n"
            "            None => Ok(None),\n"
            "        }\n"
            "    }\n"
        )

        # We implement the all method for the struct when the frontend feature is enabled
        # using GlueSQL. This method will be extremely similar to the `all` method for the
        # Diesel-based approach of the backend.

        document.write(
            "    /// Get all the nested structs from the database.\n"
            "    ///\n"
            "    /// # Arguments\n"
        )
        if nested_struct.has_filter_variant():
            document.write("    /// * `filter` - The filter to apply to the results.\n")
        document.write(
            "    /// * `limit` - The maximum number of rows to return.\n"
            "    /// * `offset` - The number of rows to skip.\n"
            "    /// * `connection` - The database connection.\n"
            "    pub async fn all<C>(\n"
        )
        if nested_struct.has_filter_variant():
            filter_variant = nested_struct.get_filter_variant()
            document.write(f"        filter: Option<&{filter_variant.name}>,\n")
        document.write(
            "        limit: Option<i64>,\n"
            "        offset: Option<i64>,\n"
            "        connection: &mut gluesql::prelude::Glue<C>,\n"
            "    ) -> Result<Vec<Self>, gluesql::prelude::Error> where\n"
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
            "    {\n"
        )
        if nested_struct.has_filter_variant():
            document.write(
                f"        let flat_variants = {flat_variant.name}::all(filter, limit, offset, connection).await?;\n"
            )
        else:
            document.write(
                f"        let flat_variants = {flat_variant.name}::all(None, limit, offset, connection).await?;\n"
            )
        document.write(
            "         let mut nested_structs = Vec::with_capacity(flat_variants.len());\n"
            "         for flat_variant in flat_variants {\n"
            "             nested_structs.push(Self::from_flat(flat_variant, connection).await?);\n"
            "         }\n"
            "         Ok(nested_structs)\n"
            "    }\n"
        )

        # We implement the all_by_updated_at method for the struct when the frontend feature
        # is enabled using GlueSQL. This method will be extremely similar to the `all_by_updated_at`
        # method for the Diesel-based approach of the backend.

        if flat_variant.table_metadata.has_updated_at_column(flat_variant.table_name):
            document.write(
                "    /// Get all the nested structs from the database ordered by the `updated_at` column.\n"
                "    ///\n"
                "    /// # Arguments\n"
            )
            if nested_struct.has_filter_variant():
                document.write(
                    "    /// * `filter` - The filter to apply to the results.\n"
                )
            document.write(
                "    /// * `limit` - The maximum number of rows to return.\n"
                "    /// * `offset` - The number of rows to skip.\n"
                "    /// * `connection` - The database connection.\n"
                "    pub async fn all_by_updated_at<C>(\n"
            )
            if nested_struct.has_filter_variant():
                filter_variant = nested_struct.get_filter_variant()
                document.write(f"        filter: Option<&{filter_variant.name}>,\n")
            document.write(
                "        limit: Option<i64>,\n"
                "        offset: Option<i64>,\n"
                "        connection: &mut gluesql::prelude::Glue<C>,\n"
                "    ) -> Result<Vec<Self>, gluesql::prelude::Error> where\n"
                "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
                "    {\n"
            )
            if nested_struct.has_filter_variant():
                document.write(
                    f"        let flat_variants = {flat_variant.name}::all_by_updated_at(filter, limit, offset, connection).await?;\n"
                )
            else:
                document.write(
                    f"        let flat_variants = {flat_variant.name}::all_by_updated_at(None, limit, offset, connection).await?;\n"
                )
            document.write(
                "         let mut nested_structs = Vec::with_capacity(flat_variants.len());\n"
                "         for flat_variant in flat_variants {\n"
                "             nested_structs.push(Self::from_flat(flat_variant, connection).await?);\n"
                "         }\n"
                "         Ok(nested_structs)\n"
                "    }\n"
            )

        # We implement the update_or_insert method for the struct when the frontend feature
        # is enabled using GlueSQL. This method will be extremely similar to the `update_or_insert`
        # for the flat version of the struct, with the important difference that we will call it
        # on all of its attributes that are nested structs.

        document.write(
            "    /// Update or insert the nested struct into the database.\n"
            "    ///\n"
            "    /// # Arguments\n"
            "    /// * `connection` - The database connection.\n"
            "    pub async fn update_or_insert<C>(\n"
            "        self,\n"
            "        connection: &mut gluesql::prelude::Glue<C>,\n"
            "    ) -> Result<(), gluesql::prelude::Error> where\n"
            "        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
            "    {\n"
        )
        for attribute in nested_struct.attributes:
            assert attribute.has_struct_data_type()

            if attribute.optional:
                document.write(
                    f"        if let Some({attribute.name}) = self.{attribute.name} {{\n"
                    f"            {attribute.name}.update_or_insert(connection).await?;\n"
                    "        }\n"
                )
            else:
                document.write(
                    f"        self.{attribute.name}.update_or_insert(connection).await?;\n"
                )

        document.write("        Ok(())\n    }\n}\n")

    document.flush()
    document.close()