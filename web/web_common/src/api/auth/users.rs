pub mod name;

use crate::combine_path;

pub const ENDPOINT: &str = "/users";
pub const FULL_ENDPOINT: &str = combine_path!(super::FULL_ENDPOINT, ENDPOINT);

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct User {
    name: Option<name::Name>,
    id: Uuid,
}

impl User {
    pub fn new(name: Option<name::Name>, id: Uuid) -> User {
        User { name, id }
    }

    pub fn full_name(&self) -> Result<String, String> {
        match &self.name {
            Some(name) => name.full_name(),
            None => Err("Name is not complete.".to_string()),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn has_complete_profile(&self) -> bool {
        match &self.name {
            Some(name) => name.is_complete(),
            None => false,
        }
    }

    pub fn name(&self) -> Result<name::Name, String> {
        match &self.name {
            Some(name) => Ok(name.clone()),
            None => Err("Name is not complete.".to_string()),
        }
    }

    pub fn first_name(&self) -> Result<String, String> {
        match &self.name {
            Some(name) => Ok(name.first_name().to_string()),
            None => Err("Name is not complete.".to_string()),
        }
    }

    pub fn last_name(&self) -> Result<String, String> {
        match &self.name {
            Some(name) => Ok(name.last_name().to_string()),
            None => Err("Name is not complete.".to_string()),
        }
    }

    pub fn middle_name(&self) -> Option<String> {
        self.name.as_ref().and_then(|name| {
            name.middle_name()
                .map(|middle_name| middle_name.to_string())
        })
    }
}
