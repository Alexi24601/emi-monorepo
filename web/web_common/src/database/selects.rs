//! Submodule providing select queries, such as `Id` and `Search` queries.

use serde::{Deserialize, Serialize};
use crate::api::ws::messages::FrontendMessage;
use crate::database::Operation;
use crate::database::Task;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Select {
    Id(super::Table, uuid::Uuid),
    SearchTable {
        table: super::Table,
        query: String,
        number_of_results: u32,
    },
}

impl Select {
    pub fn authorizations(&self) -> Vec<super::Authorization> {
        // TODO! SPECIFY IT BETTER!
        Vec::new()
    }

    /// Create a new `Select` query for a given `Table`, query, and number of results.
    /// 
    /// # Arguments
    /// * `table` - The table to select from.
    /// * `query` - The query to search for.
    /// * `number_of_results` - The number of results to return.
    pub fn search(table: super::Table, query: String, number_of_results: u32) -> Self {
        Self::SearchTable {
            table: table.into(),
            query,
            number_of_results,
        }
    }
}

impl From<Select> for super::Operation {
    fn from(select: Select) -> Self {
        Self::Select(select)
    }
}

impl From<Select> for FrontendMessage {
    fn from(select: Select) -> Self {
        let operation: Operation = select.into();
        let task: Task = operation.into();
        task.into()
    }
}