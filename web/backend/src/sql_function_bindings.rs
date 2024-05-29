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

diesel::expression::functions::sql_function! {
   fn can_update_spectra_collections(
        author_user_id: diesel::sql_types::Integer,
        this_spectra_collections_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_admin_spectra_collections(
        author_user_id: diesel::sql_types::Integer,
        this_spectra_collections_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_view_spectra_collections(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        this_spectra_collections_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_update_spectra(
        author_user_id: diesel::sql_types::Integer,
        this_spectra_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_admin_spectra(
        author_user_id: diesel::sql_types::Integer,
        this_spectra_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_view_spectra(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        this_spectra_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_update_sample_bio_ott_taxon_items(
        author_user_id: diesel::sql_types::Integer,
        this_sample_bio_ott_taxon_items_sample_id: diesel::sql_types::Uuid,
        this_sample_bio_ott_taxon_items_taxon_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_admin_sample_bio_ott_taxon_items(
        author_user_id: diesel::sql_types::Integer,
        this_sample_bio_ott_taxon_items_sample_id: diesel::sql_types::Uuid,
        this_sample_bio_ott_taxon_items_taxon_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_view_sample_bio_ott_taxon_items(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        this_sample_bio_ott_taxon_items_sample_id: diesel::sql_types::Uuid,
        this_sample_bio_ott_taxon_items_taxon_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_update_organisms(
        author_user_id: diesel::sql_types::Integer,
        this_organisms_id: diesel::sql_types::Uuid,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_admin_organisms(
        author_user_id: diesel::sql_types::Integer,
        this_organisms_id: diesel::sql_types::Uuid,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_view_organisms(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        this_organisms_id: diesel::sql_types::Uuid,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_update_observations(
        author_user_id: diesel::sql_types::Integer,
        this_observations_id: diesel::sql_types::Uuid,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_admin_observations(
        author_user_id: diesel::sql_types::Integer,
        this_observations_id: diesel::sql_types::Uuid,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_view_observations(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        this_observations_id: diesel::sql_types::Uuid,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_update_organism_bio_ott_taxon_items(
        author_user_id: diesel::sql_types::Integer,
        this_organism_bio_ott_taxon_items_organism_id: diesel::sql_types::Uuid,
        this_organism_bio_ott_taxon_items_taxon_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_admin_organism_bio_ott_taxon_items(
        author_user_id: diesel::sql_types::Integer,
        this_organism_bio_ott_taxon_items_organism_id: diesel::sql_types::Uuid,
        this_organism_bio_ott_taxon_items_taxon_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_view_organism_bio_ott_taxon_items(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        this_organism_bio_ott_taxon_items_organism_id: diesel::sql_types::Uuid,
        this_organism_bio_ott_taxon_items_taxon_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn similarity(
        arg_0: diesel::sql_types::Text,
        arg_1: diesel::sql_types::Text,
    ) -> diesel::sql_types::Float;
}

diesel::expression::functions::sql_function! {
   fn similarity_op(
        arg_0: diesel::sql_types::Text,
        arg_1: diesel::sql_types::Text,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn word_similarity(
        arg_0: diesel::sql_types::Text,
        arg_1: diesel::sql_types::Text,
    ) -> diesel::sql_types::Float;
}

diesel::expression::functions::sql_function! {
   fn word_similarity_op(
        arg_0: diesel::sql_types::Text,
        arg_1: diesel::sql_types::Text,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn word_similarity_commutator_op(
        arg_0: diesel::sql_types::Text,
        arg_1: diesel::sql_types::Text,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn similarity_dist(
        arg_0: diesel::sql_types::Text,
        arg_1: diesel::sql_types::Text,
    ) -> diesel::sql_types::Float;
}

diesel::expression::functions::sql_function! {
   fn word_similarity_dist_op(
        arg_0: diesel::sql_types::Text,
        arg_1: diesel::sql_types::Text,
    ) -> diesel::sql_types::Float;
}

diesel::expression::functions::sql_function! {
   fn word_similarity_dist_commutator_op(
        arg_0: diesel::sql_types::Text,
        arg_1: diesel::sql_types::Text,
    ) -> diesel::sql_types::Float;
}

diesel::expression::functions::sql_function! {
   fn strict_word_similarity(
        arg_0: diesel::sql_types::Text,
        arg_1: diesel::sql_types::Text,
    ) -> diesel::sql_types::Float;
}

diesel::expression::functions::sql_function! {
   fn strict_word_similarity_op(
        arg_0: diesel::sql_types::Text,
        arg_1: diesel::sql_types::Text,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn strict_word_similarity_commutator_op(
        arg_0: diesel::sql_types::Text,
        arg_1: diesel::sql_types::Text,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn strict_word_similarity_dist_op(
        arg_0: diesel::sql_types::Text,
        arg_1: diesel::sql_types::Text,
    ) -> diesel::sql_types::Float;
}

diesel::expression::functions::sql_function! {
   fn strict_word_similarity_dist_commutator_op(
        arg_0: diesel::sql_types::Text,
        arg_1: diesel::sql_types::Text,
    ) -> diesel::sql_types::Float;
}

diesel::expression::functions::sql_function! {
   fn concat_colors_name(
        name: diesel::sql_types::Text,
        description: diesel::sql_types::Text,
    ) -> diesel::sql_types::Text;
}

diesel::expression::functions::sql_function! {
   fn concat_font_awesome_icons_name(
        name: diesel::sql_types::Text,
        description: diesel::sql_types::Text,
    ) -> diesel::sql_types::Text;
}

diesel::expression::functions::sql_function! {
   fn concat_sample_container_categories_brand(
        name: diesel::sql_types::Text,
        description: diesel::sql_types::Text,
    ) -> diesel::sql_types::Text;
}

diesel::expression::functions::sql_function! {
   fn concat_roles_name(
        name: diesel::sql_types::Text,
        description: diesel::sql_types::Text,
    ) -> diesel::sql_types::Text;
}

diesel::expression::functions::sql_function! {
   fn can_update_users(
        author_user_id: diesel::sql_types::Integer,
        this_users_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_admin_users(
        author_user_id: diesel::sql_types::Integer,
        this_users_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn concat_users_name(
        first_name: diesel::sql_types::Text,
        middle_name: diesel::sql_types::Nullable<diesel::sql_types::Text>,
        last_name: diesel::sql_types::Text,
    ) -> diesel::sql_types::Text;
}

diesel::expression::functions::sql_function! {
   fn can_update_users_users_role_requests(
        author_user_id: diesel::sql_types::Integer,
        this_users_users_role_requests_table_id: diesel::sql_types::Integer,
        this_users_users_role_requests_user_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_admin_users_users_role_requests(
        author_user_id: diesel::sql_types::Integer,
        this_users_users_role_requests_table_id: diesel::sql_types::Integer,
        this_users_users_role_requests_user_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_view_users_users_role_requests(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        this_users_users_role_requests_table_id: diesel::sql_types::Integer,
        this_users_users_role_requests_user_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_update_users_users_roles(
        author_user_id: diesel::sql_types::Integer,
        this_users_users_roles_table_id: diesel::sql_types::Integer,
        this_users_users_roles_user_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_admin_users_users_roles(
        author_user_id: diesel::sql_types::Integer,
        this_users_users_roles_table_id: diesel::sql_types::Integer,
        this_users_users_roles_user_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_update_users_users_role_invitations(
        author_user_id: diesel::sql_types::Integer,
        this_users_users_role_invitations_table_id: diesel::sql_types::Integer,
        this_users_users_role_invitations_user_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_admin_users_users_role_invitations(
        author_user_id: diesel::sql_types::Integer,
        this_users_users_role_invitations_table_id: diesel::sql_types::Integer,
        this_users_users_role_invitations_user_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_view_users_users_role_invitations(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        this_users_users_role_invitations_table_id: diesel::sql_types::Integer,
        this_users_users_role_invitations_user_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn concat_team_states_name_description(
        name: diesel::sql_types::Text,
        description: diesel::sql_types::Text,
    ) -> diesel::sql_types::Text;
}

diesel::expression::functions::sql_function! {
   fn can_update_teams(
        author_user_id: diesel::sql_types::Integer,
        this_teams_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_admin_teams(
        author_user_id: diesel::sql_types::Integer,
        this_teams_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn concat_teams_name_description(
        name: diesel::sql_types::Text,
        description: diesel::sql_types::Text,
    ) -> diesel::sql_types::Text;
}

diesel::expression::functions::sql_function! {
   fn can_update_teams_users_roles(
        author_user_id: diesel::sql_types::Integer,
        this_teams_users_roles_table_id: diesel::sql_types::Integer,
        this_teams_users_roles_user_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_admin_teams_users_roles(
        author_user_id: diesel::sql_types::Integer,
        this_teams_users_roles_table_id: diesel::sql_types::Integer,
        this_teams_users_roles_user_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_update_teams_users_role_requests(
        author_user_id: diesel::sql_types::Integer,
        this_teams_users_role_requests_table_id: diesel::sql_types::Integer,
        this_teams_users_role_requests_user_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_admin_teams_users_role_requests(
        author_user_id: diesel::sql_types::Integer,
        this_teams_users_role_requests_table_id: diesel::sql_types::Integer,
        this_teams_users_role_requests_user_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_view_teams_users_role_requests(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        this_teams_users_role_requests_table_id: diesel::sql_types::Integer,
        this_teams_users_role_requests_user_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_update_teams_users_role_invitations(
        author_user_id: diesel::sql_types::Integer,
        this_teams_users_role_invitations_table_id: diesel::sql_types::Integer,
        this_teams_users_role_invitations_user_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_admin_teams_users_role_invitations(
        author_user_id: diesel::sql_types::Integer,
        this_teams_users_role_invitations_table_id: diesel::sql_types::Integer,
        this_teams_users_role_invitations_user_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_view_teams_users_role_invitations(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        this_teams_users_role_invitations_table_id: diesel::sql_types::Integer,
        this_teams_users_role_invitations_user_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_update_teams_teams_role_invitations(
        author_user_id: diesel::sql_types::Integer,
        this_teams_teams_role_invitations_table_id: diesel::sql_types::Integer,
        this_teams_teams_role_invitations_team_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_admin_teams_teams_role_invitations(
        author_user_id: diesel::sql_types::Integer,
        this_teams_teams_role_invitations_table_id: diesel::sql_types::Integer,
        this_teams_teams_role_invitations_team_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_view_teams_teams_role_invitations(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        this_teams_teams_role_invitations_table_id: diesel::sql_types::Integer,
        this_teams_teams_role_invitations_team_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_update_user_emails(
        author_user_id: diesel::sql_types::Integer,
        this_user_emails_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_admin_user_emails(
        author_user_id: diesel::sql_types::Integer,
        this_user_emails_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_view_user_emails(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        this_user_emails_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn concat_sample_states_name_description(
        name: diesel::sql_types::Text,
        description: diesel::sql_types::Text,
    ) -> diesel::sql_types::Text;
}

diesel::expression::functions::sql_function! {
   fn concat_bio_ott_ranks_name_description(
        name: diesel::sql_types::Text,
        description: diesel::sql_types::Text,
    ) -> diesel::sql_types::Text;
}

diesel::expression::functions::sql_function! {
   fn concat_document_formats_extension_mime_type(
        extension: diesel::sql_types::Text,
        mime_type: diesel::sql_types::Text,
    ) -> diesel::sql_types::Text;
}

diesel::expression::functions::sql_function! {
   fn concat_project_states_name_description(
        name: diesel::sql_types::Text,
        description: diesel::sql_types::Text,
    ) -> diesel::sql_types::Text;
}

diesel::expression::functions::sql_function! {
   fn can_update_projects(
        author_user_id: diesel::sql_types::Integer,
        this_projects_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_admin_projects(
        author_user_id: diesel::sql_types::Integer,
        this_projects_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_view_projects(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        this_projects_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn concat_projects_name_description(
        name: diesel::sql_types::Text,
        description: diesel::sql_types::Text,
    ) -> diesel::sql_types::Text;
}

diesel::expression::functions::sql_function! {
   fn can_update_sample_containers(
        author_user_id: diesel::sql_types::Integer,
        this_sample_containers_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_admin_sample_containers(
        author_user_id: diesel::sql_types::Integer,
        this_sample_containers_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_view_sample_containers(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        this_sample_containers_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_update_projects_teams_roles(
        author_user_id: diesel::sql_types::Integer,
        this_projects_teams_roles_table_id: diesel::sql_types::Integer,
        this_projects_teams_roles_team_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_admin_projects_teams_roles(
        author_user_id: diesel::sql_types::Integer,
        this_projects_teams_roles_table_id: diesel::sql_types::Integer,
        this_projects_teams_roles_team_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_view_projects_teams_roles(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        this_projects_teams_roles_table_id: diesel::sql_types::Integer,
        this_projects_teams_roles_team_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn concat_nameplate_categories_brand(
        name: diesel::sql_types::Text,
        description: diesel::sql_types::Text,
    ) -> diesel::sql_types::Text;
}

diesel::expression::functions::sql_function! {
   fn can_update_nameplates(
        author_user_id: diesel::sql_types::Integer,
        this_nameplates_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_admin_nameplates(
        author_user_id: diesel::sql_types::Integer,
        this_nameplates_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_view_nameplates(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        this_nameplates_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_update_samples(
        author_user_id: diesel::sql_types::Integer,
        this_samples_id: diesel::sql_types::Uuid,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_admin_samples(
        author_user_id: diesel::sql_types::Integer,
        this_samples_id: diesel::sql_types::Uuid,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_view_samples(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        this_samples_id: diesel::sql_types::Uuid,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_update_projects_users_roles(
        author_user_id: diesel::sql_types::Integer,
        this_projects_users_roles_table_id: diesel::sql_types::Integer,
        this_projects_users_roles_user_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_admin_projects_users_roles(
        author_user_id: diesel::sql_types::Integer,
        this_projects_users_roles_table_id: diesel::sql_types::Integer,
        this_projects_users_roles_user_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_view_projects_users_roles(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        this_projects_users_roles_table_id: diesel::sql_types::Integer,
        this_projects_users_roles_user_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_update_projects_teams_role_requests(
        author_user_id: diesel::sql_types::Integer,
        this_projects_teams_role_requests_table_id: diesel::sql_types::Integer,
        this_projects_teams_role_requests_team_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_admin_projects_teams_role_requests(
        author_user_id: diesel::sql_types::Integer,
        this_projects_teams_role_requests_table_id: diesel::sql_types::Integer,
        this_projects_teams_role_requests_team_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_view_projects_teams_role_requests(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        this_projects_teams_role_requests_table_id: diesel::sql_types::Integer,
        this_projects_teams_role_requests_team_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_update_derived_samples(
        author_user_id: diesel::sql_types::Integer,
        this_derived_samples_parent_sample_id: diesel::sql_types::Uuid,
        this_derived_samples_child_sample_id: diesel::sql_types::Uuid,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_admin_derived_samples(
        author_user_id: diesel::sql_types::Integer,
        this_derived_samples_parent_sample_id: diesel::sql_types::Uuid,
        this_derived_samples_child_sample_id: diesel::sql_types::Uuid,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_view_derived_samples(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        this_derived_samples_parent_sample_id: diesel::sql_types::Uuid,
        this_derived_samples_child_sample_id: diesel::sql_types::Uuid,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_update_projects_teams_role_invitations(
        author_user_id: diesel::sql_types::Integer,
        this_projects_teams_role_invitations_table_id: diesel::sql_types::Integer,
        this_projects_teams_role_invitations_team_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_admin_projects_teams_role_invitations(
        author_user_id: diesel::sql_types::Integer,
        this_projects_teams_role_invitations_table_id: diesel::sql_types::Integer,
        this_projects_teams_role_invitations_team_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_view_projects_teams_role_invitations(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        this_projects_teams_role_invitations_table_id: diesel::sql_types::Integer,
        this_projects_teams_role_invitations_team_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_update_projects_users_role_invitations(
        author_user_id: diesel::sql_types::Integer,
        this_projects_users_role_invitations_table_id: diesel::sql_types::Integer,
        this_projects_users_role_invitations_user_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_admin_projects_users_role_invitations(
        author_user_id: diesel::sql_types::Integer,
        this_projects_users_role_invitations_table_id: diesel::sql_types::Integer,
        this_projects_users_role_invitations_user_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_view_projects_users_role_invitations(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        this_projects_users_role_invitations_table_id: diesel::sql_types::Integer,
        this_projects_users_role_invitations_user_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_update_projects_users_role_requests(
        author_user_id: diesel::sql_types::Integer,
        this_projects_users_role_requests_table_id: diesel::sql_types::Integer,
        this_projects_users_role_requests_user_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_admin_projects_users_role_requests(
        author_user_id: diesel::sql_types::Integer,
        this_projects_users_role_requests_table_id: diesel::sql_types::Integer,
        this_projects_users_role_requests_user_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn can_view_projects_users_role_requests(
        author_user_id: diesel::sql_types::Nullable<diesel::sql_types::Integer>,
        this_projects_users_role_requests_table_id: diesel::sql_types::Integer,
        this_projects_users_role_requests_user_id: diesel::sql_types::Integer,
    ) -> diesel::sql_types::Bool;
}

diesel::expression::functions::sql_function! {
   fn concat_units_name_description_symbol(
        name: diesel::sql_types::Text,
        description: diesel::sql_types::Text,
        symbol: diesel::sql_types::Text,
    ) -> diesel::sql_types::Text;
}

