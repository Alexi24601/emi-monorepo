//! This module contains the RowToBadge implementation for the SearchableStruct enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use super::{BadgeProps, RowToBadge};
use crate::router::AppRoute;
use web_common::database::*;
use yew::prelude::*;

impl RowToBadge for SearchableStruct {
    fn badge_title(&self) -> String {
        match self {
            SearchableStruct::NestedDerivedSample(value) => value.badge_title(),
            SearchableStruct::NestedDocumentFormat(value) => value.badge_title(),
            SearchableStruct::NestedNameplateCategory(value) => value.badge_title(),
            SearchableStruct::NestedNameplate(value) => value.badge_title(),
            SearchableStruct::NestedObservationSubject(value) => value.badge_title(),
            SearchableStruct::NestedObservation(value) => value.badge_title(),
            SearchableStruct::NestedOrganismTaxon(value) => value.badge_title(),
            SearchableStruct::NestedOrganism(value) => value.badge_title(),
            SearchableStruct::NestedOrganization(value) => value.badge_title(),
            SearchableStruct::NestedProjectState(value) => value.badge_title(),
            SearchableStruct::NestedProject(value) => value.badge_title(),
            SearchableStruct::NestedProjectsTeamsRoleInvitation(value) => value.badge_title(),
            SearchableStruct::NestedProjectsTeamsRoleRequest(value) => value.badge_title(),
            SearchableStruct::NestedProjectsTeamsRole(value) => value.badge_title(),
            SearchableStruct::NestedProjectsUsersRoleInvitation(value) => value.badge_title(),
            SearchableStruct::NestedProjectsUsersRoleRequest(value) => value.badge_title(),
            SearchableStruct::NestedProjectsUsersRole(value) => value.badge_title(),
            SearchableStruct::NestedRank(value) => value.badge_title(),
            SearchableStruct::NestedRole(value) => value.badge_title(),
            SearchableStruct::NestedSampleContainerCategory(value) => value.badge_title(),
            SearchableStruct::NestedSampleContainer(value) => value.badge_title(),
            SearchableStruct::NestedSampleState(value) => value.badge_title(),
            SearchableStruct::NestedSampleTaxon(value) => value.badge_title(),
            SearchableStruct::NestedSample(value) => value.badge_title(),
            SearchableStruct::NestedSpectraCollection(value) => value.badge_title(),
            SearchableStruct::NestedTaxon(value) => value.badge_title(),
            SearchableStruct::NestedTeamState(value) => value.badge_title(),
            SearchableStruct::NestedTeam(value) => value.badge_title(),
            SearchableStruct::NestedTeamsTeamsRoleInvitation(value) => value.badge_title(),
            SearchableStruct::NestedTeamsUsersRoleInvitation(value) => value.badge_title(),
            SearchableStruct::NestedTeamsUsersRoleRequest(value) => value.badge_title(),
            SearchableStruct::NestedTeamsUsersRole(value) => value.badge_title(),
            SearchableStruct::NestedUnit(value) => value.badge_title(),
            SearchableStruct::NestedUser(value) => value.badge_title(),
            SearchableStruct::NestedUsersUsersRoleInvitation(value) => value.badge_title(),
            SearchableStruct::NestedUsersUsersRoleRequest(value) => value.badge_title(),
            SearchableStruct::NestedUsersUsersRole(value) => value.badge_title(),
            SearchableStruct::Color(value) => value.badge_title(),
            SearchableStruct::Country(value) => value.badge_title(),
            SearchableStruct::FontAwesomeIcon(value) => value.badge_title(),
        }
    }

    fn path(&self) -> Option<AppRoute> {
        match self {
            SearchableStruct::NestedDerivedSample(value) => value.path(),
            SearchableStruct::NestedDocumentFormat(value) => value.path(),
            SearchableStruct::NestedNameplateCategory(value) => value.path(),
            SearchableStruct::NestedNameplate(value) => value.path(),
            SearchableStruct::NestedObservationSubject(value) => value.path(),
            SearchableStruct::NestedObservation(value) => value.path(),
            SearchableStruct::NestedOrganismTaxon(value) => value.path(),
            SearchableStruct::NestedOrganism(value) => value.path(),
            SearchableStruct::NestedOrganization(value) => value.path(),
            SearchableStruct::NestedProjectState(value) => value.path(),
            SearchableStruct::NestedProject(value) => value.path(),
            SearchableStruct::NestedProjectsTeamsRoleInvitation(value) => value.path(),
            SearchableStruct::NestedProjectsTeamsRoleRequest(value) => value.path(),
            SearchableStruct::NestedProjectsTeamsRole(value) => value.path(),
            SearchableStruct::NestedProjectsUsersRoleInvitation(value) => value.path(),
            SearchableStruct::NestedProjectsUsersRoleRequest(value) => value.path(),
            SearchableStruct::NestedProjectsUsersRole(value) => value.path(),
            SearchableStruct::NestedRank(value) => value.path(),
            SearchableStruct::NestedRole(value) => value.path(),
            SearchableStruct::NestedSampleContainerCategory(value) => value.path(),
            SearchableStruct::NestedSampleContainer(value) => value.path(),
            SearchableStruct::NestedSampleState(value) => value.path(),
            SearchableStruct::NestedSampleTaxon(value) => value.path(),
            SearchableStruct::NestedSample(value) => value.path(),
            SearchableStruct::NestedSpectraCollection(value) => value.path(),
            SearchableStruct::NestedTaxon(value) => value.path(),
            SearchableStruct::NestedTeamState(value) => value.path(),
            SearchableStruct::NestedTeam(value) => value.path(),
            SearchableStruct::NestedTeamsTeamsRoleInvitation(value) => value.path(),
            SearchableStruct::NestedTeamsUsersRoleInvitation(value) => value.path(),
            SearchableStruct::NestedTeamsUsersRoleRequest(value) => value.path(),
            SearchableStruct::NestedTeamsUsersRole(value) => value.path(),
            SearchableStruct::NestedUnit(value) => value.path(),
            SearchableStruct::NestedUser(value) => value.path(),
            SearchableStruct::NestedUsersUsersRoleInvitation(value) => value.path(),
            SearchableStruct::NestedUsersUsersRoleRequest(value) => value.path(),
            SearchableStruct::NestedUsersUsersRole(value) => value.path(),
            SearchableStruct::Color(value) => value.path(),
            SearchableStruct::Country(value) => value.path(),
            SearchableStruct::FontAwesomeIcon(value) => value.path(),
        }
    }

    fn primary_image_url(&self) -> Option<String> {
        match self {
            SearchableStruct::NestedDerivedSample(value) => value.primary_image_url(),
            SearchableStruct::NestedDocumentFormat(value) => value.primary_image_url(),
            SearchableStruct::NestedNameplateCategory(value) => value.primary_image_url(),
            SearchableStruct::NestedNameplate(value) => value.primary_image_url(),
            SearchableStruct::NestedObservationSubject(value) => value.primary_image_url(),
            SearchableStruct::NestedObservation(value) => value.primary_image_url(),
            SearchableStruct::NestedOrganismTaxon(value) => value.primary_image_url(),
            SearchableStruct::NestedOrganism(value) => value.primary_image_url(),
            SearchableStruct::NestedOrganization(value) => value.primary_image_url(),
            SearchableStruct::NestedProjectState(value) => value.primary_image_url(),
            SearchableStruct::NestedProject(value) => value.primary_image_url(),
            SearchableStruct::NestedProjectsTeamsRoleInvitation(value) => value.primary_image_url(),
            SearchableStruct::NestedProjectsTeamsRoleRequest(value) => value.primary_image_url(),
            SearchableStruct::NestedProjectsTeamsRole(value) => value.primary_image_url(),
            SearchableStruct::NestedProjectsUsersRoleInvitation(value) => value.primary_image_url(),
            SearchableStruct::NestedProjectsUsersRoleRequest(value) => value.primary_image_url(),
            SearchableStruct::NestedProjectsUsersRole(value) => value.primary_image_url(),
            SearchableStruct::NestedRank(value) => value.primary_image_url(),
            SearchableStruct::NestedRole(value) => value.primary_image_url(),
            SearchableStruct::NestedSampleContainerCategory(value) => value.primary_image_url(),
            SearchableStruct::NestedSampleContainer(value) => value.primary_image_url(),
            SearchableStruct::NestedSampleState(value) => value.primary_image_url(),
            SearchableStruct::NestedSampleTaxon(value) => value.primary_image_url(),
            SearchableStruct::NestedSample(value) => value.primary_image_url(),
            SearchableStruct::NestedSpectraCollection(value) => value.primary_image_url(),
            SearchableStruct::NestedTaxon(value) => value.primary_image_url(),
            SearchableStruct::NestedTeamState(value) => value.primary_image_url(),
            SearchableStruct::NestedTeam(value) => value.primary_image_url(),
            SearchableStruct::NestedTeamsTeamsRoleInvitation(value) => value.primary_image_url(),
            SearchableStruct::NestedTeamsUsersRoleInvitation(value) => value.primary_image_url(),
            SearchableStruct::NestedTeamsUsersRoleRequest(value) => value.primary_image_url(),
            SearchableStruct::NestedTeamsUsersRole(value) => value.primary_image_url(),
            SearchableStruct::NestedUnit(value) => value.primary_image_url(),
            SearchableStruct::NestedUser(value) => value.primary_image_url(),
            SearchableStruct::NestedUsersUsersRoleInvitation(value) => value.primary_image_url(),
            SearchableStruct::NestedUsersUsersRoleRequest(value) => value.primary_image_url(),
            SearchableStruct::NestedUsersUsersRole(value) => value.primary_image_url(),
            SearchableStruct::Color(value) => value.primary_image_url(),
            SearchableStruct::Country(value) => value.primary_image_url(),
            SearchableStruct::FontAwesomeIcon(value) => value.primary_image_url(),
        }
    }

    fn font_awesome_icon(&self) -> Option<&str> {
        match self {
            SearchableStruct::NestedDerivedSample(value) => value.font_awesome_icon(),
            SearchableStruct::NestedDocumentFormat(value) => value.font_awesome_icon(),
            SearchableStruct::NestedNameplateCategory(value) => value.font_awesome_icon(),
            SearchableStruct::NestedNameplate(value) => value.font_awesome_icon(),
            SearchableStruct::NestedObservationSubject(value) => value.font_awesome_icon(),
            SearchableStruct::NestedObservation(value) => value.font_awesome_icon(),
            SearchableStruct::NestedOrganismTaxon(value) => value.font_awesome_icon(),
            SearchableStruct::NestedOrganism(value) => value.font_awesome_icon(),
            SearchableStruct::NestedOrganization(value) => value.font_awesome_icon(),
            SearchableStruct::NestedProjectState(value) => value.font_awesome_icon(),
            SearchableStruct::NestedProject(value) => value.font_awesome_icon(),
            SearchableStruct::NestedProjectsTeamsRoleInvitation(value) => value.font_awesome_icon(),
            SearchableStruct::NestedProjectsTeamsRoleRequest(value) => value.font_awesome_icon(),
            SearchableStruct::NestedProjectsTeamsRole(value) => value.font_awesome_icon(),
            SearchableStruct::NestedProjectsUsersRoleInvitation(value) => value.font_awesome_icon(),
            SearchableStruct::NestedProjectsUsersRoleRequest(value) => value.font_awesome_icon(),
            SearchableStruct::NestedProjectsUsersRole(value) => value.font_awesome_icon(),
            SearchableStruct::NestedRank(value) => value.font_awesome_icon(),
            SearchableStruct::NestedRole(value) => value.font_awesome_icon(),
            SearchableStruct::NestedSampleContainerCategory(value) => value.font_awesome_icon(),
            SearchableStruct::NestedSampleContainer(value) => value.font_awesome_icon(),
            SearchableStruct::NestedSampleState(value) => value.font_awesome_icon(),
            SearchableStruct::NestedSampleTaxon(value) => value.font_awesome_icon(),
            SearchableStruct::NestedSample(value) => value.font_awesome_icon(),
            SearchableStruct::NestedSpectraCollection(value) => value.font_awesome_icon(),
            SearchableStruct::NestedTaxon(value) => value.font_awesome_icon(),
            SearchableStruct::NestedTeamState(value) => value.font_awesome_icon(),
            SearchableStruct::NestedTeam(value) => value.font_awesome_icon(),
            SearchableStruct::NestedTeamsTeamsRoleInvitation(value) => value.font_awesome_icon(),
            SearchableStruct::NestedTeamsUsersRoleInvitation(value) => value.font_awesome_icon(),
            SearchableStruct::NestedTeamsUsersRoleRequest(value) => value.font_awesome_icon(),
            SearchableStruct::NestedTeamsUsersRole(value) => value.font_awesome_icon(),
            SearchableStruct::NestedUnit(value) => value.font_awesome_icon(),
            SearchableStruct::NestedUser(value) => value.font_awesome_icon(),
            SearchableStruct::NestedUsersUsersRoleInvitation(value) => value.font_awesome_icon(),
            SearchableStruct::NestedUsersUsersRoleRequest(value) => value.font_awesome_icon(),
            SearchableStruct::NestedUsersUsersRole(value) => value.font_awesome_icon(),
            SearchableStruct::Color(value) => value.font_awesome_icon(),
            SearchableStruct::Country(value) => value.font_awesome_icon(),
            SearchableStruct::FontAwesomeIcon(value) => value.font_awesome_icon(),
        }
    }

    fn children(&self, props: &BadgeProps<Self>) -> Option<Html> {
        match self {
            SearchableStruct::NestedDerivedSample(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedDocumentFormat(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedNameplateCategory(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedNameplate(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedObservationSubject(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedObservation(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedOrganismTaxon(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedOrganism(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedOrganization(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedProjectState(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedProject(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedProjectsTeamsRoleInvitation(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedProjectsTeamsRoleRequest(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedProjectsTeamsRole(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedProjectsUsersRoleInvitation(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedProjectsUsersRoleRequest(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedProjectsUsersRole(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedRank(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedRole(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedSampleContainerCategory(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedSampleContainer(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedSampleState(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedSampleTaxon(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedSample(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedSpectraCollection(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedTaxon(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedTeamState(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedTeam(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedTeamsTeamsRoleInvitation(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedTeamsUsersRoleInvitation(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedTeamsUsersRoleRequest(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedTeamsUsersRole(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedUnit(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedUser(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedUsersUsersRoleInvitation(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedUsersUsersRoleRequest(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::NestedUsersUsersRole(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::Color(value) => value.children(&props.to_child_props(value.clone())),
            SearchableStruct::Country(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
            SearchableStruct::FontAwesomeIcon(value) => {
                value.children(&props.to_child_props(value.clone()))
            }
        }
    }
}
