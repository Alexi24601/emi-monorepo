//! This file is automatically generated, do not modify it directly.
use super::*;
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub public: bool,
    pub state_id: i32,
    pub icon_id: i32,
    pub color_id: i32,
    pub parent_project_id: Option<i32>,
    pub budget: Option<f64>,
    pub expenses: Option<f64>,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: chrono::NaiveDateTime,
    pub expected_end_date: Option<chrono::NaiveDateTime>,
    pub end_date: Option<chrono::NaiveDateTime>,
}

unsafe impl Send for Project {}
unsafe impl Sync for Project {}
impl Tabular for Project {
    const TABLE: crate::database::Table = crate::database::Table::Projects;
}
impl Describable for Project {
    fn description(&self) -> Option<&str> {
        Some(self.description.as_str())
    }
}
impl Colorable for Project {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for crate::database::flat_variants::Project {
    type Filter = crate::database::filter_variants::ProjectFilter;
}
#[cfg(feature = "frontend")]
impl AllRecords for Project {
    fn all_records<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        filter: Option<&<Self as Filtrable>::Filter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> impl std::future::Future<Output = Result<Vec<Self>, crate::api::ApiError>> {
        Self::all(filter, limit, offset, connection)
    }
}
#[cfg(feature = "frontend")]
impl Project {
    /// Get the id attribute.
    pub fn get_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.id.as_ref()
    }

    /// Get the name attribute.
    pub fn get_name<E>(&self) -> &E
    where
        String: AsRef<E>,
    {
        self.name.as_ref()
    }

    /// Get the description attribute.
    pub fn get_description<E>(&self) -> &E
    where
        String: AsRef<E>,
    {
        self.description.as_ref()
    }

    /// Whether the Project is public.
    pub fn is_public<E>(&self) -> &E
    where
        bool: AsRef<E>,
    {
        self.public.as_ref()
    }

    /// Get the state_id attribute.
    pub fn get_state_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.state_id.as_ref()
    }

    /// Get the icon_id attribute.
    pub fn get_icon_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.icon_id.as_ref()
    }

    /// Get the color_id attribute.
    pub fn get_color_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.color_id.as_ref()
    }

    /// Get the parent_project_id attribute.
    pub fn get_parent_project_id<E>(&self) -> Option<&E>
    where
        i32: AsRef<E>,
    {
        self.parent_project_id.as_ref().map(|value| value.as_ref())
    }

    /// Get the budget attribute.
    pub fn get_budget<E>(&self) -> Option<&E>
    where
        f64: AsRef<E>,
    {
        self.budget.as_ref().map(|value| value.as_ref())
    }

    /// Get the expenses attribute.
    pub fn get_expenses<E>(&self) -> Option<&E>
    where
        f64: AsRef<E>,
    {
        self.expenses.as_ref().map(|value| value.as_ref())
    }

    /// Get the created_by attribute.
    pub fn get_created_by<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.created_by.as_ref()
    }

    /// Get the created_at attribute.
    pub fn get_created_at<E>(&self) -> &E
    where
        chrono::NaiveDateTime: AsRef<E>,
    {
        self.created_at.as_ref()
    }

    /// Get the updated_by attribute.
    pub fn get_updated_by<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.updated_by.as_ref()
    }

    /// Get the updated_at attribute.
    pub fn get_updated_at<E>(&self) -> &E
    where
        chrono::NaiveDateTime: AsRef<E>,
    {
        self.updated_at.as_ref()
    }

    /// Get the expected_end_date attribute.
    pub fn get_expected_end_date<E>(&self) -> Option<&E>
    where
        chrono::NaiveDateTime: AsRef<E>,
    {
        self.expected_end_date.as_ref().map(|value| value.as_ref())
    }

    /// Get the end_date attribute.
    pub fn get_end_date<E>(&self) -> Option<&E>
    where
        chrono::NaiveDateTime: AsRef<E>,
    {
        self.end_date.as_ref().map(|value| value.as_ref())
    }

    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
            (self.public.into()),
            gluesql::core::ast_builder::num(self.state_id),
            gluesql::core::ast_builder::num(self.icon_id),
            gluesql::core::ast_builder::num(self.color_id),
            match self.parent_project_id {
                Some(parent_project_id) => gluesql::core::ast_builder::num(parent_project_id),
                None => gluesql::core::ast_builder::null(),
            },
            match self.budget {
                Some(budget) => gluesql::core::ast_builder::num(budget),
                None => gluesql::core::ast_builder::null(),
            },
            match self.expenses {
                Some(expenses) => gluesql::core::ast_builder::num(expenses),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::num(self.updated_by),
            gluesql::core::ast_builder::timestamp(self.updated_at.to_string()),
            match self.expected_end_date {
                Some(expected_end_date) => {
                    gluesql::core::ast_builder::timestamp(expected_end_date.to_string())
                }
                None => gluesql::core::ast_builder::null(),
            },
            match self.end_date {
                Some(end_date) => gluesql::core::ast_builder::timestamp(end_date.to_string()),
                None => gluesql::core::ast_builder::null(),
            },
        ]
    }

    /// Insert the Project into the database.
    ///
    /// * `connection` - The connection to the database.
    async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        Ok(table("projects")
            .insert()
            .columns("id, name, description, public, state_id, icon_id, color_id, parent_project_id, budget, expenses, created_by, created_at, updated_by, updated_at, expected_end_date, end_date")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })?)
    }

    /// Get the Project from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `connection` - The connection to the database.
    pub async fn get<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        let select_row = table("projects")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, description, public, state_id, icon_id, color_id, parent_project_id, budget, expenses, created_by, created_at, updated_by, updated_at, expected_end_date, end_date")
            .limit(1)
            .execute(connection)
            .await?;
        Ok(select_row
            .select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete the Project from the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn delete<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        Self::delete_from_id(self.id, connection).await
    }

    /// Delete the Project from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    pub async fn delete_from_id<
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    >(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        Ok(table("projects")
            .delete()
            .filter(col("id").eq(id.to_string()))
            .execute(connection)
            .await
            .map(|payload| match payload {
                gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                _ => unreachable!("Payload must be a Delete"),
            })?)
    }

    /// Update the struct in the database.
    ///
    /// * `connection` - The connection to the database.
    async fn update<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        let mut update_row = table("projects")
            .update()
            .set("id", gluesql::core::ast_builder::num(self.id))
            .set("name", gluesql::core::ast_builder::text(self.name))
            .set(
                "description",
                gluesql::core::ast_builder::text(self.description),
            )
            .set("public", self.public)
            .set("state_id", gluesql::core::ast_builder::num(self.state_id))
            .set("icon_id", gluesql::core::ast_builder::num(self.icon_id))
            .set("color_id", gluesql::core::ast_builder::num(self.color_id))
            .set(
                "created_by",
                gluesql::core::ast_builder::num(self.created_by),
            )
            .set(
                "created_at",
                gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            )
            .set(
                "updated_by",
                gluesql::core::ast_builder::num(self.updated_by),
            )
            .set(
                "updated_at",
                gluesql::core::ast_builder::timestamp(self.updated_at.to_string()),
            );
        if let Some(parent_project_id) = self.parent_project_id {
            update_row = update_row.set(
                "parent_project_id",
                gluesql::core::ast_builder::num(parent_project_id),
            );
        }
        if let Some(budget) = self.budget {
            update_row = update_row.set("budget", gluesql::core::ast_builder::num(budget));
        }
        if let Some(expenses) = self.expenses {
            update_row = update_row.set("expenses", gluesql::core::ast_builder::num(expenses));
        }
        if let Some(expected_end_date) = self.expected_end_date {
            update_row = update_row.set(
                "expected_end_date",
                gluesql::core::ast_builder::timestamp(expected_end_date.to_string()),
            );
        }
        if let Some(end_date) = self.end_date {
            update_row = update_row.set(
                "end_date",
                gluesql::core::ast_builder::timestamp(end_date.to_string()),
            );
        }
        update_row
            .execute(connection)
            .await
            .map(|payload| match payload {
                gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                _ => unreachable!("Expected Payload::Update"),
            })
            .map_err(crate::api::ApiError::from)
    }

    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// * `connection` - The connection to the database.
    pub async fn update_or_insert<
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    >(
        &self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.clone().insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Project from the database.
    ///
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    pub async fn all<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        filter: Option<&crate::database::filter_variants::ProjectFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        let select_row = table("projects")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, name, description, public, state_id, icon_id, color_id, parent_project_id, budget, expenses, created_by, created_at, updated_by, updated_at, expected_end_date, end_date")
            .order_by("end_date DESC")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row
            .select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32"),
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str"),
            },
            description: match row.get("description").unwrap() {
                gluesql::prelude::Value::Str(description) => description.clone(),
                _ => unreachable!("Expected Str"),
            },
            public: match row.get("public").unwrap() {
                gluesql::prelude::Value::Bool(public) => public.clone(),
                _ => unreachable!("Expected Bool"),
            },
            state_id: match row.get("state_id").unwrap() {
                gluesql::prelude::Value::I32(state_id) => state_id.clone(),
                _ => unreachable!("Expected I32"),
            },
            icon_id: match row.get("icon_id").unwrap() {
                gluesql::prelude::Value::I32(icon_id) => icon_id.clone(),
                _ => unreachable!("Expected I32"),
            },
            color_id: match row.get("color_id").unwrap() {
                gluesql::prelude::Value::I32(color_id) => color_id.clone(),
                _ => unreachable!("Expected I32"),
            },
            parent_project_id: match row.get("parent_project_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(parent_project_id) => Some(parent_project_id.clone()),
                _ => unreachable!("Expected I32"),
            },
            budget: match row.get("budget").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(budget) => Some(budget.clone()),
                _ => unreachable!("Expected F64"),
            },
            expenses: match row.get("expenses").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(expenses) => Some(expenses.clone()),
                _ => unreachable!("Expected F64"),
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32"),
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp"),
            },
            updated_by: match row.get("updated_by").unwrap() {
                gluesql::prelude::Value::I32(updated_by) => updated_by.clone(),
                _ => unreachable!("Expected I32"),
            },
            updated_at: match row.get("updated_at").unwrap() {
                gluesql::prelude::Value::Timestamp(updated_at) => updated_at.clone(),
                _ => unreachable!("Expected Timestamp"),
            },
            expected_end_date: match row.get("expected_end_date").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Timestamp(expected_end_date) => {
                    Some(expected_end_date.clone())
                }
                _ => unreachable!("Expected Timestamp"),
            },
            end_date: match row.get("end_date").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Timestamp(end_date) => Some(end_date.clone()),
                _ => unreachable!("Expected Timestamp"),
            },
        }
    }
}
