"""Write the nested structs to the backend crate."""

from typing import List
import os
from tqdm.auto import tqdm
from constraint_checkers.struct_metadata import StructMetadata


def write_backend_nested_structs(nested_structs: List[StructMetadata]):
    """Write the nested structs to the backend crate."""
    assert isinstance(nested_structs, list), "The nested structs must be a list."
    assert all(
        isinstance(nested_struct, StructMetadata) for nested_struct in nested_structs
    ), "All the nested structs must be of type StructMetadata."
    assert all(
        nested_struct.is_nested() for nested_struct in nested_structs
    ), "All the nested structs must be nested. "
    assert len(nested_structs) > 0, "No nested structs to write."

    # We open the file to write the nested structs
    module_document = open("./src/nested_models.rs", "w", encoding="utf8")

    # Preliminarly, we write a docstring at the very head
    # of this submodule to explain what it does and warn the
    # reader not to write anything in this file as it is
    # automatically generated.

    warning = (
        "//! This module contains the nested structs for the database tables.\n"
        "//!\n"
        "//! This file is automatically generated. Do not write anything here.\n\n"
    )
    module_document.write(warning)

    # We start with the necessary imports.
    imports = [
        "use serde::Deserialize;",
        "use serde::Serialize;",
        "use crate::models::*;",
        "use std::rc::Rc;",
        "use web_common::database::filter_structs::*;",
        # "use crate::views::views::*;",
    ]

    os.makedirs("./src/nested_models", exist_ok=True)

    for nested_struct in tqdm(
        nested_structs,
        desc="Writing nested structs",
        unit="nested struct",
        leave=False,
    ):
        document = open(f"./src/nested_models/{nested_struct.table_name}.rs", "w", encoding="utf8")
        module_document.write(
            f"mod {nested_struct.table_name};\n"
            f"pub use {nested_struct.table_name}::*;\n"
        )

        default_imports = list(imports)

        if nested_struct.contains_nested_structs():
            default_imports.append("use super::*;")

        document.write("\n".join(default_imports) + "\n\n")
        nested_struct.write_to(document)
        flat_variant = nested_struct.get_flat_variant()

        # We implement the all for the nested structs

        # First, we implement a method that will be reused by several of the following methods,
        # including the all, get and search ones: a method that given the flat struct and a connection
        # to the database returns a result containing the nested struct.
        document.write(
            f"impl {nested_struct.name} {{\n"
            "    /// Convert the flat struct to the nested struct.\n"
            "    ///\n"
            "    /// # Arguments\n"
            "    /// * `flat_variant` - The flat struct.\n"
        )

        contains_struct_that_may_be_hidden = nested_struct.has_attribute_that_may_be_hidden()
        
        if contains_struct_that_may_be_hidden:
            document.write(
            "    /// * `author_user_id` - The author user id.\n"
            )
        document.write(
            "    /// * `connection` - The database connection.\n"
            "    pub fn from_flat(\n"
            f"        flat_variant: {flat_variant.name},\n"
        )

        if contains_struct_that_may_be_hidden:
            document.write(
                "        author_user_id: Option<i32>,\n"
            )
        document.write(
            "        connection: &mut diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::prelude::PgConnection>>,\n"
            "    ) -> Result<Self, web_common::api::ApiError> {\n"
            "        Ok(Self {\n"
        )
        for attribute in nested_struct.attributes:
            assert (
                not attribute.data_type() == nested_struct.name
            ), "The data type of the attribute cannot be the same as the nested struct."
            if attribute.is_inner():
                continue
            if flat_variant.has_attribute(attribute):
                document.write(
                    f"            {attribute.name}: flat_variant.{attribute.name},\n"
                )
                continue

            assert attribute.has_struct_data_type()
            if attribute.raw_data_type().may_be_hidden():
                author_user_id_argument = "author_user_id, "
            else:
                author_user_id_argument = ""

            if attribute.optional:
                document.write(
                    f"            {attribute.name}: flat_variant.{attribute.original_name}.map(|{attribute.original_name}| {attribute.data_type()}::get({attribute.original_name}, {author_user_id_argument}connection)).transpose()?,\n"
                )
            else:
                document.write(
                    f"            {attribute.name}: {attribute.data_type()}::get(flat_variant.{attribute.original_name}, {author_user_id_argument}connection)?,\n"
                )

        document.write("                inner: flat_variant,\n        })\n    }\n")

        for method in flat_variant.backend_methods():
            return_type = method.get_return_type()
            nested_struct.add_backend_method(method.into_new_owner(nested_struct))
            author_user_id = method.get_argument_by_name("author_user_id")
            this_author_user_id_argument = ""

            if contains_struct_that_may_be_hidden:
                assert author_user_id is not None, (
                    f" In the struct {nested_struct.name}, the author_user_id argument must be present in all methods or in none of them. "
                    f"The method {method.name} does not have the author_user_id argument. "
                )

            if author_user_id is None:
                assert not contains_struct_that_may_be_hidden
                assert author_user_id_argument == ""
            elif contains_struct_that_may_be_hidden:
                if author_user_id.optional:
                    this_author_user_id_argument = f"{author_user_id.name}, "
                else:
                    this_author_user_id_argument = f"Some({author_user_id.name}), "

            if return_type.format_data_type() == "Result<Vec<Self>, web_common::api::ApiError>":
                method.write_header_to(document)
                document.write(
                    "{\n"
                    f"        {flat_variant.name}::{method.name}({', '.join(arg.name for arg in method.arguments)})?.into_iter().map(|flat_variant| Self::from_flat(flat_variant, {this_author_user_id_argument}connection)).collect()\n"
                    "}\n"
                )
            elif return_type.format_data_type() == "Result<Self, web_common::api::ApiError>":
                method.write_header_to(document)
                document.write(
                    "{\n"
                    f"        {flat_variant.name}::{method.name}({', '.join(arg.name for arg in method.arguments)}).and_then(|flat_variant| Self::from_flat(flat_variant, {this_author_user_id_argument}connection))\n"
                    "}\n"
                )
            elif return_type.format_data_type() == "Result<Vec<(Self, f32)>, web_common::api::ApiError>":
                method.write_header_to(document)
                document.write(
                    "{\n"
                    f"        {flat_variant.name}::{method.name}({', '.join(arg.name for arg in method.arguments)})?.into_iter().map(|(flat_variant, score)| Ok((Self::from_flat(flat_variant, {this_author_user_id_argument}connection)?, score))).collect()\n"
                    "}\n"
                )
            elif "Self" in return_type.format_data_type():
                raise NotImplementedError(
                    "All cases returning a Self must be handled. "
                    f"The method {method.name} returns {return_type.format_data_type()}. "
                    f"The method {method.name} is in the struct {nested_struct.name}."
                )
            elif method.has_self_reference():
                assert any(attr.is_inner() for attr in nested_struct.attributes), (
                    "The struct must have at least one inner attribute, which is the flat struct. "
                    f"The struct {nested_struct.name} has the following attributes: {nested_struct.attributes}."
                )
                method.write_header_to(document)
                document.write(
                    "{\n"
                    f"        self.inner.{method.name}({', '.join(arg.name for arg in method.arguments if arg.name != 'self')})\n"
                    "}\n"
                )
            else:
                method.write_header_to(document)
                document.write(
                    "{\n"
                    f"        {flat_variant.name}::{method.name}({', '.join(arg.name for arg in method.arguments)})\n"
                    "}\n"
                )

        document.write("}\n")
    

        # We implement the bidirectional From methods for the nested struct
        # present in the web_common crate, which does not use Diesel or its
        # structs, but the web_common version of the structs.
        document.write(
            f"impl From<web_common::database::nested_models::{nested_struct.name}> for {nested_struct.name} {{\n"
            f"    fn from(item: web_common::database::nested_models::{nested_struct.name}) -> Self {{\n"
            "        Self {\n"
        )
        for attribute in nested_struct.attributes:
            if attribute.optional:
                if attribute.implements_copy():
                    document.write(
                        f"            {attribute.name}: item.{attribute.name}.map({attribute.data_type()}::from),\n"
                    )
                else:
                    document.write(
                        f"            {attribute.name}: item.{attribute.name}.as_deref().cloned().map({attribute.data_type()}::from),\n"
                    )
            else:
                if attribute.implements_copy():
                    document.write(
                        f"            {attribute.name}: {attribute.data_type()}::from(item.{attribute.name}),\n"
                    )
                else:
                    document.write(
                        f"            {attribute.name}: {attribute.data_type()}::from(item.{attribute.name}.as_ref().clone()),\n"
                    )
        document.write("        }\n    }\n}\n")

        document.write(
            f"impl From<{nested_struct.name}> for web_common::database::nested_models::{nested_struct.name} {{\n"
            f"    fn from(item: {nested_struct.name}) -> Self {{\n"
            "        Self {\n"
        )
        for attribute in nested_struct.attributes:
            if attribute.optional:
                rc_variant = ""
                if not attribute.implements_copy():
                    rc_variant = ".map(Rc::from)"
                    
                document.write(
                    f"            {attribute.name}: item.{attribute.name}.map(web_common::database::{attribute.data_type()}::from){rc_variant},\n"
                )
            else:
                if attribute.implements_copy():
                    document.write(
                        f"            {attribute.name}: web_common::database::{attribute.data_type()}::from(item.{attribute.name}),\n"
                    )
                else:
                    document.write(
                        f"            {attribute.name}: Rc::from(web_common::database::{attribute.data_type()}::from(item.{attribute.name})),\n"
                    )
        document.write("        }\n    }\n}\n")
        document.close()

    module_document.close()

    return nested_structs
