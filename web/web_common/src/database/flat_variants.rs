//! This file is automatically generated, do not modify it directly.
/// A struct that is associated to a table in the database.

pub trait Tabular {
    const TABLE: crate::database::Table;
}

/// A struct that is associated to a filter struct.
pub trait Filtrable: PartialEq {
    type Filter: serde::Serialize + PartialEq + Clone;
}

/// A struct that may be associated to a textual description.
pub trait Describable {
    fn description(&self) -> Option<&str>;
}

/// A struct that may be associated to a color.
pub trait Colorable {
    fn color(&self) -> Option<&str>;
}

#[cfg(feature = "frontend")]
/// A struct that can be queries with an all method.
pub trait AllRecords: Filtrable + Sized {
    fn all_records<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        filter: Option<&<Self as Filtrable>::Filter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> impl std::future::Future<Output = Result<Vec<Self>, crate::api::ApiError>>;
}

mod bio_ott_ranks;
pub use bio_ott_ranks::BioOttRank;
mod bio_ott_taxon_items;
pub use bio_ott_taxon_items::BioOttTaxonItem;
mod colors;
pub use colors::Color;
mod countries;
pub use countries::Country;
mod derived_samples;
pub use derived_samples::DerivedSample;
mod document_formats;
pub use document_formats::DocumentFormat;
mod font_awesome_icons;
pub use font_awesome_icons::FontAwesomeIcon;
mod login_providers;
pub use login_providers::LoginProvider;
mod materials;
pub use materials::Material;
mod nameplate_categories;
pub use nameplate_categories::NameplateCategory;
mod nameplates;
pub use nameplates::Nameplate;
mod notifications;
pub use notifications::Notification;
mod observation_subjects;
pub use observation_subjects::ObservationSubject;
mod observations;
pub use observations::Observation;
mod organism_bio_ott_taxon_items;
pub use organism_bio_ott_taxon_items::OrganismBioOttTaxonItem;
mod organisms;
pub use organisms::Organism;
mod organizations;
pub use organizations::Organization;
mod permanence_categories;
pub use permanence_categories::PermanenceCategory;
mod project_states;
pub use project_states::ProjectState;
mod projects;
pub use projects::Project;
mod projects_teams_role_invitations;
pub use projects_teams_role_invitations::ProjectsTeamsRoleInvitation;
mod projects_teams_role_requests;
pub use projects_teams_role_requests::ProjectsTeamsRoleRequest;
mod projects_teams_roles;
pub use projects_teams_roles::ProjectsTeamsRole;
mod projects_users_role_invitations;
pub use projects_users_role_invitations::ProjectsUsersRoleInvitation;
mod projects_users_role_requests;
pub use projects_users_role_requests::ProjectsUsersRoleRequest;
mod projects_users_roles;
pub use projects_users_roles::ProjectsUsersRole;
mod roles;
pub use roles::Role;
mod sample_bio_ott_taxon_items;
pub use sample_bio_ott_taxon_items::SampleBioOttTaxonItem;
mod sample_container_categories;
pub use sample_container_categories::SampleContainerCategory;
mod sample_containers;
pub use sample_containers::SampleContainer;
mod sample_states;
pub use sample_states::SampleState;
mod samples;
pub use samples::Sample;
mod spectra;
pub use spectra::Spectra;
mod spectra_collections;
pub use spectra_collections::SpectraCollection;
mod team_states;
pub use team_states::TeamState;
mod teams;
pub use teams::Team;
mod teams_teams_role_invitations;
pub use teams_teams_role_invitations::TeamsTeamsRoleInvitation;
mod teams_users_role_invitations;
pub use teams_users_role_invitations::TeamsUsersRoleInvitation;
mod teams_users_role_requests;
pub use teams_users_role_requests::TeamsUsersRoleRequest;
mod teams_users_roles;
pub use teams_users_roles::TeamsUsersRole;
mod units;
pub use units::Unit;
mod user_emails;
pub use user_emails::UserEmail;
mod users;
pub use users::User;
mod users_users_role_invitations;
pub use users_users_role_invitations::UsersUsersRoleInvitation;
mod users_users_role_requests;
pub use users_users_role_requests::UsersUsersRoleRequest;
mod users_users_roles;
pub use users_users_roles::UsersUsersRole;
