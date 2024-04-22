//! This module contains the nested structs for the database tables.
//!
//! This file is automatically generated. Do not write anything here.

use serde::Deserialize;
use serde::Serialize;
use super::tables::*;
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NestedContainerHorizontalRule {
    pub inner: ContainerHorizontalRule,
    pub created_by: User,
    pub item_type: NestedItemCategory,
    pub other_item_type: NestedItemCategory,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NestedContainerVerticalRule {
    pub inner: ContainerVerticalRule,
    pub created_by: User,
    pub container_item_type: NestedItemCategory,
    pub contained_item_type: NestedItemCategory,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NestedDocument {
    pub inner: Document,
    pub author: User,
    pub format: DocumentFormat,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NestedItemCategory {
    pub inner: ItemCategory,
    pub created_by: User,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NestedItemCategoryRelationship {
    pub inner: ItemCategoryRelationship,
    pub parent: NestedItemCategory,
    pub child: NestedItemCategory,
    pub added_by: User,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NestedItemCategoryUnit {
    pub inner: ItemCategoryUnit,
    pub item_category: NestedItemCategory,
    pub unit: Unit,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NestedItemContinuousQuantity {
    pub inner: ItemContinuousQuantity,
    pub item: NestedItem,
    pub unit: Unit,
    pub sensor: Option<NestedItem>,
    pub measured_by: Option<User>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NestedItemDiscreteQuantity {
    pub inner: ItemDiscreteQuantity,
    pub item: NestedItem,
    pub unit: Unit,
    pub measured_by: Option<User>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NestedItemLocation {
    pub inner: ItemLocation,
    pub item: Option<NestedItem>,
    pub located_by: Option<User>,
    pub location: Option<NestedLocation>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NestedItemUnit {
    pub inner: ItemUnit,
    pub item: NestedItem,
    pub unit: Unit,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NestedItem {
    pub inner: Item,
    pub parent_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NestedLocation {
    pub inner: Location,
    pub geolocalization_device: Option<NestedItem>,
    pub altitude_device: Option<NestedItem>,
    pub parent_location_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NestedManufacturedItemCategory {
    pub inner: ManufacturedItemCategory,
    pub manifacturer: NestedOrganization,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NestedNotification {
    pub inner: Notification,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NestedOrganization {
    pub inner: Organization,
    pub parent_organization_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NestedProcedureContinuousRequirement {
    pub inner: ProcedureContinuousRequirement,
    pub created_by: User,
    pub procedure: NestedProcedure,
    pub item_category: NestedItemCategory,
    pub unit: Option<Unit>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NestedProcedureDiscreteRequirement {
    pub inner: ProcedureDiscreteRequirement,
    pub created_by: User,
    pub procedure: NestedProcedure,
    pub item_category: NestedItemCategory,
    pub unit: Option<Unit>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NestedProcedure {
    pub inner: Procedure,
    pub created_by: Option<User>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NestedProjectRequirement {
    pub inner: ProjectRequirement,
    pub created_by: User,
    pub project: NestedProject,
    pub item_category: NestedItemCategory,
    pub unit: Option<Unit>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NestedProject {
    pub inner: Project,
    pub state: ProjectState,
    pub parent_project_id: Option<i32>,
    pub created_by: User,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NestedSampleTaxa {
    pub inner: SampleTaxa,
    pub created_by: User,
    pub sample: NestedSample,
    pub taxon: Taxa,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NestedSampledIndividualTaxa {
    pub inner: SampledIndividualTaxa,
    pub created_by: User,
    pub sampled_individual: SampledIndividual,
    pub taxon: Taxa,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NestedSample {
    pub inner: Sample,
    pub created_by: Option<User>,
    pub derived_from: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NestedSpectra {
    pub inner: Spectra,
    pub spectra_collection: NestedSpectraCollection,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NestedSpectraCollection {
    pub inner: SpectraCollection,
    pub sample: NestedSample,
    pub created_by: User,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NestedTeam {
    pub inner: Team,
    pub parent_team_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NestedUserEmail {
    pub inner: UserEmail,
    pub user: User,
    pub login_provider: LoginProvider,
}

