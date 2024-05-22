"""Submodule providing functions to regroup tables.

Implementative details
----------------------
When adding new migrations to the diesel migrations, expecially when multiple
users are adding migrations at once, the migrations may start to result sparse
and harder to logically follow. This method ensures that the migrations are regrouped
in such a way that all the migrations relative to a table are grouped together.
"""

import os
from typing import Dict, List

WHITE_LISTED_MIGRATIONS = [
    "00000000000000_diesel_initial_setup",
    "00000000000001_enable_uuid_extension",
    "00000000000002_enable_pg_trgm_extension",
]


def get_best_insertion_point(table_name: str, expected_desinence: str) -> int:
    """Get the best insertion point for a new migration related to a table.

    Parameters
    ----------
    table_name : str
        The name of the table for which we want to find the best insertion point.

    Returns
    -------
    int
        The best insertion point for a new migration related to the table.

    Raises
    ------
    Exception
        If the migration that created the table cannot be found.
    """

    assert (
        table_name in expected_desinence
    ), f"Table name {table_name} is not in the expected desinence {expected_desinence}"

    valid_desinences = get_desinences(table_name)

    assert (
        expected_desinence in valid_desinences
    ), f"Desinence {expected_desinence} is not in the desinences of the table {table_name}"

    index_in_desinences = valid_desinences.index(expected_desinence)

    # First, we identify the position of the migration that has created the current
    # table by finding the one with desinence `_create_{table_name}_table`.
    migrations = [
        directory
        for directory in os.listdir("migrations")
        if os.path.isdir(f"migrations/{directory}")
        and os.path.exists(f"migrations/{directory}/up.sql")
    ]

    migration_number = None

    for migration in migrations:
        number, desinence = migration.split("_", maxsplit=1)
        if desinence == expected_desinence:
            raise RuntimeError(f"Migration {migration} already exists.")
        if (
            desinence in valid_desinences
            and valid_desinences.index(desinence) < index_in_desinences
        ):
            migration_number = number

    if migration_number is None:
        raise RuntimeError(
            f"Could not find the migration that created the {table_name} table."
        )

    return int(migration_number) + 1


def get_desinences(table_name: str) -> List[str]:
    """Get the possible desinences of a table."""
    return [
        f"create_{table_name}_table",
        f"create_{table_name}_sequential_index",
        f"create_{table_name}_updated_at_trigger",
        f"create_{table_name}_parent_circularity_trigger",
        f"populate_{table_name}_table",
        f"create_{table_name}_gin_index",
        f"create_{table_name}_btree_index",
    ]


def get_tables() -> List[str]:
    """Extracts the table names from the migrations.

    Implementative details
    ----------------------
    The table names are extracted from the migrations by searching for the
    string "CREATE TABLE IF NOT EXISTS {table_name} (" in the up migrations.
    """
    tables = []

    target = "CREATE TABLE IF NOT EXISTS"

    migrations = os.listdir("migrations")
    migrations = sorted(migrations)

    for migration in migrations:
        if not os.path.isdir(f"migrations/{migration}"):
            continue
        with open(f"migrations/{migration}/up.sql", "r", encoding="utf-8") as f:
            lines = f.readlines()
            for line in lines:
                if target in line:
                    table_name = line.split(target)[1].split("(")[0].strip()
                    tables.append(table_name)
    return tables


def table_dependencies() -> Dict[str, List[str]]:
    """Return the table dependencies."""

    tables = get_tables()
    dependencies = {table: [] for table in tables}

    for migration in os.listdir("migrations"):
        if not os.path.isdir(f"migrations/{migration}"):
            continue

        if migration in WHITE_LISTED_MIGRATIONS:
            continue

        _, desinence = migration.split("_", 1)
        current_table = None

        # We identify the table associated to this migration.
        for table in tables:
            desinences = get_desinences(table)
            if desinence in desinences:
                current_table = table
                break

        assert (
            current_table is not None
        ), f"Could not find the table associated to the migration {migration}"

        # We identify the tables that are being referenced in the migration.
        with open(f"migrations/{migration}/up.sql", "r", encoding="utf-8") as f:
            for line in f.readlines():
                if line.startswith("--"):
                    continue
                for table in tables:
                    needles = (f" {table}(", f" {table} (")
                    if (
                        any(needle in line for needle in needles)
                        and table != current_table
                    ):
                        if table not in dependencies[current_table]:
                            dependencies[current_table].append(table)

    return dependencies


def get_sort_tables_by_dependencies() -> List[str]:
    """Returns list of tables sorted by dependencies.

    Implementative details
    ----------------------
    The tables are sorted by dependencies using a topological sort algorithm.
    """
    dependencies = table_dependencies()
    tables = list(dependencies.keys())
    sorted_tables = []

    while len(tables) > 0:
        for table in tables:
            if len(dependencies[table]) == 0:
                sorted_tables.append(table)
                tables.remove(table)
                for other_table in tables:
                    if table in dependencies[other_table]:
                        dependencies[other_table].remove(table)
                break
        else:
            raise RuntimeError(f"Circular dependency detected in the tables {tables}")

    return sorted_tables


def regroup_tables():
    """Regroup the tables."""

    associated_tables = {}
    orphan_migrations = []
    mapped_migrations = []

    table_names = get_sort_tables_by_dependencies()

    for table_name in table_names:
        desinences = get_desinences(table_name)
        associated_tables[table_name] = []
        for migration in os.listdir("migrations"):
            if not os.path.isdir(f"migrations/{migration}"):
                continue

            if migration in WHITE_LISTED_MIGRATIONS:
                continue

            if migration in mapped_migrations:
                continue

            if table_name not in migration:
                continue

            _number, desinence = migration.split("_", 1)

            if desinence in desinences:
                associated_tables[table_name].append(migration)
                mapped_migrations.append(migration)
                if migration in orphan_migrations:
                    orphan_migrations.remove(migration)
                continue

            if migration not in orphan_migrations:
                orphan_migrations.append(migration)

    if len(orphan_migrations) > 0:
        raise RuntimeError(f"Orphaned migrations found {orphan_migrations}")

    starting_number = len(WHITE_LISTED_MIGRATIONS)

    for table_name in table_names:
        migrations = associated_tables[table_name]
        desinences = get_desinences(table_name)

        # We sort the migrations by the index of the desinence in the desinences list.
        migrations = sorted(
            migrations, key=lambda x: desinences.index(x.split("_", 1)[1])
        )

        for i, migration in enumerate(migrations):
            _number, desinence = migration.split("_", 1)
            padded_migration_number = str(i + starting_number).zfill(14)
            full_migration_name = f"{padded_migration_number}_{desinence}"
            if migration != full_migration_name:
                os.rename(
                    f"migrations/{migration}", f"migrations/{full_migration_name}"
                )

        starting_number += len(migrations)
