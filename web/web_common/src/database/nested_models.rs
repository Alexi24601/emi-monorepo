//! This module contains the nested structs for the database tables.
//!
//! This file is automatically generated. Do not write anything here.

use serde::Deserialize;
use serde::Serialize;
use super::tables::*;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedContainerHorizontalRule {
    pub inner: ContainerHorizontalRule,
    pub created_by: User,
    pub item_type: NestedItemCategory,
    pub other_item_type: NestedItemCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedContainerVerticalRule {
    pub inner: ContainerVerticalRule,
    pub created_by: User,
    pub container_item_type: NestedItemCategory,
    pub contained_item_type: NestedItemCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedDocument {
    pub inner: Document,
    pub author: User,
    pub format: DocumentFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedItemCategory {
    pub inner: ItemCategory,
    pub created_by: User,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedItemCategoryRelationship {
    pub inner: ItemCategoryRelationship,
    pub parent: NestedItemCategory,
    pub child: NestedItemCategory,
    pub added_by: User,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedItemCategoryUnit {
    pub inner: ItemCategoryUnit,
    pub item_category: NestedItemCategory,
    pub unit: Unit,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedItemContinuousQuantity {
    pub inner: ItemContinuousQuantity,
    pub item: NestedItem,
    pub unit: Unit,
    pub sensor: Option<NestedItem>,
    pub measured_by: Option<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedItemDiscreteQuantity {
    pub inner: ItemDiscreteQuantity,
    pub item: NestedItem,
    pub unit: Unit,
    pub measured_by: Option<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedItemLocation {
    pub inner: ItemLocation,
    pub item: Option<NestedItem>,
    pub located_by: Option<User>,
    pub location: Option<NestedLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedItemUnit {
    pub inner: ItemUnit,
    pub item: NestedItem,
    pub unit: Unit,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedItem {
    pub inner: Item,
    pub parent: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedLocation {
    pub inner: Location,
    pub geolocalization_device: Option<NestedItem>,
    pub altitude_device: Option<NestedItem>,
    pub parent_location: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedManufacturedItemCategory {
    pub inner: ManufacturedItemCategory,
    pub manifacturer: NestedOrganization,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedNotification {
    pub inner: Notification,
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedOrganization {
    pub inner: Organization,
    pub parent_organization: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedProcedureContinuousRequirement {
    pub inner: ProcedureContinuousRequirement,
    pub created_by: User,
    pub procedure: NestedProcedure,
    pub item_category: NestedItemCategory,
    pub unit: Option<Unit>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedProcedureDiscreteRequirement {
    pub inner: ProcedureDiscreteRequirement,
    pub created_by: User,
    pub procedure: NestedProcedure,
    pub item_category: NestedItemCategory,
    pub unit: Option<Unit>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedProcedure {
    pub inner: Procedure,
    pub created_by: Option<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedProjectRequirement {
    pub inner: ProjectRequirement,
    pub created_by: User,
    pub project: NestedProject,
    pub item_category: NestedItemCategory,
    pub unit: Option<Unit>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedProject {
    pub inner: Project,
    pub state: ProjectState,
    pub parent_project: Option<i32>,
    pub created_by: User,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedSampleTaxa {
    pub inner: SampleTaxa,
    pub created_by: User,
    pub sample: NestedSample,
    pub taxon: Taxa,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedSampledIndividualTaxa {
    pub inner: SampledIndividualTaxa,
    pub created_by: User,
    pub sampled_individual: SampledIndividual,
    pub taxon: Taxa,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedSample {
    pub inner: Sample,
    pub created_by: Option<User>,
    pub derived_from: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedSpectra {
    pub inner: Spectra,
    pub spectra_collection: NestedSpectraCollection,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedSpectraCollection {
    pub inner: SpectraCollection,
    pub sample: NestedSample,
    pub created_by: User,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedTeam {
    pub inner: Team,
    pub parent_team: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedUserEmail {
    pub inner: UserEmail,
    pub user: User,
    pub login_provider: LoginProvider,
}

