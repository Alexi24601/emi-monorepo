"""Submodule containing the enforcement of the migration naming convention."""
import os



def enforce_migration_naming_convention():
    """Check that the migrations are named according to the convention."""

    # We check that if a migration directory contains a population of a given table,
    # we verify that if there is also another migration that creates a search index
    # as indicated by the suffix `_index`, the migration that populates the table must
    # have a lower number than the migration that creates the search index.
    migrations = [
        directory
        for directory in os.listdir("migrations")
        if os.path.isdir(f"migrations/{directory}")
        and os.path.exists(f"migrations/{directory}/up.sql")
    ]

    for directory in os.listdir("migrations"):
        if not os.path.isdir(f"migrations/{directory}"):
            continue

        if not os.path.exists(f"migrations/{directory}/up.sql") or not os.path.exists(
            f"migrations/{directory}/down.sql"
        ):
            continue

        if directory == "00000000000000_diesel_initial_setup":
            continue

        with open(f"migrations/{directory}/up.sql", "r", encoding="utf8") as up_file:
            up_content = up_file.read()

        expected_name = directory

        if "CREATE TEMP TABLE" in up_content:
            raise RuntimeError(
                f"Migration {directory} contains a `CREATE TEMP TABLE` constraint. Please standardize the naming convention "
                "and replace it with `CREATE TEMPORARY TABLE`."
            )

        if "CREATE TABLE IF NOT EXISTS" in up_content:
            # This document contains a table creation, and as such
            # we expect its name to conform to {number_of_migration}_create_{table_name}_table
            table_name = (
                up_content.split("CREATE TABLE IF NOT EXISTS")[1].split("(")[0].strip()
            )
            number_of_migration = directory.split("_")[0]

            expected_name = f"{number_of_migration}_create_{table_name}_table"

        if directory != expected_name:
            raise RuntimeError(
                f"Migration {directory} does not conform to the naming convention. "
                f"Expected name: {expected_name}."
            )