//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL functions in the database.
//!
//! # Implementative details
//! All postgres function receive parameter that are potentially options, and there
//! is no way to explicitly write a function that receives a non-nullable parameter.
//! Therefore, all parameters are wrapped in an Option, and the check of whether the
//! parameter is null or not is done in the postgres function itself.

use diesel::prelude::*;

define_sql_function! {
   fn f_concat_colors_name(
        name: diesel::sql_types::Nullable<diesel::sql_types::Text>,
        description: diesel::sql_types::Nullable<diesel::sql_types::Text>,
    ) -> diesel::sql_types::Text,
}

define_sql_function! {
   fn f_concat_font_awesome_icons_name(
        name: diesel::sql_types::Nullable<diesel::sql_types::Text>,
        description: diesel::sql_types::Nullable<diesel::sql_types::Text>,
    ) -> diesel::sql_types::Text,
}

define_sql_function! {
   fn f_concat_sample_container_categories_brand(
        name: diesel::sql_types::Nullable<diesel::sql_types::Text>,
        description: diesel::sql_types::Nullable<diesel::sql_types::Text>,
    ) -> diesel::sql_types::Text,
}

define_sql_function! {
   fn f_concat_roles_name(
        name: diesel::sql_types::Nullable<diesel::sql_types::Text>,
        description: diesel::sql_types::Nullable<diesel::sql_types::Text>,
    ) -> diesel::sql_types::Text,
}

define_sql_function! {
   fn can_edit_users(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn can_delete_users(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn can_view_users(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn f_concat_users_name(
        first_value: diesel::sql_types::Nullable<diesel::sql_types::Text>,
        middle_value: diesel::sql_types::Nullable<diesel::sql_types::Text>,
        last_value: diesel::sql_types::Nullable<diesel::sql_types::Text>,
    ) -> diesel::sql_types::Text,
}

define_sql_function! {
   fn f_concat_team_states_name_description(
        name: diesel::sql_types::Nullable<diesel::sql_types::Text>,
        description: diesel::sql_types::Nullable<diesel::sql_types::Text>,
    ) -> diesel::sql_types::Text,
}

define_sql_function! {
   fn can_edit_teams(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn can_delete_teams(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn can_view_teams(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn f_concat_teams_name_description(
        name: diesel::sql_types::Nullable<diesel::sql_types::Text>,
        description: diesel::sql_types::Nullable<diesel::sql_types::Text>,
    ) -> diesel::sql_types::Text,
}

define_sql_function! {
   fn f_concat_sample_states_name_description(
        name: diesel::sql_types::Nullable<diesel::sql_types::Text>,
        description: diesel::sql_types::Nullable<diesel::sql_types::Text>,
    ) -> diesel::sql_types::Text,
}

define_sql_function! {
   fn f_concat_bio_ott_ranks_name_description(
        name: diesel::sql_types::Nullable<diesel::sql_types::Text>,
        description: diesel::sql_types::Nullable<diesel::sql_types::Text>,
    ) -> diesel::sql_types::Text,
}

define_sql_function! {
   fn f_concat_document_formats_extension_mime_type(
        extension: diesel::sql_types::Nullable<diesel::sql_types::Text>,
        mime_type: diesel::sql_types::Nullable<diesel::sql_types::Text>,
    ) -> diesel::sql_types::Text,
}

define_sql_function! {
   fn f_concat_project_states_name_description(
        name: diesel::sql_types::Nullable<diesel::sql_types::Text>,
        description: diesel::sql_types::Nullable<diesel::sql_types::Text>,
    ) -> diesel::sql_types::Text,
}

define_sql_function! {
   fn can_edit_projects(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn can_delete_projects(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn can_view_projects(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn f_concat_projects_name_description(
        name: diesel::sql_types::Nullable<diesel::sql_types::Text>,
        description: diesel::sql_types::Nullable<diesel::sql_types::Text>,
    ) -> diesel::sql_types::Text,
}

define_sql_function! {
   fn can_edit_sample_containers(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn can_delete_sample_containers(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn can_view_sample_containers(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn can_edit_samples(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        barcode_id: diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn can_delete_samples(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        barcode_id: diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn can_view_samples(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        barcode_id: diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn can_edit_derived_samples(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        parent_sample_id: diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
        child_sample_id: diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn can_delete_derived_samples(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        parent_sample_id: diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
        child_sample_id: diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn can_view_derived_samples(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        parent_sample_id: diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
        child_sample_id: diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn f_concat_units_name_description_symbol(
        name: diesel::sql_types::Nullable<diesel::sql_types::Text>,
        description: diesel::sql_types::Nullable<diesel::sql_types::Text>,
        symbol: diesel::sql_types::Nullable<diesel::sql_types::Text>,
    ) -> diesel::sql_types::Text,
}

define_sql_function! {
   fn can_edit_spectra_collections(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn can_delete_spectra_collections(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn can_view_spectra_collections(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn can_edit_sampled_individuals(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        id: diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn can_delete_sampled_individuals(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        id: diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn can_view_sampled_individuals(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        id: diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn f_concat_sampled_individuals_notes_barcode(
        notes: diesel::sql_types::Nullable<diesel::sql_types::Text>,
        barcode: diesel::sql_types::Nullable<diesel::sql_types::Text>,
    ) -> diesel::sql_types::Text,
}

define_sql_function! {
   fn can_edit_observations(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        id: diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn can_delete_observations(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        id: diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn can_view_observations(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        id: diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn can_edit_sample_bio_ott_taxon_items(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        sample_id: diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
        taxon_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn can_delete_sample_bio_ott_taxon_items(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        sample_id: diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
        taxon_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn can_view_sample_bio_ott_taxon_items(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        sample_id: diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
        taxon_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn can_edit_sampled_individual_bio_ott_taxon_items(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        sampled_individual_id: diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
        taxon_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn can_delete_sampled_individual_bio_ott_taxon_items(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        sampled_individual_id: diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
        taxon_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
    ) -> diesel::sql_types::Bool,
}

define_sql_function! {
   fn can_view_sampled_individual_bio_ott_taxon_items(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        sampled_individual_id: diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
        taxon_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
    ) -> diesel::sql_types::Bool,
}

