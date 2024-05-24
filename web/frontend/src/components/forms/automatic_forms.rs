//! This module contains the forms for the frontend.
//!
//! This module is automatically generated. Do not write anything here.

use serde::{Deserialize, Serialize};
use web_common::database::*;
use yew::prelude::*;
use yewdux::{use_store, Reducer, Store};
use crate::components::forms::*;
use web_common::api::form_traits::FormMethod;
use std::rc::Rc;
use uuid::Uuid;
use std::ops::Deref;
use yewdux::Dispatch;
use chrono::NaiveDateTime;
use web_common::api::ApiError;
use crate::workers::ws_worker::ComponentMessage;
use web_common::custom_validators::Image;
use web_common::file_formats::GenericFileFormat;

#[derive(Store, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct DerivedSampleBuilder {
    pub parent_sample: Option<NestedSample>,
    pub child_sample: Option<NestedSample>,
    pub errors_parent_sample: Vec<ApiError>,
    pub errors_child_sample: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

impl Default for DerivedSampleBuilder {
    fn default() -> Self {
        Self {
            parent_sample: Default::default(),
            child_sample: Default::default(),
            errors_parent_sample: Default::default(),
            errors_child_sample: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum DerivedSampleActions {
    SetParentSample(Option<NestedSample>),
    SetChildSample(Option<NestedSample>),
}

impl FromOperation for DerivedSampleActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "parent_sample" => DerivedSampleActions::SetParentSample(Some(bincode::deserialize(&row).unwrap())),
            "child_sample" => DerivedSampleActions::SetChildSample(Some(bincode::deserialize(&row).unwrap())),
            operation_name => unreachable!("The operation name '{}' is not supported.", operation_name),
        }
    }
}

impl Reducer<DerivedSampleBuilder> for DerivedSampleActions {
    fn apply(self, mut state: std::rc::Rc<DerivedSampleBuilder>) -> std::rc::Rc<DerivedSampleBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            DerivedSampleActions::SetParentSample(parent_sample) => 'parent_sample: {
                state_mut.errors_parent_sample.clear();
        if parent_sample.is_none() {
            state_mut.errors_parent_sample.push(ApiError::BadRequest(vec![
                "The Parent sample field is required.".to_string()
             ]));
            state_mut.parent_sample = None;
             break 'parent_sample;
        }
                state_mut.parent_sample = parent_sample;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'parent_sample;
            }
            DerivedSampleActions::SetChildSample(child_sample) => 'child_sample: {
                state_mut.errors_child_sample.clear();
        if child_sample.is_none() {
            state_mut.errors_child_sample.push(ApiError::BadRequest(vec![
                "The Child sample field is required.".to_string()
             ]));
            state_mut.child_sample = None;
             break 'child_sample;
        }
                state_mut.child_sample = child_sample;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'child_sample;
            }
        }
        state
    }
}
impl FormBuilder for DerivedSampleBuilder {
    type Actions = DerivedSampleActions;

    type RichVariant = NestedDerivedSample;

    fn has_errors(&self) -> bool {
!self.errors_parent_sample.is_empty() || !self.errors_child_sample.is_empty()
    }

    fn update(dispatcher: &Dispatch<Self>, richest_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.apply(DerivedSampleActions::SetParentSample(Some(richest_variant.parent_sample)));
        dispatcher.apply(DerivedSampleActions::SetChildSample(Some(richest_variant.child_sample)));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.parent_sample.is_some()
        && self.child_sample.is_some()
    }

}

impl From<DerivedSampleBuilder> for NewDerivedSample {
    fn from(builder: DerivedSampleBuilder) -> Self {
        Self {
            parent_sample_id: builder.parent_sample.as_ref().map(|parent_sample| parent_sample.inner.id).unwrap(),
            child_sample_id: builder.child_sample.as_ref().map(|child_sample| child_sample.inner.id).unwrap(),
        }
    }
}
impl FormBuildable for NewDerivedSample {
    type Builder = DerivedSampleBuilder;
    fn title() -> &'static str {
        "Derived sample"
    }
    fn task_target() -> &'static str {
        "Derived sample"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateDerivedSampleFormProp {
    pub parent_sample_id: Uuid,
    pub child_sample_id: Uuid,
}

#[function_component(CreateDerivedSampleForm)]
pub fn create_derived_sample_form(props: &CreateDerivedSampleFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<DerivedSampleBuilder>();
    named_requests.push(ComponentMessage::get_named::<&str, Sample>("parent_sample", props.parent_sample_id.into()));
    named_requests.push(ComponentMessage::get_named::<&str, Sample>("child_sample", props.child_sample_id.into()));
    let set_parent_sample = builder_dispatch.apply_callback(|parent_sample: Option<NestedSample>| DerivedSampleActions::SetParentSample(parent_sample));
    let set_child_sample = builder_dispatch.apply_callback(|child_sample: Option<NestedSample>| DerivedSampleActions::SetChildSample(child_sample));
    html! {
        <BasicForm<NewDerivedSample>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
if let Some(parent_sample) = builder_store.parent_sample.as_ref() {
    <span>{"TODO Selected parent_sample"}</span>
} else {
    <></>
}
if let Some(child_sample) = builder_store.child_sample.as_ref() {
    <span>{"TODO Selected child_sample"}</span>
} else {
    <></>
}
        </BasicForm<NewDerivedSample>>
    }
}
#[derive(Store, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct ObservationBuilder {
    pub id: Option<Uuid>,
    pub notes: Option<String>,
    pub picture: Option<Vec<u8>>,
    pub project: Option<NestedProject>,
    pub individual: Option<NestedSampledIndividual>,
    pub errors_notes: Vec<ApiError>,
    pub errors_picture: Vec<ApiError>,
    pub errors_project: Vec<ApiError>,
    pub errors_individual: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

impl Default for ObservationBuilder {
    fn default() -> Self {
        Self {
            id: None,
            notes: None,
            picture: None,
            project: Default::default(),
            individual: Default::default(),
            errors_notes: Default::default(),
            errors_picture: Default::default(),
            errors_project: Default::default(),
            errors_individual: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum ObservationActions {
    SetNotes(Option<String>),
    SetPicture(Option<Vec<u8>>),
    SetProject(Option<NestedProject>),
    SetIndividual(Option<NestedSampledIndividual>),
}

impl FromOperation for ObservationActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "project" => ObservationActions::SetProject(Some(bincode::deserialize(&row).unwrap())),
            "individual" => ObservationActions::SetIndividual(Some(bincode::deserialize(&row).unwrap())),
            operation_name => unreachable!("The operation name '{}' is not supported.", operation_name),
        }
    }
}

impl Reducer<ObservationBuilder> for ObservationActions {
    fn apply(self, mut state: std::rc::Rc<ObservationBuilder>) -> std::rc::Rc<ObservationBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            ObservationActions::SetNotes(notes) => 'notes: {
                state_mut.errors_notes.clear();
                if let Some(value) = notes.as_ref() {
                    if value.is_empty() {
                        state_mut.errors_notes.push(ApiError::BadRequest(vec![
                            "The Notes field cannot be left empty.".to_string()
                        ]));
                         state_mut.notes = None;
                          break 'notes;
                    }
                }
                state_mut.notes = notes;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'notes;
            }
            ObservationActions::SetPicture(picture) => 'picture: {
                state_mut.errors_picture.clear();
        if picture.is_none() {
            state_mut.errors_picture.push(ApiError::BadRequest(vec![
                "The Picture field is required.".to_string()
             ]));
            state_mut.picture = None;
             break 'picture;
        }
                state_mut.picture = picture;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'picture;
            }
            ObservationActions::SetProject(project) => 'project: {
                state_mut.errors_project.clear();
        if project.is_none() {
            state_mut.errors_project.push(ApiError::BadRequest(vec![
                "The Project field is required.".to_string()
             ]));
            state_mut.project = None;
             break 'project;
        }
                state_mut.project = project;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'project;
            }
            ObservationActions::SetIndividual(individual) => 'individual: {
                state_mut.errors_individual.clear();
                state_mut.individual = individual;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'individual;
            }
        }
        state
    }
}
impl FormBuilder for ObservationBuilder {
    type Actions = ObservationActions;

    type RichVariant = NestedObservation;

    fn has_errors(&self) -> bool {
!self.errors_notes.is_empty() || !self.errors_picture.is_empty() || !self.errors_project.is_empty() || !self.errors_individual.is_empty()
    }

    fn update(dispatcher: &Dispatch<Self>, richest_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.reduce_mut(|state| {state.id = Some(richest_variant.inner.id);});
    dispatcher.apply(ObservationActions::SetNotes(richest_variant.inner.notes.map(|notes| notes.to_string())));
        dispatcher.apply(ObservationActions::SetPicture(Some(richest_variant.inner.picture)));        dispatcher.apply(ObservationActions::SetProject(Some(richest_variant.project)));
        dispatcher.apply(ObservationActions::SetIndividual(richest_variant.individual));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.picture.is_some()
        && self.project.is_some()
    }

}

impl From<ObservationBuilder> for NewObservation {
    fn from(builder: ObservationBuilder) -> Self {
        Self {
            id: builder.id.unwrap_or_else(Uuid::new_v4),
            project_id: builder.project.unwrap().inner.id,
            individual_id: builder.individual.map(|individual| individual.inner.id),
            notes: builder.notes,
            picture: builder.picture.unwrap(),
        }
    }
}
impl FormBuildable for NewObservation {
    type Builder = ObservationBuilder;
    fn title() -> &'static str {
        "Observation"
    }
    fn task_target() -> &'static str {
        "Observation"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        true
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateObservationFormProp {
     #[prop_or_default]
    pub project_id: Option<i32>,
     #[prop_or_default]
    pub individual_id: Option<Uuid>,
}

#[function_component(CreateObservationForm)]
pub fn create_observation_form(props: &CreateObservationFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<ObservationBuilder>();
   if let Some(project_id) = props.project_id {
         named_requests.push(ComponentMessage::get_named::<&str, Project>("project", project_id.into()));
    }
   if let Some(individual_id) = props.individual_id {
         named_requests.push(ComponentMessage::get_named::<&str, SampledIndividual>("individual", individual_id.into()));
    }
    let set_notes = builder_dispatch.apply_callback(|notes: Option<String>| ObservationActions::SetNotes(notes));
    let set_picture = builder_dispatch.apply_callback(|picture: Option<Image>| ObservationActions::SetPicture(picture.map(|picture| picture.into())));
    let set_project = builder_dispatch.apply_callback(|project: Option<NestedProject>| ObservationActions::SetProject(project));
    let set_individual = builder_dispatch.apply_callback(|individual: Option<NestedSampledIndividual>| ObservationActions::SetIndividual(individual));
    html! {
        <BasicForm<NewObservation>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="Notes" optional={true} errors={builder_store.errors_notes.clone()} builder={set_notes} value={builder_store.notes.clone()} />
            <FileInput<Image> label="Picture" optional={false} errors={builder_store.errors_picture.clone()} builder={set_picture} allowed_formats={vec![GenericFileFormat::Image]} value={builder_store.picture.clone().map(|picture| picture.into())} />
            <Datalist<NestedProject, true> builder={set_project} optional={false} errors={builder_store.errors_project.clone()} value={builder_store.project.clone()} label="Project" />
            <Datalist<NestedSampledIndividual, false> builder={set_individual} optional={true} errors={builder_store.errors_individual.clone()} value={builder_store.individual.clone()} label="Individual" />
        </BasicForm<NewObservation>>
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct UpdateObservationFormProp {
    pub id: Uuid,
}

#[function_component(UpdateObservationForm)]
pub fn update_observation_form(props: &UpdateObservationFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<ObservationBuilder>();
    // We push the ID of the row to the named requests.
    let props = props.clone();
   named_requests.push(ComponentMessage::get::<NewObservation>(props.id.into()));
    let set_notes = builder_dispatch.apply_callback(|notes: Option<String>| ObservationActions::SetNotes(notes));
    let set_picture = builder_dispatch.apply_callback(|picture: Option<Image>| ObservationActions::SetPicture(picture.map(|picture| picture.into())));
    let set_project = builder_dispatch.apply_callback(|project: Option<NestedProject>| ObservationActions::SetProject(project));
    let set_individual = builder_dispatch.apply_callback(|individual: Option<NestedSampledIndividual>| ObservationActions::SetIndividual(individual));
    html! {
        <BasicForm<NewObservation>
            method={FormMethod::PUT}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="Notes" optional={true} errors={builder_store.errors_notes.clone()} builder={set_notes} value={builder_store.notes.clone()} />
            <FileInput<Image> label="Picture" optional={false} errors={builder_store.errors_picture.clone()} builder={set_picture} allowed_formats={vec![GenericFileFormat::Image]} value={builder_store.picture.clone().map(|picture| picture.into())} />
            <Datalist<NestedProject, true> builder={set_project} optional={false} errors={builder_store.errors_project.clone()} value={builder_store.project.clone()} label="Project" />
            <Datalist<NestedSampledIndividual, false> builder={set_individual} optional={true} errors={builder_store.errors_individual.clone()} value={builder_store.individual.clone()} label="Individual" />
        </BasicForm<NewObservation>>
    }
}
#[derive(Store, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct ProjectBuilder {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub public: Option<bool>,
    pub budget: Option<f64>,
    pub expenses: Option<f64>,
    pub expected_end_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub state: Option<NestedProjectState>,
    pub icon: Option<FontAwesomeIcon>,
    pub color: Option<Color>,
    pub parent_project: Option<NestedProject>,
    pub errors_name: Vec<ApiError>,
    pub errors_description: Vec<ApiError>,
    pub errors_public: Vec<ApiError>,
    pub errors_budget: Vec<ApiError>,
    pub errors_expenses: Vec<ApiError>,
    pub errors_expected_end_date: Vec<ApiError>,
    pub errors_end_date: Vec<ApiError>,
    pub errors_state: Vec<ApiError>,
    pub errors_icon: Vec<ApiError>,
    pub errors_color: Vec<ApiError>,
    pub errors_parent_project: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

impl Default for ProjectBuilder {
    fn default() -> Self {
        Self {
            id: None,
            name: None,
            description: None,
            public: Some(true),
            budget: None,
            expenses: None,
            expected_end_date: None,
            end_date: None,
            state: Default::default(),
            icon: Default::default(),
            color: Default::default(),
            parent_project: Default::default(),
            errors_name: Default::default(),
            errors_description: Default::default(),
            errors_public: Default::default(),
            errors_budget: Default::default(),
            errors_expenses: Default::default(),
            errors_expected_end_date: Default::default(),
            errors_end_date: Default::default(),
            errors_state: Default::default(),
            errors_icon: Default::default(),
            errors_color: Default::default(),
            errors_parent_project: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum ProjectActions {
    SetName(Option<String>),
    SetDescription(Option<String>),
    SetPublic(Option<bool>),
    SetBudget(Option<String>),
    SetExpenses(Option<String>),
    SetExpectedEndDate(Option<String>),
    SetEndDate(Option<String>),
    SetState(Option<NestedProjectState>),
    SetIcon(Option<FontAwesomeIcon>),
    SetColor(Option<Color>),
    SetParentProject(Option<NestedProject>),
}

impl FromOperation for ProjectActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "state" => ProjectActions::SetState(Some(bincode::deserialize(&row).unwrap())),
            "icon" => ProjectActions::SetIcon(Some(bincode::deserialize(&row).unwrap())),
            "color" => ProjectActions::SetColor(Some(bincode::deserialize(&row).unwrap())),
            "parent_project" => ProjectActions::SetParentProject(Some(bincode::deserialize(&row).unwrap())),
            operation_name => unreachable!("The operation name '{}' is not supported.", operation_name),
        }
    }
}

impl Reducer<ProjectBuilder> for ProjectActions {
    fn apply(self, mut state: std::rc::Rc<ProjectBuilder>) -> std::rc::Rc<ProjectBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            ProjectActions::SetName(name) => 'name: {
                state_mut.errors_name.clear();
        if name.is_none() {
            state_mut.errors_name.push(ApiError::BadRequest(vec![
                "The Name field is required.".to_string()
             ]));
            state_mut.name = None;
             break 'name;
        }
                if let Some(value) = name.as_ref() {
                    if value.is_empty() {
                        state_mut.errors_name.push(ApiError::BadRequest(vec![
                            "The Name field cannot be left empty.".to_string()
                        ]));
                         state_mut.name = None;
                          break 'name;
                    }
                }
                state_mut.name = name;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'name;
            }
            ProjectActions::SetDescription(description) => 'description: {
                state_mut.errors_description.clear();
        if description.is_none() {
            state_mut.errors_description.push(ApiError::BadRequest(vec![
                "The Description field is required.".to_string()
             ]));
            state_mut.description = None;
             break 'description;
        }
                if let Some(value) = description.as_ref() {
                    if value.is_empty() {
                        state_mut.errors_description.push(ApiError::BadRequest(vec![
                            "The Description field cannot be left empty.".to_string()
                        ]));
                         state_mut.description = None;
                          break 'description;
                    }
                }
                state_mut.description = description;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'description;
            }
            ProjectActions::SetPublic(public) => 'public: {
                state_mut.errors_public.clear();
        if public.is_none() {
            state_mut.errors_public.push(ApiError::BadRequest(vec![
                "The Public field is required.".to_string()
             ]));
            state_mut.public = None;
             break 'public;
        }
                state_mut.public = public;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'public;
            }
            ProjectActions::SetBudget(budget) => 'budget: {
                state_mut.errors_budget.clear();
                state_mut.form_updated_at = chrono::Utc::now().naive_utc();
                match budget {
                    Some(value) => match value.parse::<f64>() {
                        Ok(value) => {
                            if value.is_nan() || value.is_infinite() {
                                state_mut.errors_budget.push(ApiError::BadRequest(vec![
                                    "The budget field must be a valid f64.".to_string()
                                ]));
                            } else                             if value < f64::MIN as f64 || value > f64::MAX as f64 {
                                state_mut.errors_budget.push(ApiError::BadRequest(vec![
                                    format!(                                            "The budget field must be between {} and {}.",
                                            f64::MIN,
                                            f64::MAX
                                    )
                                ]));
                            } else {
                                state_mut.budget = Some(value as f64);
                            }
                        }
                        Err(_) => {
                            state_mut.errors_budget.push(ApiError::BadRequest(vec![
                                "The budget field must be a valid f64.".to_string()
                            ]));
                        }
                    },
                    None => state_mut.budget = None,
                }
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'budget;
            }
            ProjectActions::SetExpenses(expenses) => 'expenses: {
                state_mut.errors_expenses.clear();
                state_mut.form_updated_at = chrono::Utc::now().naive_utc();
                match expenses {
                    Some(value) => match value.parse::<f64>() {
                        Ok(value) => {
                            if value.is_nan() || value.is_infinite() {
                                state_mut.errors_expenses.push(ApiError::BadRequest(vec![
                                    "The expenses field must be a valid f64.".to_string()
                                ]));
                            } else                             if value < f64::MIN as f64 || value > f64::MAX as f64 {
                                state_mut.errors_expenses.push(ApiError::BadRequest(vec![
                                    format!(                                            "The expenses field must be between {} and {}.",
                                            f64::MIN,
                                            f64::MAX
                                    )
                                ]));
                            } else {
                                state_mut.expenses = Some(value as f64);
                            }
                        }
                        Err(_) => {
                            state_mut.errors_expenses.push(ApiError::BadRequest(vec![
                                "The expenses field must be a valid f64.".to_string()
                            ]));
                        }
                    },
                    None => state_mut.expenses = None,
                }
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'expenses;
            }
            ProjectActions::SetExpectedEndDate(expected_end_date) => 'expected_end_date: {
                state_mut.errors_expected_end_date.clear();
                match expected_end_date {
                    Some(value) => match NaiveDateTime::parse_from_str(&value, "%Y-%m-%dT%H:%M") {
                        Ok(expected_end_date) => state_mut.expected_end_date = Some(expected_end_date),
                        Err(_) => state_mut.errors_expected_end_date.push(ApiError::BadRequest(vec![
                            "The expected_end_date field must be a valid date and time.".to_string()
                        ])),
                    },
                    None => state_mut.expected_end_date = None,
                }
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'expected_end_date;
            }
            ProjectActions::SetEndDate(end_date) => 'end_date: {
                state_mut.errors_end_date.clear();
                match end_date {
                    Some(value) => match NaiveDateTime::parse_from_str(&value, "%Y-%m-%dT%H:%M") {
                        Ok(end_date) => state_mut.end_date = Some(end_date),
                        Err(_) => state_mut.errors_end_date.push(ApiError::BadRequest(vec![
                            "The end_date field must be a valid date and time.".to_string()
                        ])),
                    },
                    None => state_mut.end_date = None,
                }
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'end_date;
            }
            ProjectActions::SetState(state) => 'state: {
                state_mut.errors_state.clear();
        if state.is_none() {
            state_mut.errors_state.push(ApiError::BadRequest(vec![
                "The State field is required.".to_string()
             ]));
            state_mut.state = None;
             break 'state;
        }
                state_mut.state = state;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'state;
            }
            ProjectActions::SetIcon(icon) => 'icon: {
                state_mut.errors_icon.clear();
        if icon.is_none() {
            state_mut.errors_icon.push(ApiError::BadRequest(vec![
                "The Icon field is required.".to_string()
             ]));
            state_mut.icon = None;
             break 'icon;
        }
                state_mut.icon = icon;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'icon;
            }
            ProjectActions::SetColor(color) => 'color: {
                state_mut.errors_color.clear();
        if color.is_none() {
            state_mut.errors_color.push(ApiError::BadRequest(vec![
                "The Color field is required.".to_string()
             ]));
            state_mut.color = None;
             break 'color;
        }
                state_mut.color = color;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'color;
            }
            ProjectActions::SetParentProject(parent_project) => 'parent_project: {
                state_mut.errors_parent_project.clear();
                match parent_project.as_ref() {
                    Some(parent_project) => {
                            if state_mut.id.map_or(false, |id| id == parent_project.inner.id)
                        {
                            state_mut.errors_parent_project.push(ApiError::BadRequest(vec![
                                "The Parent project field must be distinct from the current value.".to_string()
                             ]));
                            break 'parent_project;
                        }
                    }
                    None => (),
                }
                state_mut.parent_project = parent_project;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'parent_project;
            }
        }
        state
    }
}
impl FormBuilder for ProjectBuilder {
    type Actions = ProjectActions;

    type RichVariant = NestedProject;

    fn has_errors(&self) -> bool {
!self.errors_name.is_empty() || !self.errors_description.is_empty() || !self.errors_public.is_empty() || !self.errors_budget.is_empty() || !self.errors_expenses.is_empty() || !self.errors_expected_end_date.is_empty() || !self.errors_end_date.is_empty() || !self.errors_state.is_empty() || !self.errors_icon.is_empty() || !self.errors_color.is_empty() || !self.errors_parent_project.is_empty()
    }

    fn update(dispatcher: &Dispatch<Self>, richest_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.reduce_mut(|state| {state.id = Some(richest_variant.inner.id);});
    dispatcher.apply(ProjectActions::SetName(Some(richest_variant.inner.name.to_string())));
    dispatcher.apply(ProjectActions::SetDescription(Some(richest_variant.inner.description.to_string())));
        dispatcher.apply(ProjectActions::SetPublic(Some(richest_variant.inner.public)));    dispatcher.apply(ProjectActions::SetBudget(richest_variant.inner.budget.map(|budget| budget.to_string())));
    dispatcher.apply(ProjectActions::SetExpenses(richest_variant.inner.expenses.map(|expenses| expenses.to_string())));
    dispatcher.apply(ProjectActions::SetExpectedEndDate(richest_variant.inner.expected_end_date.map(|expected_end_date| expected_end_date.to_string())));
    dispatcher.apply(ProjectActions::SetEndDate(richest_variant.inner.end_date.map(|end_date| end_date.to_string())));
        dispatcher.apply(ProjectActions::SetState(Some(richest_variant.state)));
        dispatcher.apply(ProjectActions::SetIcon(Some(richest_variant.icon)));
        dispatcher.apply(ProjectActions::SetColor(Some(richest_variant.color)));
        let mut named_requests = Vec::new();
        if let Some(parent_project_id) = richest_variant.inner.parent_project_id {
    named_requests.push(ComponentMessage::get_named::<&str, Project>("parent_project", parent_project_id.into()));
 } else {
    dispatcher.apply(ProjectActions::SetParentProject(None));
 }
        named_requests
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.name.is_some()
        && self.description.is_some()
        && self.public.is_some()
        && self.state.is_some()
        && self.icon.is_some()
        && self.color.is_some()
    }

}

impl From<ProjectBuilder> for NewProject {
    fn from(builder: ProjectBuilder) -> Self {
        Self {
            name: builder.name.unwrap(),
            description: builder.description.unwrap(),
            public: builder.public.unwrap(),
            state_id: builder.state.unwrap().inner.id,
            icon_id: builder.icon.unwrap().id,
            color_id: builder.color.unwrap().id,
            parent_project_id: builder.parent_project.map(|parent_project| parent_project.inner.id),
            budget: builder.budget,
            expenses: builder.expenses,
            expected_end_date: builder.expected_end_date,
            end_date: builder.end_date,
        }
    }
}
impl From<ProjectBuilder> for UpdateProject {
    fn from(builder: ProjectBuilder) -> Self {
        Self {
            id: builder.id.unwrap(),
            name: builder.name.unwrap(),
            description: builder.description.unwrap(),
            public: builder.public.unwrap(),
            state_id: builder.state.unwrap().inner.id,
            icon_id: builder.icon.unwrap().id,
            color_id: builder.color.unwrap().id,
            parent_project_id: builder.parent_project.map(|parent_project| parent_project.inner.id),
            budget: builder.budget,
            expenses: builder.expenses,
            expected_end_date: builder.expected_end_date,
            end_date: builder.end_date,
        }
    }
}
impl FormBuildable for NewProject {
    type Builder = ProjectBuilder;
    fn title() -> &'static str {
        "Project"
    }
    fn task_target() -> &'static str {
        "Project"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

impl FormBuildable for UpdateProject {
    type Builder = ProjectBuilder;
    fn title() -> &'static str {
        "Project"
    }
    fn task_target() -> &'static str {
        "Project"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        true
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateProjectFormProp {
     #[prop_or(1)]
     pub state_id: i32,
     #[prop_or(415)]
     pub icon_id: i32,
     #[prop_or(1)]
     pub color_id: i32,
     #[prop_or_default]
    pub parent_project_id: Option<i32>,
}

#[function_component(CreateProjectForm)]
pub fn create_project_form(props: &CreateProjectFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<ProjectBuilder>();
    named_requests.push(ComponentMessage::get_named::<&str, ProjectState>("state", props.state_id.into()));
    named_requests.push(ComponentMessage::get_named::<&str, FontAwesomeIcon>("icon", props.icon_id.into()));
    named_requests.push(ComponentMessage::get_named::<&str, Color>("color", props.color_id.into()));
   if let Some(parent_project_id) = props.parent_project_id {
         named_requests.push(ComponentMessage::get_named::<&str, Project>("parent_project", parent_project_id.into()));
    }
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| ProjectActions::SetName(name));
    let set_description = builder_dispatch.apply_callback(|description: Option<String>| ProjectActions::SetDescription(description));
    let set_public = builder_dispatch.apply_callback(|public: bool| ProjectActions::SetPublic(Some(public)));
    let set_budget = builder_dispatch.apply_callback(|budget: Option<String>| ProjectActions::SetBudget(budget));
    let set_expenses = builder_dispatch.apply_callback(|expenses: Option<String>| ProjectActions::SetExpenses(expenses));
    let set_expected_end_date = builder_dispatch.apply_callback(|expected_end_date: Option<String>| ProjectActions::SetExpectedEndDate(expected_end_date));
    let set_end_date = builder_dispatch.apply_callback(|end_date: Option<String>| ProjectActions::SetEndDate(end_date));
    let set_state = builder_dispatch.apply_callback(|state: Option<NestedProjectState>| ProjectActions::SetState(state));
    let set_icon = builder_dispatch.apply_callback(|icon: Option<FontAwesomeIcon>| ProjectActions::SetIcon(icon));
    let set_color = builder_dispatch.apply_callback(|color: Option<Color>| ProjectActions::SetColor(color));
    let set_parent_project = builder_dispatch.apply_callback(|parent_project: Option<NestedProject>| ProjectActions::SetParentProject(parent_project));
    html! {
        <BasicForm<NewProject>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="Name" optional={false} errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} />
            <BasicInput<String> label="Description" optional={false} errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} />
            <Checkbox label="Public" errors={builder_store.errors_public.clone()} builder={set_public} value={builder_store.public.unwrap_or(false)} />
            <BasicInput<f64> label="Budget" optional={true} errors={builder_store.errors_budget.clone()} builder={set_budget} value={builder_store.budget.clone()} />
            <BasicInput<f64> label="Expenses" optional={true} errors={builder_store.errors_expenses.clone()} builder={set_expenses} value={builder_store.expenses.clone()} />
            <BasicInput<NaiveDateTime> label="Expected end date" optional={true} errors={builder_store.errors_expected_end_date.clone()} builder={set_expected_end_date} value={builder_store.expected_end_date.clone()} />
            <BasicInput<NaiveDateTime> label="End date" optional={true} errors={builder_store.errors_end_date.clone()} builder={set_end_date} value={builder_store.end_date.clone()} />
            <Datalist<NestedProjectState, false> builder={set_state} optional={false} errors={builder_store.errors_state.clone()} value={builder_store.state.clone()} label="State" />
            <Datalist<FontAwesomeIcon, false> builder={set_icon} optional={false} errors={builder_store.errors_icon.clone()} value={builder_store.icon.clone()} label="Icon" />
            <Datalist<Color, false> builder={set_color} optional={false} errors={builder_store.errors_color.clone()} value={builder_store.color.clone()} label="Color" />
            <Datalist<NestedProject, true> builder={set_parent_project} optional={true} errors={builder_store.errors_parent_project.clone()} value={builder_store.parent_project.clone()} label="Parent project" />
        </BasicForm<NewProject>>
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct UpdateProjectFormProp {
    pub id: i32,
}

#[function_component(UpdateProjectForm)]
pub fn update_project_form(props: &UpdateProjectFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<ProjectBuilder>();
    // We push the ID of the row to the named requests.
    let props = props.clone();
   named_requests.push(ComponentMessage::get::<UpdateProject>(props.id.into()));
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| ProjectActions::SetName(name));
    let set_description = builder_dispatch.apply_callback(|description: Option<String>| ProjectActions::SetDescription(description));
    let set_public = builder_dispatch.apply_callback(|public: bool| ProjectActions::SetPublic(Some(public)));
    let set_budget = builder_dispatch.apply_callback(|budget: Option<String>| ProjectActions::SetBudget(budget));
    let set_expenses = builder_dispatch.apply_callback(|expenses: Option<String>| ProjectActions::SetExpenses(expenses));
    let set_expected_end_date = builder_dispatch.apply_callback(|expected_end_date: Option<String>| ProjectActions::SetExpectedEndDate(expected_end_date));
    let set_end_date = builder_dispatch.apply_callback(|end_date: Option<String>| ProjectActions::SetEndDate(end_date));
    let set_state = builder_dispatch.apply_callback(|state: Option<NestedProjectState>| ProjectActions::SetState(state));
    let set_icon = builder_dispatch.apply_callback(|icon: Option<FontAwesomeIcon>| ProjectActions::SetIcon(icon));
    let set_color = builder_dispatch.apply_callback(|color: Option<Color>| ProjectActions::SetColor(color));
    let set_parent_project = builder_dispatch.apply_callback(|parent_project: Option<NestedProject>| ProjectActions::SetParentProject(parent_project));
    html! {
        <BasicForm<UpdateProject>
            method={FormMethod::PUT}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="Name" optional={false} errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} />
            <BasicInput<String> label="Description" optional={false} errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} />
            <Checkbox label="Public" errors={builder_store.errors_public.clone()} builder={set_public} value={builder_store.public.unwrap_or(false)} />
            <BasicInput<f64> label="Budget" optional={true} errors={builder_store.errors_budget.clone()} builder={set_budget} value={builder_store.budget.clone()} />
            <BasicInput<f64> label="Expenses" optional={true} errors={builder_store.errors_expenses.clone()} builder={set_expenses} value={builder_store.expenses.clone()} />
            <BasicInput<NaiveDateTime> label="Expected end date" optional={true} errors={builder_store.errors_expected_end_date.clone()} builder={set_expected_end_date} value={builder_store.expected_end_date.clone()} />
            <BasicInput<NaiveDateTime> label="End date" optional={true} errors={builder_store.errors_end_date.clone()} builder={set_end_date} value={builder_store.end_date.clone()} />
            <Datalist<NestedProjectState, false> builder={set_state} optional={false} errors={builder_store.errors_state.clone()} value={builder_store.state.clone()} label="State" />
            <Datalist<FontAwesomeIcon, false> builder={set_icon} optional={false} errors={builder_store.errors_icon.clone()} value={builder_store.icon.clone()} label="Icon" />
            <Datalist<Color, false> builder={set_color} optional={false} errors={builder_store.errors_color.clone()} value={builder_store.color.clone()} label="Color" />
            <Datalist<NestedProject, true> builder={set_parent_project} optional={true} errors={builder_store.errors_parent_project.clone()} value={builder_store.parent_project.clone()} label="Parent project" />
        </BasicForm<UpdateProject>>
    }
}
#[derive(Store, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct ProjectsTeamsRoleInvitationBuilder {
    pub table: Option<NestedProject>,
    pub team: Option<NestedTeam>,
    pub role: Option<NestedRole>,
    pub errors_table: Vec<ApiError>,
    pub errors_team: Vec<ApiError>,
    pub errors_role: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

impl Default for ProjectsTeamsRoleInvitationBuilder {
    fn default() -> Self {
        Self {
            table: Default::default(),
            team: Default::default(),
            role: Default::default(),
            errors_table: Default::default(),
            errors_team: Default::default(),
            errors_role: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum ProjectsTeamsRoleInvitationActions {
    SetTable(Option<NestedProject>),
    SetTeam(Option<NestedTeam>),
    SetRole(Option<NestedRole>),
}

impl FromOperation for ProjectsTeamsRoleInvitationActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "table" => ProjectsTeamsRoleInvitationActions::SetTable(Some(bincode::deserialize(&row).unwrap())),
            "team" => ProjectsTeamsRoleInvitationActions::SetTeam(Some(bincode::deserialize(&row).unwrap())),
            "role" => ProjectsTeamsRoleInvitationActions::SetRole(Some(bincode::deserialize(&row).unwrap())),
            operation_name => unreachable!("The operation name '{}' is not supported.", operation_name),
        }
    }
}

impl Reducer<ProjectsTeamsRoleInvitationBuilder> for ProjectsTeamsRoleInvitationActions {
    fn apply(self, mut state: std::rc::Rc<ProjectsTeamsRoleInvitationBuilder>) -> std::rc::Rc<ProjectsTeamsRoleInvitationBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            ProjectsTeamsRoleInvitationActions::SetTable(table) => 'table: {
                state_mut.errors_table.clear();
        if table.is_none() {
            state_mut.errors_table.push(ApiError::BadRequest(vec![
                "The Table field is required.".to_string()
             ]));
            state_mut.table = None;
             break 'table;
        }
                state_mut.table = table;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'table;
            }
            ProjectsTeamsRoleInvitationActions::SetTeam(team) => 'team: {
                state_mut.errors_team.clear();
        if team.is_none() {
            state_mut.errors_team.push(ApiError::BadRequest(vec![
                "The Team field is required.".to_string()
             ]));
            state_mut.team = None;
             break 'team;
        }
                state_mut.team = team;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'team;
            }
            ProjectsTeamsRoleInvitationActions::SetRole(role) => 'role: {
                state_mut.errors_role.clear();
        if role.is_none() {
            state_mut.errors_role.push(ApiError::BadRequest(vec![
                "The Role field is required.".to_string()
             ]));
            state_mut.role = None;
             break 'role;
        }
                state_mut.role = role;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'role;
            }
        }
        state
    }
}
impl FormBuilder for ProjectsTeamsRoleInvitationBuilder {
    type Actions = ProjectsTeamsRoleInvitationActions;

    type RichVariant = NestedProjectsTeamsRoleInvitation;

    fn has_errors(&self) -> bool {
!self.errors_table.is_empty() || !self.errors_team.is_empty() || !self.errors_role.is_empty()
    }

    fn update(dispatcher: &Dispatch<Self>, richest_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.apply(ProjectsTeamsRoleInvitationActions::SetTable(Some(richest_variant.table)));
        dispatcher.apply(ProjectsTeamsRoleInvitationActions::SetTeam(Some(richest_variant.team)));
        dispatcher.apply(ProjectsTeamsRoleInvitationActions::SetRole(Some(richest_variant.role)));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.table.is_some()
        && self.team.is_some()
        && self.role.is_some()
    }

}

impl From<ProjectsTeamsRoleInvitationBuilder> for NewProjectsTeamsRoleInvitation {
    fn from(builder: ProjectsTeamsRoleInvitationBuilder) -> Self {
        Self {
            table_id: builder.table.as_ref().map(|table| table.inner.id).unwrap(),
            team_id: builder.team.as_ref().map(|team| team.inner.id).unwrap(),
            role_id: builder.role.unwrap().inner.id,
        }
    }
}
impl FormBuildable for NewProjectsTeamsRoleInvitation {
    type Builder = ProjectsTeamsRoleInvitationBuilder;
    fn title() -> &'static str {
        "Projects teams role invitation"
    }
    fn task_target() -> &'static str {
        "Projects teams role invitation"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateProjectsTeamsRoleInvitationFormProp {
     #[prop_or_default]
    pub table_id: Option<i32>,
     #[prop_or_default]
    pub team_id: Option<i32>,
     #[prop_or_default]
    pub role_id: Option<i32>,
}

#[function_component(CreateProjectsTeamsRoleInvitationForm)]
pub fn create_projects_teams_role_invitation_form(props: &CreateProjectsTeamsRoleInvitationFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<ProjectsTeamsRoleInvitationBuilder>();
   if let Some(table_id) = props.table_id {
         named_requests.push(ComponentMessage::get_named::<&str, Project>("table", table_id.into()));
    }
   if let Some(team_id) = props.team_id {
         named_requests.push(ComponentMessage::get_named::<&str, Team>("team", team_id.into()));
    }
   if let Some(role_id) = props.role_id {
         named_requests.push(ComponentMessage::get_named::<&str, Role>("role", role_id.into()));
    }
    let set_table = builder_dispatch.apply_callback(|table: Option<NestedProject>| ProjectsTeamsRoleInvitationActions::SetTable(table));
    let set_team = builder_dispatch.apply_callback(|team: Option<NestedTeam>| ProjectsTeamsRoleInvitationActions::SetTeam(team));
    let set_role = builder_dispatch.apply_callback(|role: Option<NestedRole>| ProjectsTeamsRoleInvitationActions::SetRole(role));
    html! {
        <BasicForm<NewProjectsTeamsRoleInvitation>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <Datalist<NestedProject, true> builder={set_table} optional={false} errors={builder_store.errors_table.clone()} value={builder_store.table.clone()} label="Table" />
            <Datalist<NestedTeam, true> builder={set_team} optional={false} errors={builder_store.errors_team.clone()} value={builder_store.team.clone()} label="Team" />
            <Datalist<NestedRole, false> builder={set_role} optional={false} errors={builder_store.errors_role.clone()} value={builder_store.role.clone()} label="Role" />
        </BasicForm<NewProjectsTeamsRoleInvitation>>
    }
}
#[derive(Store, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct ProjectsTeamsRoleRequestBuilder {
    pub table: Option<NestedProject>,
    pub team: Option<NestedTeam>,
    pub role: Option<NestedRole>,
    pub errors_table: Vec<ApiError>,
    pub errors_team: Vec<ApiError>,
    pub errors_role: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

impl Default for ProjectsTeamsRoleRequestBuilder {
    fn default() -> Self {
        Self {
            table: Default::default(),
            team: Default::default(),
            role: Default::default(),
            errors_table: Default::default(),
            errors_team: Default::default(),
            errors_role: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum ProjectsTeamsRoleRequestActions {
    SetTable(Option<NestedProject>),
    SetTeam(Option<NestedTeam>),
    SetRole(Option<NestedRole>),
}

impl FromOperation for ProjectsTeamsRoleRequestActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "table" => ProjectsTeamsRoleRequestActions::SetTable(Some(bincode::deserialize(&row).unwrap())),
            "team" => ProjectsTeamsRoleRequestActions::SetTeam(Some(bincode::deserialize(&row).unwrap())),
            "role" => ProjectsTeamsRoleRequestActions::SetRole(Some(bincode::deserialize(&row).unwrap())),
            operation_name => unreachable!("The operation name '{}' is not supported.", operation_name),
        }
    }
}

impl Reducer<ProjectsTeamsRoleRequestBuilder> for ProjectsTeamsRoleRequestActions {
    fn apply(self, mut state: std::rc::Rc<ProjectsTeamsRoleRequestBuilder>) -> std::rc::Rc<ProjectsTeamsRoleRequestBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            ProjectsTeamsRoleRequestActions::SetTable(table) => 'table: {
                state_mut.errors_table.clear();
        if table.is_none() {
            state_mut.errors_table.push(ApiError::BadRequest(vec![
                "The Table field is required.".to_string()
             ]));
            state_mut.table = None;
             break 'table;
        }
                state_mut.table = table;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'table;
            }
            ProjectsTeamsRoleRequestActions::SetTeam(team) => 'team: {
                state_mut.errors_team.clear();
        if team.is_none() {
            state_mut.errors_team.push(ApiError::BadRequest(vec![
                "The Team field is required.".to_string()
             ]));
            state_mut.team = None;
             break 'team;
        }
                state_mut.team = team;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'team;
            }
            ProjectsTeamsRoleRequestActions::SetRole(role) => 'role: {
                state_mut.errors_role.clear();
        if role.is_none() {
            state_mut.errors_role.push(ApiError::BadRequest(vec![
                "The Role field is required.".to_string()
             ]));
            state_mut.role = None;
             break 'role;
        }
                state_mut.role = role;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'role;
            }
        }
        state
    }
}
impl FormBuilder for ProjectsTeamsRoleRequestBuilder {
    type Actions = ProjectsTeamsRoleRequestActions;

    type RichVariant = NestedProjectsTeamsRoleRequest;

    fn has_errors(&self) -> bool {
!self.errors_table.is_empty() || !self.errors_team.is_empty() || !self.errors_role.is_empty()
    }

    fn update(dispatcher: &Dispatch<Self>, richest_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.apply(ProjectsTeamsRoleRequestActions::SetTable(Some(richest_variant.table)));
        dispatcher.apply(ProjectsTeamsRoleRequestActions::SetTeam(Some(richest_variant.team)));
        dispatcher.apply(ProjectsTeamsRoleRequestActions::SetRole(Some(richest_variant.role)));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.table.is_some()
        && self.team.is_some()
        && self.role.is_some()
    }

}

impl From<ProjectsTeamsRoleRequestBuilder> for NewProjectsTeamsRoleRequest {
    fn from(builder: ProjectsTeamsRoleRequestBuilder) -> Self {
        Self {
            table_id: builder.table.as_ref().map(|table| table.inner.id).unwrap(),
            team_id: builder.team.as_ref().map(|team| team.inner.id).unwrap(),
            role_id: builder.role.unwrap().inner.id,
        }
    }
}
impl FormBuildable for NewProjectsTeamsRoleRequest {
    type Builder = ProjectsTeamsRoleRequestBuilder;
    fn title() -> &'static str {
        "Projects teams role request"
    }
    fn task_target() -> &'static str {
        "Projects teams role request"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateProjectsTeamsRoleRequestFormProp {
     #[prop_or_default]
    pub table_id: Option<i32>,
     #[prop_or_default]
    pub team_id: Option<i32>,
     #[prop_or_default]
    pub role_id: Option<i32>,
}

#[function_component(CreateProjectsTeamsRoleRequestForm)]
pub fn create_projects_teams_role_request_form(props: &CreateProjectsTeamsRoleRequestFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<ProjectsTeamsRoleRequestBuilder>();
   if let Some(table_id) = props.table_id {
         named_requests.push(ComponentMessage::get_named::<&str, Project>("table", table_id.into()));
    }
   if let Some(team_id) = props.team_id {
         named_requests.push(ComponentMessage::get_named::<&str, Team>("team", team_id.into()));
    }
   if let Some(role_id) = props.role_id {
         named_requests.push(ComponentMessage::get_named::<&str, Role>("role", role_id.into()));
    }
    let set_table = builder_dispatch.apply_callback(|table: Option<NestedProject>| ProjectsTeamsRoleRequestActions::SetTable(table));
    let set_team = builder_dispatch.apply_callback(|team: Option<NestedTeam>| ProjectsTeamsRoleRequestActions::SetTeam(team));
    let set_role = builder_dispatch.apply_callback(|role: Option<NestedRole>| ProjectsTeamsRoleRequestActions::SetRole(role));
    html! {
        <BasicForm<NewProjectsTeamsRoleRequest>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <Datalist<NestedProject, true> builder={set_table} optional={false} errors={builder_store.errors_table.clone()} value={builder_store.table.clone()} label="Table" />
            <Datalist<NestedTeam, true> builder={set_team} optional={false} errors={builder_store.errors_team.clone()} value={builder_store.team.clone()} label="Team" />
            <Datalist<NestedRole, false> builder={set_role} optional={false} errors={builder_store.errors_role.clone()} value={builder_store.role.clone()} label="Role" />
        </BasicForm<NewProjectsTeamsRoleRequest>>
    }
}
#[derive(Store, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct ProjectsTeamsRoleBuilder {
    pub table: Option<NestedProject>,
    pub team: Option<NestedTeam>,
    pub role: Option<NestedRole>,
    pub errors_table: Vec<ApiError>,
    pub errors_team: Vec<ApiError>,
    pub errors_role: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

impl Default for ProjectsTeamsRoleBuilder {
    fn default() -> Self {
        Self {
            table: Default::default(),
            team: Default::default(),
            role: Default::default(),
            errors_table: Default::default(),
            errors_team: Default::default(),
            errors_role: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum ProjectsTeamsRoleActions {
    SetTable(Option<NestedProject>),
    SetTeam(Option<NestedTeam>),
    SetRole(Option<NestedRole>),
}

impl FromOperation for ProjectsTeamsRoleActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "table" => ProjectsTeamsRoleActions::SetTable(Some(bincode::deserialize(&row).unwrap())),
            "team" => ProjectsTeamsRoleActions::SetTeam(Some(bincode::deserialize(&row).unwrap())),
            "role" => ProjectsTeamsRoleActions::SetRole(Some(bincode::deserialize(&row).unwrap())),
            operation_name => unreachable!("The operation name '{}' is not supported.", operation_name),
        }
    }
}

impl Reducer<ProjectsTeamsRoleBuilder> for ProjectsTeamsRoleActions {
    fn apply(self, mut state: std::rc::Rc<ProjectsTeamsRoleBuilder>) -> std::rc::Rc<ProjectsTeamsRoleBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            ProjectsTeamsRoleActions::SetTable(table) => 'table: {
                state_mut.errors_table.clear();
        if table.is_none() {
            state_mut.errors_table.push(ApiError::BadRequest(vec![
                "The Table field is required.".to_string()
             ]));
            state_mut.table = None;
             break 'table;
        }
                state_mut.table = table;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'table;
            }
            ProjectsTeamsRoleActions::SetTeam(team) => 'team: {
                state_mut.errors_team.clear();
        if team.is_none() {
            state_mut.errors_team.push(ApiError::BadRequest(vec![
                "The Team field is required.".to_string()
             ]));
            state_mut.team = None;
             break 'team;
        }
                state_mut.team = team;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'team;
            }
            ProjectsTeamsRoleActions::SetRole(role) => 'role: {
                state_mut.errors_role.clear();
        if role.is_none() {
            state_mut.errors_role.push(ApiError::BadRequest(vec![
                "The Role field is required.".to_string()
             ]));
            state_mut.role = None;
             break 'role;
        }
                state_mut.role = role;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'role;
            }
        }
        state
    }
}
impl FormBuilder for ProjectsTeamsRoleBuilder {
    type Actions = ProjectsTeamsRoleActions;

    type RichVariant = NestedProjectsTeamsRole;

    fn has_errors(&self) -> bool {
!self.errors_table.is_empty() || !self.errors_team.is_empty() || !self.errors_role.is_empty()
    }

    fn update(dispatcher: &Dispatch<Self>, richest_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.apply(ProjectsTeamsRoleActions::SetTable(Some(richest_variant.table)));
        dispatcher.apply(ProjectsTeamsRoleActions::SetTeam(Some(richest_variant.team)));
        dispatcher.apply(ProjectsTeamsRoleActions::SetRole(Some(richest_variant.role)));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.table.is_some()
        && self.team.is_some()
        && self.role.is_some()
    }

}

impl From<ProjectsTeamsRoleBuilder> for NewProjectsTeamsRole {
    fn from(builder: ProjectsTeamsRoleBuilder) -> Self {
        Self {
            table_id: builder.table.as_ref().map(|table| table.inner.id).unwrap(),
            team_id: builder.team.as_ref().map(|team| team.inner.id).unwrap(),
            role_id: builder.role.unwrap().inner.id,
        }
    }
}
impl FormBuildable for NewProjectsTeamsRole {
    type Builder = ProjectsTeamsRoleBuilder;
    fn title() -> &'static str {
        "Projects teams role"
    }
    fn task_target() -> &'static str {
        "Projects teams role"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateProjectsTeamsRoleFormProp {
     #[prop_or_default]
    pub table_id: Option<i32>,
     #[prop_or_default]
    pub team_id: Option<i32>,
     #[prop_or_default]
    pub role_id: Option<i32>,
}

#[function_component(CreateProjectsTeamsRoleForm)]
pub fn create_projects_teams_role_form(props: &CreateProjectsTeamsRoleFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<ProjectsTeamsRoleBuilder>();
   if let Some(table_id) = props.table_id {
         named_requests.push(ComponentMessage::get_named::<&str, Project>("table", table_id.into()));
    }
   if let Some(team_id) = props.team_id {
         named_requests.push(ComponentMessage::get_named::<&str, Team>("team", team_id.into()));
    }
   if let Some(role_id) = props.role_id {
         named_requests.push(ComponentMessage::get_named::<&str, Role>("role", role_id.into()));
    }
    let set_table = builder_dispatch.apply_callback(|table: Option<NestedProject>| ProjectsTeamsRoleActions::SetTable(table));
    let set_team = builder_dispatch.apply_callback(|team: Option<NestedTeam>| ProjectsTeamsRoleActions::SetTeam(team));
    let set_role = builder_dispatch.apply_callback(|role: Option<NestedRole>| ProjectsTeamsRoleActions::SetRole(role));
    html! {
        <BasicForm<NewProjectsTeamsRole>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <Datalist<NestedProject, true> builder={set_table} optional={false} errors={builder_store.errors_table.clone()} value={builder_store.table.clone()} label="Table" />
            <Datalist<NestedTeam, true> builder={set_team} optional={false} errors={builder_store.errors_team.clone()} value={builder_store.team.clone()} label="Team" />
            <Datalist<NestedRole, false> builder={set_role} optional={false} errors={builder_store.errors_role.clone()} value={builder_store.role.clone()} label="Role" />
        </BasicForm<NewProjectsTeamsRole>>
    }
}
#[derive(Store, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct ProjectsUsersRoleInvitationBuilder {
    pub table: Option<NestedProject>,
    pub user: Option<User>,
    pub role: Option<NestedRole>,
    pub errors_table: Vec<ApiError>,
    pub errors_user: Vec<ApiError>,
    pub errors_role: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

impl Default for ProjectsUsersRoleInvitationBuilder {
    fn default() -> Self {
        Self {
            table: Default::default(),
            user: Default::default(),
            role: Default::default(),
            errors_table: Default::default(),
            errors_user: Default::default(),
            errors_role: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum ProjectsUsersRoleInvitationActions {
    SetTable(Option<NestedProject>),
    SetUser(Option<User>),
    SetRole(Option<NestedRole>),
}

impl FromOperation for ProjectsUsersRoleInvitationActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "table" => ProjectsUsersRoleInvitationActions::SetTable(Some(bincode::deserialize(&row).unwrap())),
            "user" => ProjectsUsersRoleInvitationActions::SetUser(Some(bincode::deserialize(&row).unwrap())),
            "role" => ProjectsUsersRoleInvitationActions::SetRole(Some(bincode::deserialize(&row).unwrap())),
            operation_name => unreachable!("The operation name '{}' is not supported.", operation_name),
        }
    }
}

impl Reducer<ProjectsUsersRoleInvitationBuilder> for ProjectsUsersRoleInvitationActions {
    fn apply(self, mut state: std::rc::Rc<ProjectsUsersRoleInvitationBuilder>) -> std::rc::Rc<ProjectsUsersRoleInvitationBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            ProjectsUsersRoleInvitationActions::SetTable(table) => 'table: {
                state_mut.errors_table.clear();
        if table.is_none() {
            state_mut.errors_table.push(ApiError::BadRequest(vec![
                "The Table field is required.".to_string()
             ]));
            state_mut.table = None;
             break 'table;
        }
                state_mut.table = table;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'table;
            }
            ProjectsUsersRoleInvitationActions::SetUser(user) => 'user: {
                state_mut.errors_user.clear();
        if user.is_none() {
            state_mut.errors_user.push(ApiError::BadRequest(vec![
                "The User field is required.".to_string()
             ]));
            state_mut.user = None;
             break 'user;
        }
                state_mut.user = user;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'user;
            }
            ProjectsUsersRoleInvitationActions::SetRole(role) => 'role: {
                state_mut.errors_role.clear();
        if role.is_none() {
            state_mut.errors_role.push(ApiError::BadRequest(vec![
                "The Role field is required.".to_string()
             ]));
            state_mut.role = None;
             break 'role;
        }
                state_mut.role = role;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'role;
            }
        }
        state
    }
}
impl FormBuilder for ProjectsUsersRoleInvitationBuilder {
    type Actions = ProjectsUsersRoleInvitationActions;

    type RichVariant = NestedProjectsUsersRoleInvitation;

    fn has_errors(&self) -> bool {
!self.errors_table.is_empty() || !self.errors_user.is_empty() || !self.errors_role.is_empty()
    }

    fn update(dispatcher: &Dispatch<Self>, richest_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.apply(ProjectsUsersRoleInvitationActions::SetTable(Some(richest_variant.table)));
        dispatcher.apply(ProjectsUsersRoleInvitationActions::SetUser(Some(richest_variant.user)));
        dispatcher.apply(ProjectsUsersRoleInvitationActions::SetRole(Some(richest_variant.role)));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.table.is_some()
        && self.user.is_some()
        && self.role.is_some()
    }

}

impl From<ProjectsUsersRoleInvitationBuilder> for NewProjectsUsersRoleInvitation {
    fn from(builder: ProjectsUsersRoleInvitationBuilder) -> Self {
        Self {
            table_id: builder.table.as_ref().map(|table| table.inner.id).unwrap(),
            user_id: builder.user.as_ref().map(|user| user.id).unwrap(),
            role_id: builder.role.unwrap().inner.id,
        }
    }
}
impl FormBuildable for NewProjectsUsersRoleInvitation {
    type Builder = ProjectsUsersRoleInvitationBuilder;
    fn title() -> &'static str {
        "Projects users role invitation"
    }
    fn task_target() -> &'static str {
        "Projects users role invitation"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateProjectsUsersRoleInvitationFormProp {
     #[prop_or_default]
    pub table_id: Option<i32>,
     #[prop_or_default]
    pub user_id: Option<i32>,
     #[prop_or_default]
    pub role_id: Option<i32>,
}

#[function_component(CreateProjectsUsersRoleInvitationForm)]
pub fn create_projects_users_role_invitation_form(props: &CreateProjectsUsersRoleInvitationFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<ProjectsUsersRoleInvitationBuilder>();
   if let Some(table_id) = props.table_id {
         named_requests.push(ComponentMessage::get_named::<&str, Project>("table", table_id.into()));
    }
   if let Some(user_id) = props.user_id {
         named_requests.push(ComponentMessage::get_named::<&str, User>("user", user_id.into()));
    }
   if let Some(role_id) = props.role_id {
         named_requests.push(ComponentMessage::get_named::<&str, Role>("role", role_id.into()));
    }
    let set_table = builder_dispatch.apply_callback(|table: Option<NestedProject>| ProjectsUsersRoleInvitationActions::SetTable(table));
    let set_user = builder_dispatch.apply_callback(|user: Option<User>| ProjectsUsersRoleInvitationActions::SetUser(user));
    let set_role = builder_dispatch.apply_callback(|role: Option<NestedRole>| ProjectsUsersRoleInvitationActions::SetRole(role));
    html! {
        <BasicForm<NewProjectsUsersRoleInvitation>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <Datalist<NestedProject, true> builder={set_table} optional={false} errors={builder_store.errors_table.clone()} value={builder_store.table.clone()} label="Table" />
            <Datalist<User, false> builder={set_user} optional={false} errors={builder_store.errors_user.clone()} value={builder_store.user.clone()} label="User" />
            <Datalist<NestedRole, false> builder={set_role} optional={false} errors={builder_store.errors_role.clone()} value={builder_store.role.clone()} label="Role" />
        </BasicForm<NewProjectsUsersRoleInvitation>>
    }
}
#[derive(Store, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct ProjectsUsersRoleRequestBuilder {
    pub table: Option<NestedProject>,
    pub user: Option<User>,
    pub role: Option<NestedRole>,
    pub errors_table: Vec<ApiError>,
    pub errors_user: Vec<ApiError>,
    pub errors_role: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

impl Default for ProjectsUsersRoleRequestBuilder {
    fn default() -> Self {
        Self {
            table: Default::default(),
            user: Default::default(),
            role: Default::default(),
            errors_table: Default::default(),
            errors_user: Default::default(),
            errors_role: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum ProjectsUsersRoleRequestActions {
    SetTable(Option<NestedProject>),
    SetUser(Option<User>),
    SetRole(Option<NestedRole>),
}

impl FromOperation for ProjectsUsersRoleRequestActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "table" => ProjectsUsersRoleRequestActions::SetTable(Some(bincode::deserialize(&row).unwrap())),
            "user" => ProjectsUsersRoleRequestActions::SetUser(Some(bincode::deserialize(&row).unwrap())),
            "role" => ProjectsUsersRoleRequestActions::SetRole(Some(bincode::deserialize(&row).unwrap())),
            operation_name => unreachable!("The operation name '{}' is not supported.", operation_name),
        }
    }
}

impl Reducer<ProjectsUsersRoleRequestBuilder> for ProjectsUsersRoleRequestActions {
    fn apply(self, mut state: std::rc::Rc<ProjectsUsersRoleRequestBuilder>) -> std::rc::Rc<ProjectsUsersRoleRequestBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            ProjectsUsersRoleRequestActions::SetTable(table) => 'table: {
                state_mut.errors_table.clear();
        if table.is_none() {
            state_mut.errors_table.push(ApiError::BadRequest(vec![
                "The Table field is required.".to_string()
             ]));
            state_mut.table = None;
             break 'table;
        }
                state_mut.table = table;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'table;
            }
            ProjectsUsersRoleRequestActions::SetUser(user) => 'user: {
                state_mut.errors_user.clear();
        if user.is_none() {
            state_mut.errors_user.push(ApiError::BadRequest(vec![
                "The User field is required.".to_string()
             ]));
            state_mut.user = None;
             break 'user;
        }
                state_mut.user = user;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'user;
            }
            ProjectsUsersRoleRequestActions::SetRole(role) => 'role: {
                state_mut.errors_role.clear();
        if role.is_none() {
            state_mut.errors_role.push(ApiError::BadRequest(vec![
                "The Role field is required.".to_string()
             ]));
            state_mut.role = None;
             break 'role;
        }
                state_mut.role = role;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'role;
            }
        }
        state
    }
}
impl FormBuilder for ProjectsUsersRoleRequestBuilder {
    type Actions = ProjectsUsersRoleRequestActions;

    type RichVariant = NestedProjectsUsersRoleRequest;

    fn has_errors(&self) -> bool {
!self.errors_table.is_empty() || !self.errors_user.is_empty() || !self.errors_role.is_empty()
    }

    fn update(dispatcher: &Dispatch<Self>, richest_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.apply(ProjectsUsersRoleRequestActions::SetTable(Some(richest_variant.table)));
        dispatcher.apply(ProjectsUsersRoleRequestActions::SetUser(Some(richest_variant.user)));
        dispatcher.apply(ProjectsUsersRoleRequestActions::SetRole(Some(richest_variant.role)));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.table.is_some()
        && self.user.is_some()
        && self.role.is_some()
    }

}

impl From<ProjectsUsersRoleRequestBuilder> for NewProjectsUsersRoleRequest {
    fn from(builder: ProjectsUsersRoleRequestBuilder) -> Self {
        Self {
            table_id: builder.table.as_ref().map(|table| table.inner.id).unwrap(),
            user_id: builder.user.as_ref().map(|user| user.id).unwrap(),
            role_id: builder.role.unwrap().inner.id,
        }
    }
}
impl FormBuildable for NewProjectsUsersRoleRequest {
    type Builder = ProjectsUsersRoleRequestBuilder;
    fn title() -> &'static str {
        "Projects users role request"
    }
    fn task_target() -> &'static str {
        "Projects users role request"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateProjectsUsersRoleRequestFormProp {
     #[prop_or_default]
    pub table_id: Option<i32>,
     #[prop_or_default]
    pub user_id: Option<i32>,
     #[prop_or_default]
    pub role_id: Option<i32>,
}

#[function_component(CreateProjectsUsersRoleRequestForm)]
pub fn create_projects_users_role_request_form(props: &CreateProjectsUsersRoleRequestFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<ProjectsUsersRoleRequestBuilder>();
   if let Some(table_id) = props.table_id {
         named_requests.push(ComponentMessage::get_named::<&str, Project>("table", table_id.into()));
    }
   if let Some(user_id) = props.user_id {
         named_requests.push(ComponentMessage::get_named::<&str, User>("user", user_id.into()));
    }
   if let Some(role_id) = props.role_id {
         named_requests.push(ComponentMessage::get_named::<&str, Role>("role", role_id.into()));
    }
    let set_table = builder_dispatch.apply_callback(|table: Option<NestedProject>| ProjectsUsersRoleRequestActions::SetTable(table));
    let set_user = builder_dispatch.apply_callback(|user: Option<User>| ProjectsUsersRoleRequestActions::SetUser(user));
    let set_role = builder_dispatch.apply_callback(|role: Option<NestedRole>| ProjectsUsersRoleRequestActions::SetRole(role));
    html! {
        <BasicForm<NewProjectsUsersRoleRequest>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <Datalist<NestedProject, true> builder={set_table} optional={false} errors={builder_store.errors_table.clone()} value={builder_store.table.clone()} label="Table" />
            <Datalist<User, false> builder={set_user} optional={false} errors={builder_store.errors_user.clone()} value={builder_store.user.clone()} label="User" />
            <Datalist<NestedRole, false> builder={set_role} optional={false} errors={builder_store.errors_role.clone()} value={builder_store.role.clone()} label="Role" />
        </BasicForm<NewProjectsUsersRoleRequest>>
    }
}
#[derive(Store, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct ProjectsUsersRoleBuilder {
    pub table: Option<NestedProject>,
    pub user: Option<User>,
    pub role: Option<NestedRole>,
    pub errors_table: Vec<ApiError>,
    pub errors_user: Vec<ApiError>,
    pub errors_role: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

impl Default for ProjectsUsersRoleBuilder {
    fn default() -> Self {
        Self {
            table: Default::default(),
            user: Default::default(),
            role: Default::default(),
            errors_table: Default::default(),
            errors_user: Default::default(),
            errors_role: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum ProjectsUsersRoleActions {
    SetTable(Option<NestedProject>),
    SetUser(Option<User>),
    SetRole(Option<NestedRole>),
}

impl FromOperation for ProjectsUsersRoleActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "table" => ProjectsUsersRoleActions::SetTable(Some(bincode::deserialize(&row).unwrap())),
            "user" => ProjectsUsersRoleActions::SetUser(Some(bincode::deserialize(&row).unwrap())),
            "role" => ProjectsUsersRoleActions::SetRole(Some(bincode::deserialize(&row).unwrap())),
            operation_name => unreachable!("The operation name '{}' is not supported.", operation_name),
        }
    }
}

impl Reducer<ProjectsUsersRoleBuilder> for ProjectsUsersRoleActions {
    fn apply(self, mut state: std::rc::Rc<ProjectsUsersRoleBuilder>) -> std::rc::Rc<ProjectsUsersRoleBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            ProjectsUsersRoleActions::SetTable(table) => 'table: {
                state_mut.errors_table.clear();
        if table.is_none() {
            state_mut.errors_table.push(ApiError::BadRequest(vec![
                "The Table field is required.".to_string()
             ]));
            state_mut.table = None;
             break 'table;
        }
                state_mut.table = table;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'table;
            }
            ProjectsUsersRoleActions::SetUser(user) => 'user: {
                state_mut.errors_user.clear();
        if user.is_none() {
            state_mut.errors_user.push(ApiError::BadRequest(vec![
                "The User field is required.".to_string()
             ]));
            state_mut.user = None;
             break 'user;
        }
                state_mut.user = user;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'user;
            }
            ProjectsUsersRoleActions::SetRole(role) => 'role: {
                state_mut.errors_role.clear();
        if role.is_none() {
            state_mut.errors_role.push(ApiError::BadRequest(vec![
                "The Role field is required.".to_string()
             ]));
            state_mut.role = None;
             break 'role;
        }
                state_mut.role = role;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'role;
            }
        }
        state
    }
}
impl FormBuilder for ProjectsUsersRoleBuilder {
    type Actions = ProjectsUsersRoleActions;

    type RichVariant = NestedProjectsUsersRole;

    fn has_errors(&self) -> bool {
!self.errors_table.is_empty() || !self.errors_user.is_empty() || !self.errors_role.is_empty()
    }

    fn update(dispatcher: &Dispatch<Self>, richest_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.apply(ProjectsUsersRoleActions::SetTable(Some(richest_variant.table)));
        dispatcher.apply(ProjectsUsersRoleActions::SetUser(Some(richest_variant.user)));
        dispatcher.apply(ProjectsUsersRoleActions::SetRole(Some(richest_variant.role)));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.table.is_some()
        && self.user.is_some()
        && self.role.is_some()
    }

}

impl From<ProjectsUsersRoleBuilder> for NewProjectsUsersRole {
    fn from(builder: ProjectsUsersRoleBuilder) -> Self {
        Self {
            table_id: builder.table.as_ref().map(|table| table.inner.id).unwrap(),
            user_id: builder.user.as_ref().map(|user| user.id).unwrap(),
            role_id: builder.role.unwrap().inner.id,
        }
    }
}
impl FormBuildable for NewProjectsUsersRole {
    type Builder = ProjectsUsersRoleBuilder;
    fn title() -> &'static str {
        "Projects users role"
    }
    fn task_target() -> &'static str {
        "Projects users role"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateProjectsUsersRoleFormProp {
     #[prop_or_default]
    pub table_id: Option<i32>,
     #[prop_or_default]
    pub user_id: Option<i32>,
     #[prop_or_default]
    pub role_id: Option<i32>,
}

#[function_component(CreateProjectsUsersRoleForm)]
pub fn create_projects_users_role_form(props: &CreateProjectsUsersRoleFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<ProjectsUsersRoleBuilder>();
   if let Some(table_id) = props.table_id {
         named_requests.push(ComponentMessage::get_named::<&str, Project>("table", table_id.into()));
    }
   if let Some(user_id) = props.user_id {
         named_requests.push(ComponentMessage::get_named::<&str, User>("user", user_id.into()));
    }
   if let Some(role_id) = props.role_id {
         named_requests.push(ComponentMessage::get_named::<&str, Role>("role", role_id.into()));
    }
    let set_table = builder_dispatch.apply_callback(|table: Option<NestedProject>| ProjectsUsersRoleActions::SetTable(table));
    let set_user = builder_dispatch.apply_callback(|user: Option<User>| ProjectsUsersRoleActions::SetUser(user));
    let set_role = builder_dispatch.apply_callback(|role: Option<NestedRole>| ProjectsUsersRoleActions::SetRole(role));
    html! {
        <BasicForm<NewProjectsUsersRole>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <Datalist<NestedProject, true> builder={set_table} optional={false} errors={builder_store.errors_table.clone()} value={builder_store.table.clone()} label="Table" />
            <Datalist<User, false> builder={set_user} optional={false} errors={builder_store.errors_user.clone()} value={builder_store.user.clone()} label="User" />
            <Datalist<NestedRole, false> builder={set_role} optional={false} errors={builder_store.errors_role.clone()} value={builder_store.role.clone()} label="Role" />
        </BasicForm<NewProjectsUsersRole>>
    }
}
#[derive(Store, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct SampleBioOttTaxonItemBuilder {
    pub sample: Option<NestedSample>,
    pub taxon: Option<NestedBioOttTaxonItem>,
    pub errors_sample: Vec<ApiError>,
    pub errors_taxon: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

impl Default for SampleBioOttTaxonItemBuilder {
    fn default() -> Self {
        Self {
            sample: Default::default(),
            taxon: Default::default(),
            errors_sample: Default::default(),
            errors_taxon: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum SampleBioOttTaxonItemActions {
    SetSample(Option<NestedSample>),
    SetTaxon(Option<NestedBioOttTaxonItem>),
}

impl FromOperation for SampleBioOttTaxonItemActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "sample" => SampleBioOttTaxonItemActions::SetSample(Some(bincode::deserialize(&row).unwrap())),
            "taxon" => SampleBioOttTaxonItemActions::SetTaxon(Some(bincode::deserialize(&row).unwrap())),
            operation_name => unreachable!("The operation name '{}' is not supported.", operation_name),
        }
    }
}

impl Reducer<SampleBioOttTaxonItemBuilder> for SampleBioOttTaxonItemActions {
    fn apply(self, mut state: std::rc::Rc<SampleBioOttTaxonItemBuilder>) -> std::rc::Rc<SampleBioOttTaxonItemBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            SampleBioOttTaxonItemActions::SetSample(sample) => 'sample: {
                state_mut.errors_sample.clear();
        if sample.is_none() {
            state_mut.errors_sample.push(ApiError::BadRequest(vec![
                "The Sample field is required.".to_string()
             ]));
            state_mut.sample = None;
             break 'sample;
        }
                state_mut.sample = sample;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'sample;
            }
            SampleBioOttTaxonItemActions::SetTaxon(taxon) => 'taxon: {
                state_mut.errors_taxon.clear();
        if taxon.is_none() {
            state_mut.errors_taxon.push(ApiError::BadRequest(vec![
                "The Taxon field is required.".to_string()
             ]));
            state_mut.taxon = None;
             break 'taxon;
        }
                state_mut.taxon = taxon;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'taxon;
            }
        }
        state
    }
}
impl FormBuilder for SampleBioOttTaxonItemBuilder {
    type Actions = SampleBioOttTaxonItemActions;

    type RichVariant = NestedSampleBioOttTaxonItem;

    fn has_errors(&self) -> bool {
!self.errors_sample.is_empty() || !self.errors_taxon.is_empty()
    }

    fn update(dispatcher: &Dispatch<Self>, richest_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.apply(SampleBioOttTaxonItemActions::SetSample(Some(richest_variant.sample)));
        dispatcher.apply(SampleBioOttTaxonItemActions::SetTaxon(Some(richest_variant.taxon)));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.sample.is_some()
        && self.taxon.is_some()
    }

}

impl From<SampleBioOttTaxonItemBuilder> for NewSampleBioOttTaxonItem {
    fn from(builder: SampleBioOttTaxonItemBuilder) -> Self {
        Self {
            sample_id: builder.sample.as_ref().map(|sample| sample.inner.id).unwrap(),
            taxon_id: builder.taxon.as_ref().map(|taxon| taxon.inner.id).unwrap(),
        }
    }
}
impl FormBuildable for NewSampleBioOttTaxonItem {
    type Builder = SampleBioOttTaxonItemBuilder;
    fn title() -> &'static str {
        "Sample bio ott taxon item"
    }
    fn task_target() -> &'static str {
        "Sample bio ott taxon item"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateSampleBioOttTaxonItemFormProp {
    pub sample_id: Uuid,
     #[prop_or_default]
    pub taxon_id: Option<i32>,
}

#[function_component(CreateSampleBioOttTaxonItemForm)]
pub fn create_sample_bio_ott_taxon_item_form(props: &CreateSampleBioOttTaxonItemFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<SampleBioOttTaxonItemBuilder>();
    named_requests.push(ComponentMessage::get_named::<&str, Sample>("sample", props.sample_id.into()));
   if let Some(taxon_id) = props.taxon_id {
         named_requests.push(ComponentMessage::get_named::<&str, BioOttTaxonItem>("taxon", taxon_id.into()));
    }
    let set_sample = builder_dispatch.apply_callback(|sample: Option<NestedSample>| SampleBioOttTaxonItemActions::SetSample(sample));
    let set_taxon = builder_dispatch.apply_callback(|taxon: Option<NestedBioOttTaxonItem>| SampleBioOttTaxonItemActions::SetTaxon(taxon));
    html! {
        <BasicForm<NewSampleBioOttTaxonItem>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
if let Some(sample) = builder_store.sample.as_ref() {
    <span>{"TODO Selected sample"}</span>
} else {
    <></>
}
            <Datalist<NestedBioOttTaxonItem, false> builder={set_taxon} optional={false} errors={builder_store.errors_taxon.clone()} value={builder_store.taxon.clone()} label="Taxon" />
        </BasicForm<NewSampleBioOttTaxonItem>>
    }
}
#[derive(Store, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct SampleContainerBuilder {
    pub id: Option<i32>,
    pub barcode: Option<String>,
    pub category: Option<NestedSampleContainerCategory>,
    pub errors_barcode: Vec<ApiError>,
    pub errors_category: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

impl Default for SampleContainerBuilder {
    fn default() -> Self {
        Self {
            id: None,
            barcode: None,
            category: Default::default(),
            errors_barcode: Default::default(),
            errors_category: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum SampleContainerActions {
    SetBarcode(Option<String>),
    SetCategory(Option<NestedSampleContainerCategory>),
}

impl FromOperation for SampleContainerActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "category" => SampleContainerActions::SetCategory(Some(bincode::deserialize(&row).unwrap())),
            operation_name => unreachable!("The operation name '{}' is not supported.", operation_name),
        }
    }
}

impl Reducer<SampleContainerBuilder> for SampleContainerActions {
    fn apply(self, mut state: std::rc::Rc<SampleContainerBuilder>) -> std::rc::Rc<SampleContainerBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            SampleContainerActions::SetBarcode(barcode) => 'barcode: {
                state_mut.errors_barcode.clear();
        if barcode.is_none() {
            state_mut.errors_barcode.push(ApiError::BadRequest(vec![
                "The Barcode field is required.".to_string()
             ]));
            state_mut.barcode = None;
             break 'barcode;
        }
                if let Some(value) = barcode.as_ref() {
                    if value.is_empty() {
                        state_mut.errors_barcode.push(ApiError::BadRequest(vec![
                            "The Barcode field cannot be left empty.".to_string()
                        ]));
                         state_mut.barcode = None;
                          break 'barcode;
                    }
                }
                state_mut.barcode = barcode;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'barcode;
            }
            SampleContainerActions::SetCategory(category) => 'category: {
                state_mut.errors_category.clear();
        if category.is_none() {
            state_mut.errors_category.push(ApiError::BadRequest(vec![
                "The Category field is required.".to_string()
             ]));
            state_mut.category = None;
             break 'category;
        }
                state_mut.category = category;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'category;
            }
        }
        state
    }
}
impl FormBuilder for SampleContainerBuilder {
    type Actions = SampleContainerActions;

    type RichVariant = NestedSampleContainer;

    fn has_errors(&self) -> bool {
!self.errors_barcode.is_empty() || !self.errors_category.is_empty()
    }

    fn update(dispatcher: &Dispatch<Self>, richest_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.reduce_mut(|state| {state.id = Some(richest_variant.inner.id);});
    dispatcher.apply(SampleContainerActions::SetBarcode(Some(richest_variant.inner.barcode.to_string())));
        dispatcher.apply(SampleContainerActions::SetCategory(Some(richest_variant.category)));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.barcode.is_some()
        && self.category.is_some()
    }

}

impl From<SampleContainerBuilder> for NewSampleContainer {
    fn from(builder: SampleContainerBuilder) -> Self {
        Self {
            barcode: builder.barcode.unwrap(),
            category_id: builder.category.unwrap().inner.id,
        }
    }
}
impl FormBuildable for NewSampleContainer {
    type Builder = SampleContainerBuilder;
    fn title() -> &'static str {
        "Sample container"
    }
    fn task_target() -> &'static str {
        "Sample container"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateSampleContainerFormProp {
     #[prop_or_default]
    pub category_id: Option<i32>,
}

#[function_component(CreateSampleContainerForm)]
pub fn create_sample_container_form(props: &CreateSampleContainerFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<SampleContainerBuilder>();
   if let Some(category_id) = props.category_id {
         named_requests.push(ComponentMessage::get_named::<&str, SampleContainerCategory>("category", category_id.into()));
    }
    let set_barcode = builder_dispatch.apply_callback(|barcode: Option<String>| SampleContainerActions::SetBarcode(barcode));
    let set_category = builder_dispatch.apply_callback(|category: Option<NestedSampleContainerCategory>| SampleContainerActions::SetCategory(category));
    html! {
        <BasicForm<NewSampleContainer>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="Barcode" optional={false} errors={builder_store.errors_barcode.clone()} builder={set_barcode} value={builder_store.barcode.clone()} />
            <Datalist<NestedSampleContainerCategory, false> builder={set_category} optional={false} errors={builder_store.errors_category.clone()} value={builder_store.category.clone()} label="Category" />
        </BasicForm<NewSampleContainer>>
    }
}
#[derive(Store, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct SampledIndividualBioOttTaxonItemBuilder {
    pub sampled_individual: Option<NestedSampledIndividual>,
    pub taxon: Option<NestedBioOttTaxonItem>,
    pub errors_sampled_individual: Vec<ApiError>,
    pub errors_taxon: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

impl Default for SampledIndividualBioOttTaxonItemBuilder {
    fn default() -> Self {
        Self {
            sampled_individual: Default::default(),
            taxon: Default::default(),
            errors_sampled_individual: Default::default(),
            errors_taxon: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum SampledIndividualBioOttTaxonItemActions {
    SetSampledIndividual(Option<NestedSampledIndividual>),
    SetTaxon(Option<NestedBioOttTaxonItem>),
}

impl FromOperation for SampledIndividualBioOttTaxonItemActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "sampled_individual" => SampledIndividualBioOttTaxonItemActions::SetSampledIndividual(Some(bincode::deserialize(&row).unwrap())),
            "taxon" => SampledIndividualBioOttTaxonItemActions::SetTaxon(Some(bincode::deserialize(&row).unwrap())),
            operation_name => unreachable!("The operation name '{}' is not supported.", operation_name),
        }
    }
}

impl Reducer<SampledIndividualBioOttTaxonItemBuilder> for SampledIndividualBioOttTaxonItemActions {
    fn apply(self, mut state: std::rc::Rc<SampledIndividualBioOttTaxonItemBuilder>) -> std::rc::Rc<SampledIndividualBioOttTaxonItemBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            SampledIndividualBioOttTaxonItemActions::SetSampledIndividual(sampled_individual) => 'sampled_individual: {
                state_mut.errors_sampled_individual.clear();
        if sampled_individual.is_none() {
            state_mut.errors_sampled_individual.push(ApiError::BadRequest(vec![
                "The Sampled individual field is required.".to_string()
             ]));
            state_mut.sampled_individual = None;
             break 'sampled_individual;
        }
                state_mut.sampled_individual = sampled_individual;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'sampled_individual;
            }
            SampledIndividualBioOttTaxonItemActions::SetTaxon(taxon) => 'taxon: {
                state_mut.errors_taxon.clear();
        if taxon.is_none() {
            state_mut.errors_taxon.push(ApiError::BadRequest(vec![
                "The Taxon field is required.".to_string()
             ]));
            state_mut.taxon = None;
             break 'taxon;
        }
                state_mut.taxon = taxon;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'taxon;
            }
        }
        state
    }
}
impl FormBuilder for SampledIndividualBioOttTaxonItemBuilder {
    type Actions = SampledIndividualBioOttTaxonItemActions;

    type RichVariant = NestedSampledIndividualBioOttTaxonItem;

    fn has_errors(&self) -> bool {
!self.errors_sampled_individual.is_empty() || !self.errors_taxon.is_empty()
    }

    fn update(dispatcher: &Dispatch<Self>, richest_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.apply(SampledIndividualBioOttTaxonItemActions::SetSampledIndividual(Some(richest_variant.sampled_individual)));
        dispatcher.apply(SampledIndividualBioOttTaxonItemActions::SetTaxon(Some(richest_variant.taxon)));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.sampled_individual.is_some()
        && self.taxon.is_some()
    }

}

impl From<SampledIndividualBioOttTaxonItemBuilder> for NewSampledIndividualBioOttTaxonItem {
    fn from(builder: SampledIndividualBioOttTaxonItemBuilder) -> Self {
        Self {
            sampled_individual_id: builder.sampled_individual.as_ref().map(|sampled_individual| sampled_individual.inner.id).unwrap(),
            taxon_id: builder.taxon.as_ref().map(|taxon| taxon.inner.id).unwrap(),
        }
    }
}
impl FormBuildable for NewSampledIndividualBioOttTaxonItem {
    type Builder = SampledIndividualBioOttTaxonItemBuilder;
    fn title() -> &'static str {
        "Sampled individual bio ott taxon item"
    }
    fn task_target() -> &'static str {
        "Sampled individual bio ott taxon item"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateSampledIndividualBioOttTaxonItemFormProp {
     #[prop_or_default]
    pub sampled_individual_id: Option<Uuid>,
     #[prop_or_default]
    pub taxon_id: Option<i32>,
}

#[function_component(CreateSampledIndividualBioOttTaxonItemForm)]
pub fn create_sampled_individual_bio_ott_taxon_item_form(props: &CreateSampledIndividualBioOttTaxonItemFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<SampledIndividualBioOttTaxonItemBuilder>();
   if let Some(sampled_individual_id) = props.sampled_individual_id {
         named_requests.push(ComponentMessage::get_named::<&str, SampledIndividual>("sampled_individual", sampled_individual_id.into()));
    }
   if let Some(taxon_id) = props.taxon_id {
         named_requests.push(ComponentMessage::get_named::<&str, BioOttTaxonItem>("taxon", taxon_id.into()));
    }
    let set_sampled_individual = builder_dispatch.apply_callback(|sampled_individual: Option<NestedSampledIndividual>| SampledIndividualBioOttTaxonItemActions::SetSampledIndividual(sampled_individual));
    let set_taxon = builder_dispatch.apply_callback(|taxon: Option<NestedBioOttTaxonItem>| SampledIndividualBioOttTaxonItemActions::SetTaxon(taxon));
    html! {
        <BasicForm<NewSampledIndividualBioOttTaxonItem>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <Datalist<NestedSampledIndividual, false> builder={set_sampled_individual} optional={false} errors={builder_store.errors_sampled_individual.clone()} value={builder_store.sampled_individual.clone()} label="Sampled individual" />
            <Datalist<NestedBioOttTaxonItem, false> builder={set_taxon} optional={false} errors={builder_store.errors_taxon.clone()} value={builder_store.taxon.clone()} label="Taxon" />
        </BasicForm<NewSampledIndividualBioOttTaxonItem>>
    }
}
#[derive(Store, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct SampledIndividualBuilder {
    pub id: Option<Uuid>,
    pub notes: Option<String>,
    pub barcode: Option<String>,
    pub picture: Option<Vec<u8>>,
    pub project: Option<NestedProject>,
    pub errors_notes: Vec<ApiError>,
    pub errors_barcode: Vec<ApiError>,
    pub errors_picture: Vec<ApiError>,
    pub errors_project: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

impl Default for SampledIndividualBuilder {
    fn default() -> Self {
        Self {
            id: None,
            notes: None,
            barcode: None,
            picture: None,
            project: Default::default(),
            errors_notes: Default::default(),
            errors_barcode: Default::default(),
            errors_picture: Default::default(),
            errors_project: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum SampledIndividualActions {
    SetNotes(Option<String>),
    SetBarcode(Option<String>),
    SetPicture(Option<Vec<u8>>),
    SetProject(Option<NestedProject>),
}

impl FromOperation for SampledIndividualActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "project" => SampledIndividualActions::SetProject(Some(bincode::deserialize(&row).unwrap())),
            operation_name => unreachable!("The operation name '{}' is not supported.", operation_name),
        }
    }
}

impl Reducer<SampledIndividualBuilder> for SampledIndividualActions {
    fn apply(self, mut state: std::rc::Rc<SampledIndividualBuilder>) -> std::rc::Rc<SampledIndividualBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            SampledIndividualActions::SetNotes(notes) => 'notes: {
                state_mut.errors_notes.clear();
                if let Some(value) = notes.as_ref() {
                    if value.is_empty() {
                        state_mut.errors_notes.push(ApiError::BadRequest(vec![
                            "The Notes field cannot be left empty.".to_string()
                        ]));
                         state_mut.notes = None;
                          break 'notes;
                    }
                }
                state_mut.notes = notes;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'notes;
            }
            SampledIndividualActions::SetBarcode(barcode) => 'barcode: {
                state_mut.errors_barcode.clear();
                if let Some(value) = barcode.as_ref() {
                    if value.is_empty() {
                        state_mut.errors_barcode.push(ApiError::BadRequest(vec![
                            "The Barcode field cannot be left empty.".to_string()
                        ]));
                         state_mut.barcode = None;
                          break 'barcode;
                    }
                }
                state_mut.barcode = barcode;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'barcode;
            }
            SampledIndividualActions::SetPicture(picture) => 'picture: {
                state_mut.errors_picture.clear();
        if picture.is_none() {
            state_mut.errors_picture.push(ApiError::BadRequest(vec![
                "The Picture field is required.".to_string()
             ]));
            state_mut.picture = None;
             break 'picture;
        }
                state_mut.picture = picture;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'picture;
            }
            SampledIndividualActions::SetProject(project) => 'project: {
                state_mut.errors_project.clear();
        if project.is_none() {
            state_mut.errors_project.push(ApiError::BadRequest(vec![
                "The Project field is required.".to_string()
             ]));
            state_mut.project = None;
             break 'project;
        }
                state_mut.project = project;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'project;
            }
        }
        state
    }
}
impl FormBuilder for SampledIndividualBuilder {
    type Actions = SampledIndividualActions;

    type RichVariant = NestedSampledIndividual;

    fn has_errors(&self) -> bool {
!self.errors_notes.is_empty() || !self.errors_barcode.is_empty() || !self.errors_picture.is_empty() || !self.errors_project.is_empty()
    }

    fn update(dispatcher: &Dispatch<Self>, richest_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.reduce_mut(|state| {state.id = Some(richest_variant.inner.id);});
    dispatcher.apply(SampledIndividualActions::SetNotes(richest_variant.inner.notes.map(|notes| notes.to_string())));
    dispatcher.apply(SampledIndividualActions::SetBarcode(richest_variant.inner.barcode.map(|barcode| barcode.to_string())));
        dispatcher.apply(SampledIndividualActions::SetPicture(Some(richest_variant.inner.picture)));        dispatcher.apply(SampledIndividualActions::SetProject(Some(richest_variant.project)));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.picture.is_some()
        && self.project.is_some()
    }

}

impl From<SampledIndividualBuilder> for NewSampledIndividual {
    fn from(builder: SampledIndividualBuilder) -> Self {
        Self {
            id: builder.id.unwrap_or_else(Uuid::new_v4),
            notes: builder.notes,
            barcode: builder.barcode,
            project_id: builder.project.unwrap().inner.id,
            picture: builder.picture.unwrap(),
        }
    }
}
impl FormBuildable for NewSampledIndividual {
    type Builder = SampledIndividualBuilder;
    fn title() -> &'static str {
        "Sampled individual"
    }
    fn task_target() -> &'static str {
        "Sampled individual"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        true
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateSampledIndividualFormProp {
     #[prop_or_default]
    pub project_id: Option<i32>,
}

#[function_component(CreateSampledIndividualForm)]
pub fn create_sampled_individual_form(props: &CreateSampledIndividualFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<SampledIndividualBuilder>();
   if let Some(project_id) = props.project_id {
         named_requests.push(ComponentMessage::get_named::<&str, Project>("project", project_id.into()));
    }
    let set_notes = builder_dispatch.apply_callback(|notes: Option<String>| SampledIndividualActions::SetNotes(notes));
    let set_barcode = builder_dispatch.apply_callback(|barcode: Option<String>| SampledIndividualActions::SetBarcode(barcode));
    let set_picture = builder_dispatch.apply_callback(|picture: Option<Image>| SampledIndividualActions::SetPicture(picture.map(|picture| picture.into())));
    let set_project = builder_dispatch.apply_callback(|project: Option<NestedProject>| SampledIndividualActions::SetProject(project));
    html! {
        <BasicForm<NewSampledIndividual>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="Notes" optional={true} errors={builder_store.errors_notes.clone()} builder={set_notes} value={builder_store.notes.clone()} />
            <BasicInput<String> label="Barcode" optional={true} errors={builder_store.errors_barcode.clone()} builder={set_barcode} value={builder_store.barcode.clone()} />
            <FileInput<Image> label="Picture" optional={false} errors={builder_store.errors_picture.clone()} builder={set_picture} allowed_formats={vec![GenericFileFormat::Image]} value={builder_store.picture.clone().map(|picture| picture.into())} />
            <Datalist<NestedProject, true> builder={set_project} optional={false} errors={builder_store.errors_project.clone()} value={builder_store.project.clone()} label="Project" />
        </BasicForm<NewSampledIndividual>>
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct UpdateSampledIndividualFormProp {
    pub id: Uuid,
}

#[function_component(UpdateSampledIndividualForm)]
pub fn update_sampled_individual_form(props: &UpdateSampledIndividualFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<SampledIndividualBuilder>();
    // We push the ID of the row to the named requests.
    let props = props.clone();
   named_requests.push(ComponentMessage::get::<NewSampledIndividual>(props.id.into()));
    let set_notes = builder_dispatch.apply_callback(|notes: Option<String>| SampledIndividualActions::SetNotes(notes));
    let set_barcode = builder_dispatch.apply_callback(|barcode: Option<String>| SampledIndividualActions::SetBarcode(barcode));
    let set_picture = builder_dispatch.apply_callback(|picture: Option<Image>| SampledIndividualActions::SetPicture(picture.map(|picture| picture.into())));
    let set_project = builder_dispatch.apply_callback(|project: Option<NestedProject>| SampledIndividualActions::SetProject(project));
    html! {
        <BasicForm<NewSampledIndividual>
            method={FormMethod::PUT}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="Notes" optional={true} errors={builder_store.errors_notes.clone()} builder={set_notes} value={builder_store.notes.clone()} />
            <BasicInput<String> label="Barcode" optional={true} errors={builder_store.errors_barcode.clone()} builder={set_barcode} value={builder_store.barcode.clone()} />
            <FileInput<Image> label="Picture" optional={false} errors={builder_store.errors_picture.clone()} builder={set_picture} allowed_formats={vec![GenericFileFormat::Image]} value={builder_store.picture.clone().map(|picture| picture.into())} />
            <Datalist<NestedProject, true> builder={set_project} optional={false} errors={builder_store.errors_project.clone()} value={builder_store.project.clone()} label="Project" />
        </BasicForm<NewSampledIndividual>>
    }
}
#[derive(Store, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct SampleBuilder {
    pub id: Option<Uuid>,
    pub notes: Option<String>,
    pub container: Option<NestedSampleContainer>,
    pub sampled_by: Option<User>,
    pub state: Option<NestedSampleState>,
    pub errors_notes: Vec<ApiError>,
    pub errors_container: Vec<ApiError>,
    pub errors_sampled_by: Vec<ApiError>,
    pub errors_state: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

impl Default for SampleBuilder {
    fn default() -> Self {
        Self {
            id: None,
            notes: None,
            container: Default::default(),
            sampled_by: None,
            state: None,
            errors_notes: Default::default(),
            errors_container: Default::default(),
            errors_sampled_by: Default::default(),
            errors_state: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum SampleActions {
    SetNotes(Option<String>),
    SetContainer(Option<NestedSampleContainer>),
    SetSampledBy(Option<User>),
    SetState(Option<NestedSampleState>),
}

impl FromOperation for SampleActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "container" => SampleActions::SetContainer(Some(bincode::deserialize(&row).unwrap())),
            "sampled_by" => SampleActions::SetSampledBy(Some(bincode::deserialize(&row).unwrap())),
            "state" => SampleActions::SetState(Some(bincode::deserialize(&row).unwrap())),
            operation_name => unreachable!("The operation name '{}' is not supported.", operation_name),
        }
    }
}

impl Reducer<SampleBuilder> for SampleActions {
    fn apply(self, mut state: std::rc::Rc<SampleBuilder>) -> std::rc::Rc<SampleBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            SampleActions::SetNotes(notes) => 'notes: {
                state_mut.errors_notes.clear();
                if let Some(value) = notes.as_ref() {
                    if value.is_empty() {
                        state_mut.errors_notes.push(ApiError::BadRequest(vec![
                            "The Notes field cannot be left empty.".to_string()
                        ]));
                         state_mut.notes = None;
                          break 'notes;
                    }
                }
                state_mut.notes = notes;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'notes;
            }
            SampleActions::SetContainer(container) => 'container: {
                state_mut.errors_container.clear();
        if container.is_none() {
            state_mut.errors_container.push(ApiError::BadRequest(vec![
                "The Container field is required.".to_string()
             ]));
            state_mut.container = None;
             break 'container;
        }
                state_mut.container = container;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'container;
            }
            SampleActions::SetSampledBy(sampled_by) => 'sampled_by: {
                state_mut.errors_sampled_by.clear();
        if sampled_by.is_none() {
            state_mut.errors_sampled_by.push(ApiError::BadRequest(vec![
                "The Sampled by field is required.".to_string()
             ]));
            state_mut.sampled_by = None;
             break 'sampled_by;
        }
                state_mut.sampled_by = sampled_by;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'sampled_by;
            }
            SampleActions::SetState(state) => 'state: {
                state_mut.errors_state.clear();
        if state.is_none() {
            state_mut.errors_state.push(ApiError::BadRequest(vec![
                "The State field is required.".to_string()
             ]));
            state_mut.state = None;
             break 'state;
        }
                state_mut.state = state;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'state;
            }
        }
        state
    }
}
impl FormBuilder for SampleBuilder {
    type Actions = SampleActions;

    type RichVariant = NestedSample;

    fn has_errors(&self) -> bool {
!self.errors_notes.is_empty() || !self.errors_container.is_empty() || !self.errors_sampled_by.is_empty() || !self.errors_state.is_empty()
    }

    fn update(dispatcher: &Dispatch<Self>, richest_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.reduce_mut(|state| {state.id = Some(richest_variant.inner.id);});
    dispatcher.apply(SampleActions::SetNotes(richest_variant.inner.notes.map(|notes| notes.to_string())));
        dispatcher.apply(SampleActions::SetContainer(Some(richest_variant.container)));
        dispatcher.apply(SampleActions::SetSampledBy(Some(richest_variant.sampled_by)));
        dispatcher.apply(SampleActions::SetState(Some(richest_variant.state)));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.container.is_some()
        && self.sampled_by.is_some()
        && self.state.is_some()
    }

}

impl From<SampleBuilder> for NewSample {
    fn from(builder: SampleBuilder) -> Self {
        Self {
            id: builder.id.unwrap_or_else(Uuid::new_v4),
            container_id: builder.container.unwrap().inner.id,
            notes: builder.notes,
            sampled_by: builder.sampled_by.unwrap().id,
            state: builder.state.unwrap().inner.id,
        }
    }
}
impl FormBuildable for NewSample {
    type Builder = SampleBuilder;
    fn title() -> &'static str {
        "Sample"
    }
    fn task_target() -> &'static str {
        "Sample"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        true
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateSampleFormProp {
     #[prop_or_default]
    pub container_id: Option<i32>,
     #[prop_or_default]
    pub sampled_by: Option<i32>,
     #[prop_or_default]
    pub state: Option<i32>,
}

#[function_component(CreateSampleForm)]
pub fn create_sample_form(props: &CreateSampleFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<SampleBuilder>();
   if let Some(container_id) = props.container_id {
         named_requests.push(ComponentMessage::get_named::<&str, SampleContainer>("container", container_id.into()));
    }
   if let Some(sampled_by) = props.sampled_by {
         named_requests.push(ComponentMessage::get_named::<&str, User>("sampled_by", sampled_by.into()));
    }
   if let Some(state) = props.state {
         named_requests.push(ComponentMessage::get_named::<&str, SampleState>("state", state.into()));
    }
    let set_notes = builder_dispatch.apply_callback(|notes: Option<String>| SampleActions::SetNotes(notes));
    let set_container = builder_dispatch.apply_callback(|container: Option<NestedSampleContainer>| SampleActions::SetContainer(container));
    let set_sampled_by = builder_dispatch.apply_callback(|sampled_by: Option<User>| SampleActions::SetSampledBy(sampled_by));
    let set_state = builder_dispatch.apply_callback(|state: Option<NestedSampleState>| SampleActions::SetState(state));
    html! {
        <BasicForm<NewSample>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="Notes" optional={true} errors={builder_store.errors_notes.clone()} builder={set_notes} value={builder_store.notes.clone()} />
            <Datalist<NestedSampleContainer, false> builder={set_container} optional={false} errors={builder_store.errors_container.clone()} value={builder_store.container.clone()} label="Container" />
            <Datalist<User, false> builder={set_sampled_by} optional={false} errors={builder_store.errors_sampled_by.clone()} value={builder_store.sampled_by.clone()} label="Sampled by" />
            <Datalist<NestedSampleState, false> builder={set_state} optional={false} errors={builder_store.errors_state.clone()} value={builder_store.state.clone()} label="State" />
        </BasicForm<NewSample>>
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct UpdateSampleFormProp {
    pub id: Uuid,
}

#[function_component(UpdateSampleForm)]
pub fn update_sample_form(props: &UpdateSampleFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<SampleBuilder>();
    // We push the ID of the row to the named requests.
    let props = props.clone();
   named_requests.push(ComponentMessage::get::<NewSample>(props.id.into()));
    let set_notes = builder_dispatch.apply_callback(|notes: Option<String>| SampleActions::SetNotes(notes));
    let set_container = builder_dispatch.apply_callback(|container: Option<NestedSampleContainer>| SampleActions::SetContainer(container));
    let set_sampled_by = builder_dispatch.apply_callback(|sampled_by: Option<User>| SampleActions::SetSampledBy(sampled_by));
    let set_state = builder_dispatch.apply_callback(|state: Option<NestedSampleState>| SampleActions::SetState(state));
    html! {
        <BasicForm<NewSample>
            method={FormMethod::PUT}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="Notes" optional={true} errors={builder_store.errors_notes.clone()} builder={set_notes} value={builder_store.notes.clone()} />
            <Datalist<NestedSampleContainer, false> builder={set_container} optional={false} errors={builder_store.errors_container.clone()} value={builder_store.container.clone()} label="Container" />
            <Datalist<User, false> builder={set_sampled_by} optional={false} errors={builder_store.errors_sampled_by.clone()} value={builder_store.sampled_by.clone()} label="Sampled by" />
            <Datalist<NestedSampleState, false> builder={set_state} optional={false} errors={builder_store.errors_state.clone()} value={builder_store.state.clone()} label="State" />
        </BasicForm<NewSample>>
    }
}
#[derive(Store, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct SpectraCollectionBuilder {
    pub id: Option<i32>,
    pub notes: Option<String>,
    pub sample: Option<NestedSample>,
    pub errors_notes: Vec<ApiError>,
    pub errors_sample: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

impl Default for SpectraCollectionBuilder {
    fn default() -> Self {
        Self {
            id: None,
            notes: None,
            sample: Default::default(),
            errors_notes: Default::default(),
            errors_sample: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum SpectraCollectionActions {
    SetNotes(Option<String>),
    SetSample(Option<NestedSample>),
}

impl FromOperation for SpectraCollectionActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "sample" => SpectraCollectionActions::SetSample(Some(bincode::deserialize(&row).unwrap())),
            operation_name => unreachable!("The operation name '{}' is not supported.", operation_name),
        }
    }
}

impl Reducer<SpectraCollectionBuilder> for SpectraCollectionActions {
    fn apply(self, mut state: std::rc::Rc<SpectraCollectionBuilder>) -> std::rc::Rc<SpectraCollectionBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            SpectraCollectionActions::SetNotes(notes) => 'notes: {
                state_mut.errors_notes.clear();
                if let Some(value) = notes.as_ref() {
                    if value.is_empty() {
                        state_mut.errors_notes.push(ApiError::BadRequest(vec![
                            "The Notes field cannot be left empty.".to_string()
                        ]));
                         state_mut.notes = None;
                          break 'notes;
                    }
                }
                state_mut.notes = notes;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'notes;
            }
            SpectraCollectionActions::SetSample(sample) => 'sample: {
                state_mut.errors_sample.clear();
        if sample.is_none() {
            state_mut.errors_sample.push(ApiError::BadRequest(vec![
                "The Sample field is required.".to_string()
             ]));
            state_mut.sample = None;
             break 'sample;
        }
                state_mut.sample = sample;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'sample;
            }
        }
        state
    }
}
impl FormBuilder for SpectraCollectionBuilder {
    type Actions = SpectraCollectionActions;

    type RichVariant = NestedSpectraCollection;

    fn has_errors(&self) -> bool {
!self.errors_notes.is_empty() || !self.errors_sample.is_empty()
    }

    fn update(dispatcher: &Dispatch<Self>, richest_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.reduce_mut(|state| {state.id = Some(richest_variant.inner.id);});
    dispatcher.apply(SpectraCollectionActions::SetNotes(richest_variant.inner.notes.map(|notes| notes.to_string())));
        dispatcher.apply(SpectraCollectionActions::SetSample(Some(richest_variant.sample)));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.sample.is_some()
    }

}

impl From<SpectraCollectionBuilder> for NewSpectraCollection {
    fn from(builder: SpectraCollectionBuilder) -> Self {
        Self {
            notes: builder.notes,
            sample_id: builder.sample.unwrap().inner.id,
        }
    }
}
impl From<SpectraCollectionBuilder> for UpdateSpectraCollection {
    fn from(builder: SpectraCollectionBuilder) -> Self {
        Self {
            id: builder.id.unwrap(),
            notes: builder.notes,
            sample_id: builder.sample.unwrap().inner.id,
        }
    }
}
impl FormBuildable for NewSpectraCollection {
    type Builder = SpectraCollectionBuilder;
    fn title() -> &'static str {
        "Spectra collection"
    }
    fn task_target() -> &'static str {
        "Spectra collection"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

impl FormBuildable for UpdateSpectraCollection {
    type Builder = SpectraCollectionBuilder;
    fn title() -> &'static str {
        "Spectra collection"
    }
    fn task_target() -> &'static str {
        "Spectra collection"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        true
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateSpectraCollectionFormProp {
    pub sample_id: Uuid,
}

#[function_component(CreateSpectraCollectionForm)]
pub fn create_spectra_collection_form(props: &CreateSpectraCollectionFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<SpectraCollectionBuilder>();
    named_requests.push(ComponentMessage::get_named::<&str, Sample>("sample", props.sample_id.into()));
    let set_notes = builder_dispatch.apply_callback(|notes: Option<String>| SpectraCollectionActions::SetNotes(notes));
    let set_sample = builder_dispatch.apply_callback(|sample: Option<NestedSample>| SpectraCollectionActions::SetSample(sample));
    html! {
        <BasicForm<NewSpectraCollection>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="Notes" optional={true} errors={builder_store.errors_notes.clone()} builder={set_notes} value={builder_store.notes.clone()} />
if let Some(sample) = builder_store.sample.as_ref() {
    <span>{"TODO Selected sample"}</span>
} else {
    <></>
}
        </BasicForm<NewSpectraCollection>>
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct UpdateSpectraCollectionFormProp {
    pub id: i32,
}

#[function_component(UpdateSpectraCollectionForm)]
pub fn update_spectra_collection_form(props: &UpdateSpectraCollectionFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<SpectraCollectionBuilder>();
    // We push the ID of the row to the named requests.
    let props = props.clone();
   named_requests.push(ComponentMessage::get::<UpdateSpectraCollection>(props.id.into()));
    let set_notes = builder_dispatch.apply_callback(|notes: Option<String>| SpectraCollectionActions::SetNotes(notes));
    let set_sample = builder_dispatch.apply_callback(|sample: Option<NestedSample>| SpectraCollectionActions::SetSample(sample));
    html! {
        <BasicForm<UpdateSpectraCollection>
            method={FormMethod::PUT}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="Notes" optional={true} errors={builder_store.errors_notes.clone()} builder={set_notes} value={builder_store.notes.clone()} />
<p>{"Sample id has to be selected with a ScannerInput, which is not yet available."}</p>
        </BasicForm<UpdateSpectraCollection>>
    }
}
#[derive(Store, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct TeamBuilder {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub icon: Option<FontAwesomeIcon>,
    pub color: Option<Color>,
    pub parent_team: Option<NestedTeam>,
    pub errors_name: Vec<ApiError>,
    pub errors_description: Vec<ApiError>,
    pub errors_icon: Vec<ApiError>,
    pub errors_color: Vec<ApiError>,
    pub errors_parent_team: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

impl Default for TeamBuilder {
    fn default() -> Self {
        Self {
            id: None,
            name: None,
            description: None,
            icon: Default::default(),
            color: Default::default(),
            parent_team: Default::default(),
            errors_name: Default::default(),
            errors_description: Default::default(),
            errors_icon: Default::default(),
            errors_color: Default::default(),
            errors_parent_team: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum TeamActions {
    SetName(Option<String>),
    SetDescription(Option<String>),
    SetIcon(Option<FontAwesomeIcon>),
    SetColor(Option<Color>),
    SetParentTeam(Option<NestedTeam>),
}

impl FromOperation for TeamActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "icon" => TeamActions::SetIcon(Some(bincode::deserialize(&row).unwrap())),
            "color" => TeamActions::SetColor(Some(bincode::deserialize(&row).unwrap())),
            "parent_team" => TeamActions::SetParentTeam(Some(bincode::deserialize(&row).unwrap())),
            operation_name => unreachable!("The operation name '{}' is not supported.", operation_name),
        }
    }
}

impl Reducer<TeamBuilder> for TeamActions {
    fn apply(self, mut state: std::rc::Rc<TeamBuilder>) -> std::rc::Rc<TeamBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            TeamActions::SetName(name) => 'name: {
                state_mut.errors_name.clear();
        if name.is_none() {
            state_mut.errors_name.push(ApiError::BadRequest(vec![
                "The Name field is required.".to_string()
             ]));
            state_mut.name = None;
             break 'name;
        }
                if let Some(value) = name.as_ref() {
                    if value.is_empty() {
                        state_mut.errors_name.push(ApiError::BadRequest(vec![
                            "The Name field cannot be left empty.".to_string()
                        ]));
                         state_mut.name = None;
                          break 'name;
                    }
                }
                state_mut.name = name;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'name;
            }
            TeamActions::SetDescription(description) => 'description: {
                state_mut.errors_description.clear();
        if description.is_none() {
            state_mut.errors_description.push(ApiError::BadRequest(vec![
                "The Description field is required.".to_string()
             ]));
            state_mut.description = None;
             break 'description;
        }
                if let Some(value) = description.as_ref() {
                    if value.is_empty() {
                        state_mut.errors_description.push(ApiError::BadRequest(vec![
                            "The Description field cannot be left empty.".to_string()
                        ]));
                         state_mut.description = None;
                          break 'description;
                    }
                }
                state_mut.description = description;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'description;
            }
            TeamActions::SetIcon(icon) => 'icon: {
                state_mut.errors_icon.clear();
        if icon.is_none() {
            state_mut.errors_icon.push(ApiError::BadRequest(vec![
                "The Icon field is required.".to_string()
             ]));
            state_mut.icon = None;
             break 'icon;
        }
                state_mut.icon = icon;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'icon;
            }
            TeamActions::SetColor(color) => 'color: {
                state_mut.errors_color.clear();
        if color.is_none() {
            state_mut.errors_color.push(ApiError::BadRequest(vec![
                "The Color field is required.".to_string()
             ]));
            state_mut.color = None;
             break 'color;
        }
                state_mut.color = color;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'color;
            }
            TeamActions::SetParentTeam(parent_team) => 'parent_team: {
                state_mut.errors_parent_team.clear();
                match parent_team.as_ref() {
                    Some(parent_team) => {
                            if state_mut.id.map_or(false, |id| id == parent_team.inner.id)
                        {
                            state_mut.errors_parent_team.push(ApiError::BadRequest(vec![
                                "The Parent team field must be distinct from the current value.".to_string()
                             ]));
                            break 'parent_team;
                        }
                    }
                    None => (),
                }
                state_mut.parent_team = parent_team;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'parent_team;
            }
        }
        state
    }
}
impl FormBuilder for TeamBuilder {
    type Actions = TeamActions;

    type RichVariant = NestedTeam;

    fn has_errors(&self) -> bool {
!self.errors_name.is_empty() || !self.errors_description.is_empty() || !self.errors_icon.is_empty() || !self.errors_color.is_empty() || !self.errors_parent_team.is_empty()
    }

    fn update(dispatcher: &Dispatch<Self>, richest_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.reduce_mut(|state| {state.id = Some(richest_variant.inner.id);});
    dispatcher.apply(TeamActions::SetName(Some(richest_variant.inner.name.to_string())));
    dispatcher.apply(TeamActions::SetDescription(Some(richest_variant.inner.description.to_string())));
        dispatcher.apply(TeamActions::SetIcon(Some(richest_variant.icon)));
        dispatcher.apply(TeamActions::SetColor(Some(richest_variant.color)));
        let mut named_requests = Vec::new();
        if let Some(parent_team_id) = richest_variant.inner.parent_team_id {
    named_requests.push(ComponentMessage::get_named::<&str, Team>("parent_team", parent_team_id.into()));
 } else {
    dispatcher.apply(TeamActions::SetParentTeam(None));
 }
        named_requests
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.name.is_some()
        && self.description.is_some()
        && self.icon.is_some()
        && self.color.is_some()
    }

}

impl From<TeamBuilder> for NewTeam {
    fn from(builder: TeamBuilder) -> Self {
        Self {
            name: builder.name.unwrap(),
            description: builder.description.unwrap(),
            icon_id: builder.icon.unwrap().id,
            color_id: builder.color.unwrap().id,
            parent_team_id: builder.parent_team.map(|parent_team| parent_team.inner.id),
        }
    }
}
impl From<TeamBuilder> for UpdateTeam {
    fn from(builder: TeamBuilder) -> Self {
        Self {
            id: builder.id.unwrap(),
            name: builder.name.unwrap(),
            description: builder.description.unwrap(),
            icon_id: builder.icon.unwrap().id,
            color_id: builder.color.unwrap().id,
            parent_team_id: builder.parent_team.map(|parent_team| parent_team.inner.id),
        }
    }
}
impl FormBuildable for NewTeam {
    type Builder = TeamBuilder;
    fn title() -> &'static str {
        "Team"
    }
    fn task_target() -> &'static str {
        "Team"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

impl FormBuildable for UpdateTeam {
    type Builder = TeamBuilder;
    fn title() -> &'static str {
        "Team"
    }
    fn task_target() -> &'static str {
        "Team"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        true
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateTeamFormProp {
     #[prop_or(1387)]
     pub icon_id: i32,
     #[prop_or(15)]
     pub color_id: i32,
     #[prop_or_default]
    pub parent_team_id: Option<i32>,
}

#[function_component(CreateTeamForm)]
pub fn create_team_form(props: &CreateTeamFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<TeamBuilder>();
    named_requests.push(ComponentMessage::get_named::<&str, FontAwesomeIcon>("icon", props.icon_id.into()));
    named_requests.push(ComponentMessage::get_named::<&str, Color>("color", props.color_id.into()));
   if let Some(parent_team_id) = props.parent_team_id {
         named_requests.push(ComponentMessage::get_named::<&str, Team>("parent_team", parent_team_id.into()));
    }
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| TeamActions::SetName(name));
    let set_description = builder_dispatch.apply_callback(|description: Option<String>| TeamActions::SetDescription(description));
    let set_icon = builder_dispatch.apply_callback(|icon: Option<FontAwesomeIcon>| TeamActions::SetIcon(icon));
    let set_color = builder_dispatch.apply_callback(|color: Option<Color>| TeamActions::SetColor(color));
    let set_parent_team = builder_dispatch.apply_callback(|parent_team: Option<NestedTeam>| TeamActions::SetParentTeam(parent_team));
    html! {
        <BasicForm<NewTeam>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="Name" optional={false} errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} />
            <BasicInput<String> label="Description" optional={false} errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} />
            <Datalist<FontAwesomeIcon, false> builder={set_icon} optional={false} errors={builder_store.errors_icon.clone()} value={builder_store.icon.clone()} label="Icon" />
            <Datalist<Color, false> builder={set_color} optional={false} errors={builder_store.errors_color.clone()} value={builder_store.color.clone()} label="Color" />
            <Datalist<NestedTeam, true> builder={set_parent_team} optional={true} errors={builder_store.errors_parent_team.clone()} value={builder_store.parent_team.clone()} label="Parent team" />
        </BasicForm<NewTeam>>
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct UpdateTeamFormProp {
    pub id: i32,
}

#[function_component(UpdateTeamForm)]
pub fn update_team_form(props: &UpdateTeamFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<TeamBuilder>();
    // We push the ID of the row to the named requests.
    let props = props.clone();
   named_requests.push(ComponentMessage::get::<UpdateTeam>(props.id.into()));
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| TeamActions::SetName(name));
    let set_description = builder_dispatch.apply_callback(|description: Option<String>| TeamActions::SetDescription(description));
    let set_icon = builder_dispatch.apply_callback(|icon: Option<FontAwesomeIcon>| TeamActions::SetIcon(icon));
    let set_color = builder_dispatch.apply_callback(|color: Option<Color>| TeamActions::SetColor(color));
    let set_parent_team = builder_dispatch.apply_callback(|parent_team: Option<NestedTeam>| TeamActions::SetParentTeam(parent_team));
    html! {
        <BasicForm<UpdateTeam>
            method={FormMethod::PUT}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="Name" optional={false} errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} />
            <BasicInput<String> label="Description" optional={false} errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} />
            <Datalist<FontAwesomeIcon, false> builder={set_icon} optional={false} errors={builder_store.errors_icon.clone()} value={builder_store.icon.clone()} label="Icon" />
            <Datalist<Color, false> builder={set_color} optional={false} errors={builder_store.errors_color.clone()} value={builder_store.color.clone()} label="Color" />
            <Datalist<NestedTeam, true> builder={set_parent_team} optional={true} errors={builder_store.errors_parent_team.clone()} value={builder_store.parent_team.clone()} label="Parent team" />
        </BasicForm<UpdateTeam>>
    }
}
#[derive(Store, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct TeamsTeamsRoleInvitationBuilder {
    pub table: Option<NestedTeam>,
    pub team: Option<NestedTeam>,
    pub role: Option<NestedRole>,
    pub errors_table: Vec<ApiError>,
    pub errors_team: Vec<ApiError>,
    pub errors_role: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

impl Default for TeamsTeamsRoleInvitationBuilder {
    fn default() -> Self {
        Self {
            table: Default::default(),
            team: Default::default(),
            role: Default::default(),
            errors_table: Default::default(),
            errors_team: Default::default(),
            errors_role: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum TeamsTeamsRoleInvitationActions {
    SetTable(Option<NestedTeam>),
    SetTeam(Option<NestedTeam>),
    SetRole(Option<NestedRole>),
}

impl FromOperation for TeamsTeamsRoleInvitationActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "table" => TeamsTeamsRoleInvitationActions::SetTable(Some(bincode::deserialize(&row).unwrap())),
            "team" => TeamsTeamsRoleInvitationActions::SetTeam(Some(bincode::deserialize(&row).unwrap())),
            "role" => TeamsTeamsRoleInvitationActions::SetRole(Some(bincode::deserialize(&row).unwrap())),
            operation_name => unreachable!("The operation name '{}' is not supported.", operation_name),
        }
    }
}

impl Reducer<TeamsTeamsRoleInvitationBuilder> for TeamsTeamsRoleInvitationActions {
    fn apply(self, mut state: std::rc::Rc<TeamsTeamsRoleInvitationBuilder>) -> std::rc::Rc<TeamsTeamsRoleInvitationBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            TeamsTeamsRoleInvitationActions::SetTable(table) => 'table: {
                state_mut.errors_table.clear();
        if table.is_none() {
            state_mut.errors_table.push(ApiError::BadRequest(vec![
                "The Table field is required.".to_string()
             ]));
            state_mut.table = None;
             break 'table;
        }
                state_mut.table = table;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'table;
            }
            TeamsTeamsRoleInvitationActions::SetTeam(team) => 'team: {
                state_mut.errors_team.clear();
        if team.is_none() {
            state_mut.errors_team.push(ApiError::BadRequest(vec![
                "The Team field is required.".to_string()
             ]));
            state_mut.team = None;
             break 'team;
        }
                state_mut.team = team;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'team;
            }
            TeamsTeamsRoleInvitationActions::SetRole(role) => 'role: {
                state_mut.errors_role.clear();
        if role.is_none() {
            state_mut.errors_role.push(ApiError::BadRequest(vec![
                "The Role field is required.".to_string()
             ]));
            state_mut.role = None;
             break 'role;
        }
                state_mut.role = role;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'role;
            }
        }
        state
    }
}
impl FormBuilder for TeamsTeamsRoleInvitationBuilder {
    type Actions = TeamsTeamsRoleInvitationActions;

    type RichVariant = NestedTeamsTeamsRoleInvitation;

    fn has_errors(&self) -> bool {
!self.errors_table.is_empty() || !self.errors_team.is_empty() || !self.errors_role.is_empty()
    }

    fn update(dispatcher: &Dispatch<Self>, richest_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.apply(TeamsTeamsRoleInvitationActions::SetTable(Some(richest_variant.table)));
        dispatcher.apply(TeamsTeamsRoleInvitationActions::SetTeam(Some(richest_variant.team)));
        dispatcher.apply(TeamsTeamsRoleInvitationActions::SetRole(Some(richest_variant.role)));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.table.is_some()
        && self.team.is_some()
        && self.role.is_some()
    }

}

impl From<TeamsTeamsRoleInvitationBuilder> for NewTeamsTeamsRoleInvitation {
    fn from(builder: TeamsTeamsRoleInvitationBuilder) -> Self {
        Self {
            table_id: builder.table.as_ref().map(|table| table.inner.id).unwrap(),
            team_id: builder.team.as_ref().map(|team| team.inner.id).unwrap(),
            role_id: builder.role.unwrap().inner.id,
        }
    }
}
impl FormBuildable for NewTeamsTeamsRoleInvitation {
    type Builder = TeamsTeamsRoleInvitationBuilder;
    fn title() -> &'static str {
        "Teams teams role invitation"
    }
    fn task_target() -> &'static str {
        "Teams teams role invitation"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateTeamsTeamsRoleInvitationFormProp {
     #[prop_or_default]
    pub table_id: Option<i32>,
     #[prop_or_default]
    pub team_id: Option<i32>,
     #[prop_or_default]
    pub role_id: Option<i32>,
}

#[function_component(CreateTeamsTeamsRoleInvitationForm)]
pub fn create_teams_teams_role_invitation_form(props: &CreateTeamsTeamsRoleInvitationFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<TeamsTeamsRoleInvitationBuilder>();
   if let Some(table_id) = props.table_id {
         named_requests.push(ComponentMessage::get_named::<&str, Team>("table", table_id.into()));
    }
   if let Some(team_id) = props.team_id {
         named_requests.push(ComponentMessage::get_named::<&str, Team>("team", team_id.into()));
    }
   if let Some(role_id) = props.role_id {
         named_requests.push(ComponentMessage::get_named::<&str, Role>("role", role_id.into()));
    }
    let set_table = builder_dispatch.apply_callback(|table: Option<NestedTeam>| TeamsTeamsRoleInvitationActions::SetTable(table));
    let set_team = builder_dispatch.apply_callback(|team: Option<NestedTeam>| TeamsTeamsRoleInvitationActions::SetTeam(team));
    let set_role = builder_dispatch.apply_callback(|role: Option<NestedRole>| TeamsTeamsRoleInvitationActions::SetRole(role));
    html! {
        <BasicForm<NewTeamsTeamsRoleInvitation>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <Datalist<NestedTeam, true> builder={set_table} optional={false} errors={builder_store.errors_table.clone()} value={builder_store.table.clone()} label="Table" />
            <Datalist<NestedTeam, true> builder={set_team} optional={false} errors={builder_store.errors_team.clone()} value={builder_store.team.clone()} label="Team" />
            <Datalist<NestedRole, false> builder={set_role} optional={false} errors={builder_store.errors_role.clone()} value={builder_store.role.clone()} label="Role" />
        </BasicForm<NewTeamsTeamsRoleInvitation>>
    }
}
#[derive(Store, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct TeamsUsersRoleInvitationBuilder {
    pub table: Option<NestedTeam>,
    pub user: Option<User>,
    pub role: Option<NestedRole>,
    pub errors_table: Vec<ApiError>,
    pub errors_user: Vec<ApiError>,
    pub errors_role: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

impl Default for TeamsUsersRoleInvitationBuilder {
    fn default() -> Self {
        Self {
            table: Default::default(),
            user: Default::default(),
            role: Default::default(),
            errors_table: Default::default(),
            errors_user: Default::default(),
            errors_role: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum TeamsUsersRoleInvitationActions {
    SetTable(Option<NestedTeam>),
    SetUser(Option<User>),
    SetRole(Option<NestedRole>),
}

impl FromOperation for TeamsUsersRoleInvitationActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "table" => TeamsUsersRoleInvitationActions::SetTable(Some(bincode::deserialize(&row).unwrap())),
            "user" => TeamsUsersRoleInvitationActions::SetUser(Some(bincode::deserialize(&row).unwrap())),
            "role" => TeamsUsersRoleInvitationActions::SetRole(Some(bincode::deserialize(&row).unwrap())),
            operation_name => unreachable!("The operation name '{}' is not supported.", operation_name),
        }
    }
}

impl Reducer<TeamsUsersRoleInvitationBuilder> for TeamsUsersRoleInvitationActions {
    fn apply(self, mut state: std::rc::Rc<TeamsUsersRoleInvitationBuilder>) -> std::rc::Rc<TeamsUsersRoleInvitationBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            TeamsUsersRoleInvitationActions::SetTable(table) => 'table: {
                state_mut.errors_table.clear();
        if table.is_none() {
            state_mut.errors_table.push(ApiError::BadRequest(vec![
                "The Table field is required.".to_string()
             ]));
            state_mut.table = None;
             break 'table;
        }
                state_mut.table = table;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'table;
            }
            TeamsUsersRoleInvitationActions::SetUser(user) => 'user: {
                state_mut.errors_user.clear();
        if user.is_none() {
            state_mut.errors_user.push(ApiError::BadRequest(vec![
                "The User field is required.".to_string()
             ]));
            state_mut.user = None;
             break 'user;
        }
                state_mut.user = user;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'user;
            }
            TeamsUsersRoleInvitationActions::SetRole(role) => 'role: {
                state_mut.errors_role.clear();
        if role.is_none() {
            state_mut.errors_role.push(ApiError::BadRequest(vec![
                "The Role field is required.".to_string()
             ]));
            state_mut.role = None;
             break 'role;
        }
                state_mut.role = role;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'role;
            }
        }
        state
    }
}
impl FormBuilder for TeamsUsersRoleInvitationBuilder {
    type Actions = TeamsUsersRoleInvitationActions;

    type RichVariant = NestedTeamsUsersRoleInvitation;

    fn has_errors(&self) -> bool {
!self.errors_table.is_empty() || !self.errors_user.is_empty() || !self.errors_role.is_empty()
    }

    fn update(dispatcher: &Dispatch<Self>, richest_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.apply(TeamsUsersRoleInvitationActions::SetTable(Some(richest_variant.table)));
        dispatcher.apply(TeamsUsersRoleInvitationActions::SetUser(Some(richest_variant.user)));
        dispatcher.apply(TeamsUsersRoleInvitationActions::SetRole(Some(richest_variant.role)));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.table.is_some()
        && self.user.is_some()
        && self.role.is_some()
    }

}

impl From<TeamsUsersRoleInvitationBuilder> for NewTeamsUsersRoleInvitation {
    fn from(builder: TeamsUsersRoleInvitationBuilder) -> Self {
        Self {
            table_id: builder.table.as_ref().map(|table| table.inner.id).unwrap(),
            user_id: builder.user.as_ref().map(|user| user.id).unwrap(),
            role_id: builder.role.unwrap().inner.id,
        }
    }
}
impl FormBuildable for NewTeamsUsersRoleInvitation {
    type Builder = TeamsUsersRoleInvitationBuilder;
    fn title() -> &'static str {
        "Teams users role invitation"
    }
    fn task_target() -> &'static str {
        "Teams users role invitation"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateTeamsUsersRoleInvitationFormProp {
     #[prop_or_default]
    pub table_id: Option<i32>,
     #[prop_or_default]
    pub user_id: Option<i32>,
     #[prop_or_default]
    pub role_id: Option<i32>,
}

#[function_component(CreateTeamsUsersRoleInvitationForm)]
pub fn create_teams_users_role_invitation_form(props: &CreateTeamsUsersRoleInvitationFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<TeamsUsersRoleInvitationBuilder>();
   if let Some(table_id) = props.table_id {
         named_requests.push(ComponentMessage::get_named::<&str, Team>("table", table_id.into()));
    }
   if let Some(user_id) = props.user_id {
         named_requests.push(ComponentMessage::get_named::<&str, User>("user", user_id.into()));
    }
   if let Some(role_id) = props.role_id {
         named_requests.push(ComponentMessage::get_named::<&str, Role>("role", role_id.into()));
    }
    let set_table = builder_dispatch.apply_callback(|table: Option<NestedTeam>| TeamsUsersRoleInvitationActions::SetTable(table));
    let set_user = builder_dispatch.apply_callback(|user: Option<User>| TeamsUsersRoleInvitationActions::SetUser(user));
    let set_role = builder_dispatch.apply_callback(|role: Option<NestedRole>| TeamsUsersRoleInvitationActions::SetRole(role));
    html! {
        <BasicForm<NewTeamsUsersRoleInvitation>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <Datalist<NestedTeam, true> builder={set_table} optional={false} errors={builder_store.errors_table.clone()} value={builder_store.table.clone()} label="Table" />
            <Datalist<User, false> builder={set_user} optional={false} errors={builder_store.errors_user.clone()} value={builder_store.user.clone()} label="User" />
            <Datalist<NestedRole, false> builder={set_role} optional={false} errors={builder_store.errors_role.clone()} value={builder_store.role.clone()} label="Role" />
        </BasicForm<NewTeamsUsersRoleInvitation>>
    }
}
#[derive(Store, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct TeamsUsersRoleRequestBuilder {
    pub table: Option<NestedTeam>,
    pub user: Option<User>,
    pub role: Option<NestedRole>,
    pub errors_table: Vec<ApiError>,
    pub errors_user: Vec<ApiError>,
    pub errors_role: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

impl Default for TeamsUsersRoleRequestBuilder {
    fn default() -> Self {
        Self {
            table: Default::default(),
            user: Default::default(),
            role: Default::default(),
            errors_table: Default::default(),
            errors_user: Default::default(),
            errors_role: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum TeamsUsersRoleRequestActions {
    SetTable(Option<NestedTeam>),
    SetUser(Option<User>),
    SetRole(Option<NestedRole>),
}

impl FromOperation for TeamsUsersRoleRequestActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "table" => TeamsUsersRoleRequestActions::SetTable(Some(bincode::deserialize(&row).unwrap())),
            "user" => TeamsUsersRoleRequestActions::SetUser(Some(bincode::deserialize(&row).unwrap())),
            "role" => TeamsUsersRoleRequestActions::SetRole(Some(bincode::deserialize(&row).unwrap())),
            operation_name => unreachable!("The operation name '{}' is not supported.", operation_name),
        }
    }
}

impl Reducer<TeamsUsersRoleRequestBuilder> for TeamsUsersRoleRequestActions {
    fn apply(self, mut state: std::rc::Rc<TeamsUsersRoleRequestBuilder>) -> std::rc::Rc<TeamsUsersRoleRequestBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            TeamsUsersRoleRequestActions::SetTable(table) => 'table: {
                state_mut.errors_table.clear();
        if table.is_none() {
            state_mut.errors_table.push(ApiError::BadRequest(vec![
                "The Table field is required.".to_string()
             ]));
            state_mut.table = None;
             break 'table;
        }
                state_mut.table = table;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'table;
            }
            TeamsUsersRoleRequestActions::SetUser(user) => 'user: {
                state_mut.errors_user.clear();
        if user.is_none() {
            state_mut.errors_user.push(ApiError::BadRequest(vec![
                "The User field is required.".to_string()
             ]));
            state_mut.user = None;
             break 'user;
        }
                state_mut.user = user;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'user;
            }
            TeamsUsersRoleRequestActions::SetRole(role) => 'role: {
                state_mut.errors_role.clear();
        if role.is_none() {
            state_mut.errors_role.push(ApiError::BadRequest(vec![
                "The Role field is required.".to_string()
             ]));
            state_mut.role = None;
             break 'role;
        }
                state_mut.role = role;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'role;
            }
        }
        state
    }
}
impl FormBuilder for TeamsUsersRoleRequestBuilder {
    type Actions = TeamsUsersRoleRequestActions;

    type RichVariant = NestedTeamsUsersRoleRequest;

    fn has_errors(&self) -> bool {
!self.errors_table.is_empty() || !self.errors_user.is_empty() || !self.errors_role.is_empty()
    }

    fn update(dispatcher: &Dispatch<Self>, richest_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.apply(TeamsUsersRoleRequestActions::SetTable(Some(richest_variant.table)));
        dispatcher.apply(TeamsUsersRoleRequestActions::SetUser(Some(richest_variant.user)));
        dispatcher.apply(TeamsUsersRoleRequestActions::SetRole(Some(richest_variant.role)));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.table.is_some()
        && self.user.is_some()
        && self.role.is_some()
    }

}

impl From<TeamsUsersRoleRequestBuilder> for NewTeamsUsersRoleRequest {
    fn from(builder: TeamsUsersRoleRequestBuilder) -> Self {
        Self {
            table_id: builder.table.as_ref().map(|table| table.inner.id).unwrap(),
            user_id: builder.user.as_ref().map(|user| user.id).unwrap(),
            role_id: builder.role.unwrap().inner.id,
        }
    }
}
impl FormBuildable for NewTeamsUsersRoleRequest {
    type Builder = TeamsUsersRoleRequestBuilder;
    fn title() -> &'static str {
        "Teams users role request"
    }
    fn task_target() -> &'static str {
        "Teams users role request"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateTeamsUsersRoleRequestFormProp {
     #[prop_or_default]
    pub table_id: Option<i32>,
     #[prop_or_default]
    pub user_id: Option<i32>,
     #[prop_or_default]
    pub role_id: Option<i32>,
}

#[function_component(CreateTeamsUsersRoleRequestForm)]
pub fn create_teams_users_role_request_form(props: &CreateTeamsUsersRoleRequestFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<TeamsUsersRoleRequestBuilder>();
   if let Some(table_id) = props.table_id {
         named_requests.push(ComponentMessage::get_named::<&str, Team>("table", table_id.into()));
    }
   if let Some(user_id) = props.user_id {
         named_requests.push(ComponentMessage::get_named::<&str, User>("user", user_id.into()));
    }
   if let Some(role_id) = props.role_id {
         named_requests.push(ComponentMessage::get_named::<&str, Role>("role", role_id.into()));
    }
    let set_table = builder_dispatch.apply_callback(|table: Option<NestedTeam>| TeamsUsersRoleRequestActions::SetTable(table));
    let set_user = builder_dispatch.apply_callback(|user: Option<User>| TeamsUsersRoleRequestActions::SetUser(user));
    let set_role = builder_dispatch.apply_callback(|role: Option<NestedRole>| TeamsUsersRoleRequestActions::SetRole(role));
    html! {
        <BasicForm<NewTeamsUsersRoleRequest>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <Datalist<NestedTeam, true> builder={set_table} optional={false} errors={builder_store.errors_table.clone()} value={builder_store.table.clone()} label="Table" />
            <Datalist<User, false> builder={set_user} optional={false} errors={builder_store.errors_user.clone()} value={builder_store.user.clone()} label="User" />
            <Datalist<NestedRole, false> builder={set_role} optional={false} errors={builder_store.errors_role.clone()} value={builder_store.role.clone()} label="Role" />
        </BasicForm<NewTeamsUsersRoleRequest>>
    }
}
#[derive(Store, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct TeamsUsersRoleBuilder {
    pub table: Option<NestedTeam>,
    pub user: Option<User>,
    pub role: Option<NestedRole>,
    pub errors_table: Vec<ApiError>,
    pub errors_user: Vec<ApiError>,
    pub errors_role: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

impl Default for TeamsUsersRoleBuilder {
    fn default() -> Self {
        Self {
            table: Default::default(),
            user: Default::default(),
            role: Default::default(),
            errors_table: Default::default(),
            errors_user: Default::default(),
            errors_role: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum TeamsUsersRoleActions {
    SetTable(Option<NestedTeam>),
    SetUser(Option<User>),
    SetRole(Option<NestedRole>),
}

impl FromOperation for TeamsUsersRoleActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "table" => TeamsUsersRoleActions::SetTable(Some(bincode::deserialize(&row).unwrap())),
            "user" => TeamsUsersRoleActions::SetUser(Some(bincode::deserialize(&row).unwrap())),
            "role" => TeamsUsersRoleActions::SetRole(Some(bincode::deserialize(&row).unwrap())),
            operation_name => unreachable!("The operation name '{}' is not supported.", operation_name),
        }
    }
}

impl Reducer<TeamsUsersRoleBuilder> for TeamsUsersRoleActions {
    fn apply(self, mut state: std::rc::Rc<TeamsUsersRoleBuilder>) -> std::rc::Rc<TeamsUsersRoleBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            TeamsUsersRoleActions::SetTable(table) => 'table: {
                state_mut.errors_table.clear();
        if table.is_none() {
            state_mut.errors_table.push(ApiError::BadRequest(vec![
                "The Table field is required.".to_string()
             ]));
            state_mut.table = None;
             break 'table;
        }
                state_mut.table = table;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'table;
            }
            TeamsUsersRoleActions::SetUser(user) => 'user: {
                state_mut.errors_user.clear();
        if user.is_none() {
            state_mut.errors_user.push(ApiError::BadRequest(vec![
                "The User field is required.".to_string()
             ]));
            state_mut.user = None;
             break 'user;
        }
                state_mut.user = user;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'user;
            }
            TeamsUsersRoleActions::SetRole(role) => 'role: {
                state_mut.errors_role.clear();
        if role.is_none() {
            state_mut.errors_role.push(ApiError::BadRequest(vec![
                "The Role field is required.".to_string()
             ]));
            state_mut.role = None;
             break 'role;
        }
                state_mut.role = role;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'role;
            }
        }
        state
    }
}
impl FormBuilder for TeamsUsersRoleBuilder {
    type Actions = TeamsUsersRoleActions;

    type RichVariant = NestedTeamsUsersRole;

    fn has_errors(&self) -> bool {
!self.errors_table.is_empty() || !self.errors_user.is_empty() || !self.errors_role.is_empty()
    }

    fn update(dispatcher: &Dispatch<Self>, richest_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.apply(TeamsUsersRoleActions::SetTable(Some(richest_variant.table)));
        dispatcher.apply(TeamsUsersRoleActions::SetUser(Some(richest_variant.user)));
        dispatcher.apply(TeamsUsersRoleActions::SetRole(Some(richest_variant.role)));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.table.is_some()
        && self.user.is_some()
        && self.role.is_some()
    }

}

impl From<TeamsUsersRoleBuilder> for NewTeamsUsersRole {
    fn from(builder: TeamsUsersRoleBuilder) -> Self {
        Self {
            table_id: builder.table.as_ref().map(|table| table.inner.id).unwrap(),
            user_id: builder.user.as_ref().map(|user| user.id).unwrap(),
            role_id: builder.role.unwrap().inner.id,
        }
    }
}
impl FormBuildable for NewTeamsUsersRole {
    type Builder = TeamsUsersRoleBuilder;
    fn title() -> &'static str {
        "Teams users role"
    }
    fn task_target() -> &'static str {
        "Teams users role"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateTeamsUsersRoleFormProp {
     #[prop_or_default]
    pub table_id: Option<i32>,
     #[prop_or_default]
    pub user_id: Option<i32>,
     #[prop_or_default]
    pub role_id: Option<i32>,
}

#[function_component(CreateTeamsUsersRoleForm)]
pub fn create_teams_users_role_form(props: &CreateTeamsUsersRoleFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<TeamsUsersRoleBuilder>();
   if let Some(table_id) = props.table_id {
         named_requests.push(ComponentMessage::get_named::<&str, Team>("table", table_id.into()));
    }
   if let Some(user_id) = props.user_id {
         named_requests.push(ComponentMessage::get_named::<&str, User>("user", user_id.into()));
    }
   if let Some(role_id) = props.role_id {
         named_requests.push(ComponentMessage::get_named::<&str, Role>("role", role_id.into()));
    }
    let set_table = builder_dispatch.apply_callback(|table: Option<NestedTeam>| TeamsUsersRoleActions::SetTable(table));
    let set_user = builder_dispatch.apply_callback(|user: Option<User>| TeamsUsersRoleActions::SetUser(user));
    let set_role = builder_dispatch.apply_callback(|role: Option<NestedRole>| TeamsUsersRoleActions::SetRole(role));
    html! {
        <BasicForm<NewTeamsUsersRole>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <Datalist<NestedTeam, true> builder={set_table} optional={false} errors={builder_store.errors_table.clone()} value={builder_store.table.clone()} label="Table" />
            <Datalist<User, false> builder={set_user} optional={false} errors={builder_store.errors_user.clone()} value={builder_store.user.clone()} label="User" />
            <Datalist<NestedRole, false> builder={set_role} optional={false} errors={builder_store.errors_role.clone()} value={builder_store.role.clone()} label="Role" />
        </BasicForm<NewTeamsUsersRole>>
    }
}
#[derive(Store, Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct UserBuilder {
    pub id: Option<i32>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub description: Option<String>,
    pub profile_picture: Option<Vec<u8>>,
    pub errors_first_name: Vec<ApiError>,
    pub errors_middle_name: Vec<ApiError>,
    pub errors_last_name: Vec<ApiError>,
    pub errors_description: Vec<ApiError>,
    pub errors_profile_picture: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

impl Default for UserBuilder {
    fn default() -> Self {
        Self {
            id: None,
            first_name: None,
            middle_name: None,
            last_name: None,
            description: None,
            profile_picture: None,
            errors_first_name: Default::default(),
            errors_middle_name: Default::default(),
            errors_last_name: Default::default(),
            errors_description: Default::default(),
            errors_profile_picture: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub(super) enum UserActions {
    SetFirstName(Option<String>),
    SetMiddleName(Option<String>),
    SetLastName(Option<String>),
    SetDescription(Option<String>),
    SetProfilePicture(Option<Vec<u8>>),
}

impl FromOperation for UserActions {
    fn from_operation<S: AsRef<str>>(_operation: S, _row: Vec<u8>) -> Self {
        unreachable!("No operations are expected to be needed for the builder UserBuilder.")
    }
}

impl Reducer<UserBuilder> for UserActions {
    fn apply(self, mut state: std::rc::Rc<UserBuilder>) -> std::rc::Rc<UserBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            UserActions::SetFirstName(first_name) => 'first_name: {
                state_mut.errors_first_name.clear();
        if first_name.is_none() {
            state_mut.errors_first_name.push(ApiError::BadRequest(vec![
                "The First name field is required.".to_string()
             ]));
            state_mut.first_name = None;
             break 'first_name;
        }
                if let Some(value) = first_name.as_ref() {
                    if value.is_empty() {
                        state_mut.errors_first_name.push(ApiError::BadRequest(vec![
                            "The First name field cannot be left empty.".to_string()
                        ]));
                         state_mut.first_name = None;
                          break 'first_name;
                    }
                }
                state_mut.first_name = first_name;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'first_name;
            }
            UserActions::SetMiddleName(middle_name) => 'middle_name: {
                state_mut.errors_middle_name.clear();
                if let Some(value) = middle_name.as_ref() {
                    if value.is_empty() {
                        state_mut.errors_middle_name.push(ApiError::BadRequest(vec![
                            "The Middle name field cannot be left empty.".to_string()
                        ]));
                         state_mut.middle_name = None;
                          break 'middle_name;
                    }
                }
                state_mut.middle_name = middle_name;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'middle_name;
            }
            UserActions::SetLastName(last_name) => 'last_name: {
                state_mut.errors_last_name.clear();
        if last_name.is_none() {
            state_mut.errors_last_name.push(ApiError::BadRequest(vec![
                "The Last name field is required.".to_string()
             ]));
            state_mut.last_name = None;
             break 'last_name;
        }
                if let Some(value) = last_name.as_ref() {
                    if value.is_empty() {
                        state_mut.errors_last_name.push(ApiError::BadRequest(vec![
                            "The Last name field cannot be left empty.".to_string()
                        ]));
                         state_mut.last_name = None;
                          break 'last_name;
                    }
                }
                state_mut.last_name = last_name;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'last_name;
            }
            UserActions::SetDescription(description) => 'description: {
                state_mut.errors_description.clear();
                if let Some(value) = description.as_ref() {
                    if value.is_empty() {
                        state_mut.errors_description.push(ApiError::BadRequest(vec![
                            "The Description field cannot be left empty.".to_string()
                        ]));
                         state_mut.description = None;
                          break 'description;
                    }
                }
                state_mut.description = description;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'description;
            }
            UserActions::SetProfilePicture(profile_picture) => 'profile_picture: {
                state_mut.errors_profile_picture.clear();
        if profile_picture.is_none() {
            state_mut.errors_profile_picture.push(ApiError::BadRequest(vec![
                "The Profile picture field is required.".to_string()
             ]));
            state_mut.profile_picture = None;
             break 'profile_picture;
        }
                state_mut.profile_picture = profile_picture;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'profile_picture;
            }
        }
        state
    }
}
impl FormBuilder for UserBuilder {
    type Actions = UserActions;

    type RichVariant = User;

    fn has_errors(&self) -> bool {
!self.errors_first_name.is_empty() || !self.errors_middle_name.is_empty() || !self.errors_last_name.is_empty() || !self.errors_description.is_empty() || !self.errors_profile_picture.is_empty()
    }

    fn update(dispatcher: &Dispatch<Self>, richest_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.reduce_mut(|state| {state.id = Some(richest_variant.id);});
        dispatcher.apply(UserActions::SetFirstName(Some(richest_variant.first_name)));
        dispatcher.apply(UserActions::SetMiddleName(richest_variant.middle_name));
        dispatcher.apply(UserActions::SetLastName(Some(richest_variant.last_name)));
        dispatcher.apply(UserActions::SetDescription(richest_variant.description));
        dispatcher.apply(UserActions::SetProfilePicture(Some(richest_variant.profile_picture)));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.first_name.is_some()
        && self.last_name.is_some()
        && self.profile_picture.is_some()
    }

}

impl From<UserBuilder> for NewUser {
    fn from(builder: UserBuilder) -> Self {
        Self {
            first_name: builder.first_name.unwrap(),
            middle_name: builder.middle_name,
            last_name: builder.last_name.unwrap(),
            description: builder.description,
            profile_picture: builder.profile_picture.unwrap(),
        }
    }
}
impl From<UserBuilder> for UpdateUser {
    fn from(builder: UserBuilder) -> Self {
        Self {
            id: builder.id.unwrap(),
            first_name: builder.first_name.unwrap(),
            middle_name: builder.middle_name,
            last_name: builder.last_name.unwrap(),
            description: builder.description,
            profile_picture: builder.profile_picture.unwrap(),
        }
    }
}
impl FormBuildable for NewUser {
    type Builder = UserBuilder;
    fn title() -> &'static str {
        "User"
    }
    fn task_target() -> &'static str {
        "User"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

impl FormBuildable for UpdateUser {
    type Builder = UserBuilder;
    fn title() -> &'static str {
        "User"
    }
    fn task_target() -> &'static str {
        "User"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        true
    }
}

#[function_component(CreateUserForm)]
pub fn create_user_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<UserBuilder>();
    let set_first_name = builder_dispatch.apply_callback(|first_name: Option<String>| UserActions::SetFirstName(first_name));
    let set_middle_name = builder_dispatch.apply_callback(|middle_name: Option<String>| UserActions::SetMiddleName(middle_name));
    let set_last_name = builder_dispatch.apply_callback(|last_name: Option<String>| UserActions::SetLastName(last_name));
    let set_description = builder_dispatch.apply_callback(|description: Option<String>| UserActions::SetDescription(description));
    let set_profile_picture = builder_dispatch.apply_callback(|profile_picture: Option<Image>| UserActions::SetProfilePicture(profile_picture.map(|profile_picture| profile_picture.into())));
    html! {
        <BasicForm<NewUser>
            method={FormMethod::POST}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="First name" optional={false} errors={builder_store.errors_first_name.clone()} builder={set_first_name} value={builder_store.first_name.clone()} />
            <BasicInput<String> label="Middle name" optional={true} errors={builder_store.errors_middle_name.clone()} builder={set_middle_name} value={builder_store.middle_name.clone()} />
            <BasicInput<String> label="Last name" optional={false} errors={builder_store.errors_last_name.clone()} builder={set_last_name} value={builder_store.last_name.clone()} />
            <BasicInput<String> label="Description" optional={true} errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} />
            <FileInput<Image> label="Profile picture" optional={false} errors={builder_store.errors_profile_picture.clone()} builder={set_profile_picture} allowed_formats={vec![GenericFileFormat::Image]} value={builder_store.profile_picture.clone().map(|profile_picture| profile_picture.into())} />
        </BasicForm<NewUser>>
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct UpdateUserFormProp {
    pub id: i32,
}

#[function_component(UpdateUserForm)]
pub fn update_user_form(props: &UpdateUserFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<UserBuilder>();
    // We push the ID of the row to the named requests.
    let props = props.clone();
   named_requests.push(ComponentMessage::get::<UpdateUser>(props.id.into()));
    let set_first_name = builder_dispatch.apply_callback(|first_name: Option<String>| UserActions::SetFirstName(first_name));
    let set_middle_name = builder_dispatch.apply_callback(|middle_name: Option<String>| UserActions::SetMiddleName(middle_name));
    let set_last_name = builder_dispatch.apply_callback(|last_name: Option<String>| UserActions::SetLastName(last_name));
    let set_description = builder_dispatch.apply_callback(|description: Option<String>| UserActions::SetDescription(description));
    let set_profile_picture = builder_dispatch.apply_callback(|profile_picture: Option<Image>| UserActions::SetProfilePicture(profile_picture.map(|profile_picture| profile_picture.into())));
    html! {
        <BasicForm<UpdateUser>
            method={FormMethod::PUT}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="First name" optional={false} errors={builder_store.errors_first_name.clone()} builder={set_first_name} value={builder_store.first_name.clone()} />
            <BasicInput<String> label="Middle name" optional={true} errors={builder_store.errors_middle_name.clone()} builder={set_middle_name} value={builder_store.middle_name.clone()} />
            <BasicInput<String> label="Last name" optional={false} errors={builder_store.errors_last_name.clone()} builder={set_last_name} value={builder_store.last_name.clone()} />
            <BasicInput<String> label="Description" optional={true} errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} />
            <FileInput<Image> label="Profile picture" optional={false} errors={builder_store.errors_profile_picture.clone()} builder={set_profile_picture} allowed_formats={vec![GenericFileFormat::Image]} value={builder_store.profile_picture.clone().map(|profile_picture| profile_picture.into())} />
        </BasicForm<UpdateUser>>
    }
}
#[derive(Store, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct UsersUsersRoleInvitationBuilder {
    pub table: Option<User>,
    pub user: Option<User>,
    pub role: Option<NestedRole>,
    pub errors_table: Vec<ApiError>,
    pub errors_user: Vec<ApiError>,
    pub errors_role: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

impl Default for UsersUsersRoleInvitationBuilder {
    fn default() -> Self {
        Self {
            table: Default::default(),
            user: Default::default(),
            role: Default::default(),
            errors_table: Default::default(),
            errors_user: Default::default(),
            errors_role: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum UsersUsersRoleInvitationActions {
    SetTable(Option<User>),
    SetUser(Option<User>),
    SetRole(Option<NestedRole>),
}

impl FromOperation for UsersUsersRoleInvitationActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "table" => UsersUsersRoleInvitationActions::SetTable(Some(bincode::deserialize(&row).unwrap())),
            "user" => UsersUsersRoleInvitationActions::SetUser(Some(bincode::deserialize(&row).unwrap())),
            "role" => UsersUsersRoleInvitationActions::SetRole(Some(bincode::deserialize(&row).unwrap())),
            operation_name => unreachable!("The operation name '{}' is not supported.", operation_name),
        }
    }
}

impl Reducer<UsersUsersRoleInvitationBuilder> for UsersUsersRoleInvitationActions {
    fn apply(self, mut state: std::rc::Rc<UsersUsersRoleInvitationBuilder>) -> std::rc::Rc<UsersUsersRoleInvitationBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            UsersUsersRoleInvitationActions::SetTable(table) => 'table: {
                state_mut.errors_table.clear();
        if table.is_none() {
            state_mut.errors_table.push(ApiError::BadRequest(vec![
                "The Table field is required.".to_string()
             ]));
            state_mut.table = None;
             break 'table;
        }
                state_mut.table = table;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'table;
            }
            UsersUsersRoleInvitationActions::SetUser(user) => 'user: {
                state_mut.errors_user.clear();
        if user.is_none() {
            state_mut.errors_user.push(ApiError::BadRequest(vec![
                "The User field is required.".to_string()
             ]));
            state_mut.user = None;
             break 'user;
        }
                state_mut.user = user;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'user;
            }
            UsersUsersRoleInvitationActions::SetRole(role) => 'role: {
                state_mut.errors_role.clear();
        if role.is_none() {
            state_mut.errors_role.push(ApiError::BadRequest(vec![
                "The Role field is required.".to_string()
             ]));
            state_mut.role = None;
             break 'role;
        }
                state_mut.role = role;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'role;
            }
        }
        state
    }
}
impl FormBuilder for UsersUsersRoleInvitationBuilder {
    type Actions = UsersUsersRoleInvitationActions;

    type RichVariant = NestedUsersUsersRoleInvitation;

    fn has_errors(&self) -> bool {
!self.errors_table.is_empty() || !self.errors_user.is_empty() || !self.errors_role.is_empty()
    }

    fn update(dispatcher: &Dispatch<Self>, richest_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.apply(UsersUsersRoleInvitationActions::SetTable(Some(richest_variant.table)));
        dispatcher.apply(UsersUsersRoleInvitationActions::SetUser(Some(richest_variant.user)));
        dispatcher.apply(UsersUsersRoleInvitationActions::SetRole(Some(richest_variant.role)));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.table.is_some()
        && self.user.is_some()
        && self.role.is_some()
    }

}

impl From<UsersUsersRoleInvitationBuilder> for NewUsersUsersRoleInvitation {
    fn from(builder: UsersUsersRoleInvitationBuilder) -> Self {
        Self {
            table_id: builder.table.as_ref().map(|table| table.id).unwrap(),
            user_id: builder.user.as_ref().map(|user| user.id).unwrap(),
            role_id: builder.role.unwrap().inner.id,
        }
    }
}
impl FormBuildable for NewUsersUsersRoleInvitation {
    type Builder = UsersUsersRoleInvitationBuilder;
    fn title() -> &'static str {
        "Users users role invitation"
    }
    fn task_target() -> &'static str {
        "Users users role invitation"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateUsersUsersRoleInvitationFormProp {
     #[prop_or_default]
    pub table_id: Option<i32>,
     #[prop_or_default]
    pub user_id: Option<i32>,
     #[prop_or_default]
    pub role_id: Option<i32>,
}

#[function_component(CreateUsersUsersRoleInvitationForm)]
pub fn create_users_users_role_invitation_form(props: &CreateUsersUsersRoleInvitationFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<UsersUsersRoleInvitationBuilder>();
   if let Some(table_id) = props.table_id {
         named_requests.push(ComponentMessage::get_named::<&str, User>("table", table_id.into()));
    }
   if let Some(user_id) = props.user_id {
         named_requests.push(ComponentMessage::get_named::<&str, User>("user", user_id.into()));
    }
   if let Some(role_id) = props.role_id {
         named_requests.push(ComponentMessage::get_named::<&str, Role>("role", role_id.into()));
    }
    let set_table = builder_dispatch.apply_callback(|table: Option<User>| UsersUsersRoleInvitationActions::SetTable(table));
    let set_user = builder_dispatch.apply_callback(|user: Option<User>| UsersUsersRoleInvitationActions::SetUser(user));
    let set_role = builder_dispatch.apply_callback(|role: Option<NestedRole>| UsersUsersRoleInvitationActions::SetRole(role));
    html! {
        <BasicForm<NewUsersUsersRoleInvitation>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <Datalist<User, false> builder={set_table} optional={false} errors={builder_store.errors_table.clone()} value={builder_store.table.clone()} label="Table" />
            <Datalist<User, false> builder={set_user} optional={false} errors={builder_store.errors_user.clone()} value={builder_store.user.clone()} label="User" />
            <Datalist<NestedRole, false> builder={set_role} optional={false} errors={builder_store.errors_role.clone()} value={builder_store.role.clone()} label="Role" />
        </BasicForm<NewUsersUsersRoleInvitation>>
    }
}
#[derive(Store, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct UsersUsersRoleRequestBuilder {
    pub table: Option<User>,
    pub user: Option<User>,
    pub role: Option<NestedRole>,
    pub errors_table: Vec<ApiError>,
    pub errors_user: Vec<ApiError>,
    pub errors_role: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

impl Default for UsersUsersRoleRequestBuilder {
    fn default() -> Self {
        Self {
            table: Default::default(),
            user: Default::default(),
            role: Default::default(),
            errors_table: Default::default(),
            errors_user: Default::default(),
            errors_role: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum UsersUsersRoleRequestActions {
    SetTable(Option<User>),
    SetUser(Option<User>),
    SetRole(Option<NestedRole>),
}

impl FromOperation for UsersUsersRoleRequestActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "table" => UsersUsersRoleRequestActions::SetTable(Some(bincode::deserialize(&row).unwrap())),
            "user" => UsersUsersRoleRequestActions::SetUser(Some(bincode::deserialize(&row).unwrap())),
            "role" => UsersUsersRoleRequestActions::SetRole(Some(bincode::deserialize(&row).unwrap())),
            operation_name => unreachable!("The operation name '{}' is not supported.", operation_name),
        }
    }
}

impl Reducer<UsersUsersRoleRequestBuilder> for UsersUsersRoleRequestActions {
    fn apply(self, mut state: std::rc::Rc<UsersUsersRoleRequestBuilder>) -> std::rc::Rc<UsersUsersRoleRequestBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            UsersUsersRoleRequestActions::SetTable(table) => 'table: {
                state_mut.errors_table.clear();
        if table.is_none() {
            state_mut.errors_table.push(ApiError::BadRequest(vec![
                "The Table field is required.".to_string()
             ]));
            state_mut.table = None;
             break 'table;
        }
                state_mut.table = table;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'table;
            }
            UsersUsersRoleRequestActions::SetUser(user) => 'user: {
                state_mut.errors_user.clear();
        if user.is_none() {
            state_mut.errors_user.push(ApiError::BadRequest(vec![
                "The User field is required.".to_string()
             ]));
            state_mut.user = None;
             break 'user;
        }
                state_mut.user = user;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'user;
            }
            UsersUsersRoleRequestActions::SetRole(role) => 'role: {
                state_mut.errors_role.clear();
        if role.is_none() {
            state_mut.errors_role.push(ApiError::BadRequest(vec![
                "The Role field is required.".to_string()
             ]));
            state_mut.role = None;
             break 'role;
        }
                state_mut.role = role;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'role;
            }
        }
        state
    }
}
impl FormBuilder for UsersUsersRoleRequestBuilder {
    type Actions = UsersUsersRoleRequestActions;

    type RichVariant = NestedUsersUsersRoleRequest;

    fn has_errors(&self) -> bool {
!self.errors_table.is_empty() || !self.errors_user.is_empty() || !self.errors_role.is_empty()
    }

    fn update(dispatcher: &Dispatch<Self>, richest_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.apply(UsersUsersRoleRequestActions::SetTable(Some(richest_variant.table)));
        dispatcher.apply(UsersUsersRoleRequestActions::SetUser(Some(richest_variant.user)));
        dispatcher.apply(UsersUsersRoleRequestActions::SetRole(Some(richest_variant.role)));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.table.is_some()
        && self.user.is_some()
        && self.role.is_some()
    }

}

impl From<UsersUsersRoleRequestBuilder> for NewUsersUsersRoleRequest {
    fn from(builder: UsersUsersRoleRequestBuilder) -> Self {
        Self {
            table_id: builder.table.as_ref().map(|table| table.id).unwrap(),
            user_id: builder.user.as_ref().map(|user| user.id).unwrap(),
            role_id: builder.role.unwrap().inner.id,
        }
    }
}
impl FormBuildable for NewUsersUsersRoleRequest {
    type Builder = UsersUsersRoleRequestBuilder;
    fn title() -> &'static str {
        "Users users role request"
    }
    fn task_target() -> &'static str {
        "Users users role request"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateUsersUsersRoleRequestFormProp {
     #[prop_or_default]
    pub table_id: Option<i32>,
     #[prop_or_default]
    pub user_id: Option<i32>,
     #[prop_or_default]
    pub role_id: Option<i32>,
}

#[function_component(CreateUsersUsersRoleRequestForm)]
pub fn create_users_users_role_request_form(props: &CreateUsersUsersRoleRequestFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<UsersUsersRoleRequestBuilder>();
   if let Some(table_id) = props.table_id {
         named_requests.push(ComponentMessage::get_named::<&str, User>("table", table_id.into()));
    }
   if let Some(user_id) = props.user_id {
         named_requests.push(ComponentMessage::get_named::<&str, User>("user", user_id.into()));
    }
   if let Some(role_id) = props.role_id {
         named_requests.push(ComponentMessage::get_named::<&str, Role>("role", role_id.into()));
    }
    let set_table = builder_dispatch.apply_callback(|table: Option<User>| UsersUsersRoleRequestActions::SetTable(table));
    let set_user = builder_dispatch.apply_callback(|user: Option<User>| UsersUsersRoleRequestActions::SetUser(user));
    let set_role = builder_dispatch.apply_callback(|role: Option<NestedRole>| UsersUsersRoleRequestActions::SetRole(role));
    html! {
        <BasicForm<NewUsersUsersRoleRequest>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <Datalist<User, false> builder={set_table} optional={false} errors={builder_store.errors_table.clone()} value={builder_store.table.clone()} label="Table" />
            <Datalist<User, false> builder={set_user} optional={false} errors={builder_store.errors_user.clone()} value={builder_store.user.clone()} label="User" />
            <Datalist<NestedRole, false> builder={set_role} optional={false} errors={builder_store.errors_role.clone()} value={builder_store.role.clone()} label="Role" />
        </BasicForm<NewUsersUsersRoleRequest>>
    }
}
#[derive(Store, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct UsersUsersRoleBuilder {
    pub table: Option<User>,
    pub user: Option<User>,
    pub role: Option<NestedRole>,
    pub errors_table: Vec<ApiError>,
    pub errors_user: Vec<ApiError>,
    pub errors_role: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

impl Default for UsersUsersRoleBuilder {
    fn default() -> Self {
        Self {
            table: Default::default(),
            user: Default::default(),
            role: Default::default(),
            errors_table: Default::default(),
            errors_user: Default::default(),
            errors_role: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum UsersUsersRoleActions {
    SetTable(Option<User>),
    SetUser(Option<User>),
    SetRole(Option<NestedRole>),
}

impl FromOperation for UsersUsersRoleActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "table" => UsersUsersRoleActions::SetTable(Some(bincode::deserialize(&row).unwrap())),
            "user" => UsersUsersRoleActions::SetUser(Some(bincode::deserialize(&row).unwrap())),
            "role" => UsersUsersRoleActions::SetRole(Some(bincode::deserialize(&row).unwrap())),
            operation_name => unreachable!("The operation name '{}' is not supported.", operation_name),
        }
    }
}

impl Reducer<UsersUsersRoleBuilder> for UsersUsersRoleActions {
    fn apply(self, mut state: std::rc::Rc<UsersUsersRoleBuilder>) -> std::rc::Rc<UsersUsersRoleBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            UsersUsersRoleActions::SetTable(table) => 'table: {
                state_mut.errors_table.clear();
        if table.is_none() {
            state_mut.errors_table.push(ApiError::BadRequest(vec![
                "The Table field is required.".to_string()
             ]));
            state_mut.table = None;
             break 'table;
        }
                state_mut.table = table;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'table;
            }
            UsersUsersRoleActions::SetUser(user) => 'user: {
                state_mut.errors_user.clear();
        if user.is_none() {
            state_mut.errors_user.push(ApiError::BadRequest(vec![
                "The User field is required.".to_string()
             ]));
            state_mut.user = None;
             break 'user;
        }
                state_mut.user = user;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'user;
            }
            UsersUsersRoleActions::SetRole(role) => 'role: {
                state_mut.errors_role.clear();
        if role.is_none() {
            state_mut.errors_role.push(ApiError::BadRequest(vec![
                "The Role field is required.".to_string()
             ]));
            state_mut.role = None;
             break 'role;
        }
                state_mut.role = role;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'role;
            }
        }
        state
    }
}
impl FormBuilder for UsersUsersRoleBuilder {
    type Actions = UsersUsersRoleActions;

    type RichVariant = NestedUsersUsersRole;

    fn has_errors(&self) -> bool {
!self.errors_table.is_empty() || !self.errors_user.is_empty() || !self.errors_role.is_empty()
    }

    fn update(dispatcher: &Dispatch<Self>, richest_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.apply(UsersUsersRoleActions::SetTable(Some(richest_variant.table)));
        dispatcher.apply(UsersUsersRoleActions::SetUser(Some(richest_variant.user)));
        dispatcher.apply(UsersUsersRoleActions::SetRole(Some(richest_variant.role)));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.table.is_some()
        && self.user.is_some()
        && self.role.is_some()
    }

}

impl From<UsersUsersRoleBuilder> for NewUsersUsersRole {
    fn from(builder: UsersUsersRoleBuilder) -> Self {
        Self {
            table_id: builder.table.as_ref().map(|table| table.id).unwrap(),
            user_id: builder.user.as_ref().map(|user| user.id).unwrap(),
            role_id: builder.role.unwrap().inner.id,
        }
    }
}
impl FormBuildable for NewUsersUsersRole {
    type Builder = UsersUsersRoleBuilder;
    fn title() -> &'static str {
        "Users users role"
    }
    fn task_target() -> &'static str {
        "Users users role"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateUsersUsersRoleFormProp {
     #[prop_or_default]
    pub table_id: Option<i32>,
     #[prop_or_default]
    pub user_id: Option<i32>,
     #[prop_or_default]
    pub role_id: Option<i32>,
}

#[function_component(CreateUsersUsersRoleForm)]
pub fn create_users_users_role_form(props: &CreateUsersUsersRoleFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<UsersUsersRoleBuilder>();
   if let Some(table_id) = props.table_id {
         named_requests.push(ComponentMessage::get_named::<&str, User>("table", table_id.into()));
    }
   if let Some(user_id) = props.user_id {
         named_requests.push(ComponentMessage::get_named::<&str, User>("user", user_id.into()));
    }
   if let Some(role_id) = props.role_id {
         named_requests.push(ComponentMessage::get_named::<&str, Role>("role", role_id.into()));
    }
    let set_table = builder_dispatch.apply_callback(|table: Option<User>| UsersUsersRoleActions::SetTable(table));
    let set_user = builder_dispatch.apply_callback(|user: Option<User>| UsersUsersRoleActions::SetUser(user));
    let set_role = builder_dispatch.apply_callback(|role: Option<NestedRole>| UsersUsersRoleActions::SetRole(role));
    html! {
        <BasicForm<NewUsersUsersRole>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <Datalist<User, false> builder={set_table} optional={false} errors={builder_store.errors_table.clone()} value={builder_store.table.clone()} label="Table" />
            <Datalist<User, false> builder={set_user} optional={false} errors={builder_store.errors_user.clone()} value={builder_store.user.clone()} label="User" />
            <Datalist<NestedRole, false> builder={set_role} optional={false} errors={builder_store.errors_role.clone()} value={builder_store.role.clone()} label="Role" />
        </BasicForm<NewUsersUsersRole>>
    }
}
