"""Submodule to write the SQL function bindings for the Diesel ORM as loaded from the database."""
from typing import List
from constraint_checkers.find_foreign_keys import TableMetadata, SQLFunction

def write_diesel_sql_function_bindings(table_metadata: TableMetadata):
    """Write the SQL function bindings for the Diesel ORM as loaded from the database."""
    sql_functions: List[SQLFunction] = table_metadata.get_all_postgres_functions()
    
    with open('src/sql_function_bindings.rs', 'w', encoding="utf8") as f:
        # First thing, we write that this file is automatically generated
        # and discourage the user from editing it manually.
        f.write(
            "//! This file is automatically generated by the code generation suite.\n"
            "//! Do not edit it manually.\n"
            "//!\n"
            "//! This file contains the bindings for the SQL functions in the database.\n"
            "//!\n"
            "//! # Implementative details\n"
            "//! All postgres function receive parameter that are potentially options, and there\n"
            "//! is no way to explicitly write a function that receives a non-nullable parameter.\n"
            "//! Therefore, all parameters are wrapped in an Option, and the check of whether the\n"
            "//! parameter is null or not is done in the postgres function itself.\n\n"
        )

        for sql_function in sql_functions:
            sql_function.write_diesel_binding_to_file(f)

    print(f"Successfully wrote {len(sql_functions)} SQL function bindings to src/sql_function_bindings.rs.")