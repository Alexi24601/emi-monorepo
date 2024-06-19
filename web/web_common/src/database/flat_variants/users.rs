//! This file is automatically generated, do not modify it directly.
use super::*;
use crate::traits::GuessImageFormat;
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub description: Option<String>,
    pub organization_id: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub picture: crate::types::JPEG,
}

unsafe impl Send for User {}
unsafe impl Sync for User {}
impl Tabular for User {
    const TABLE: crate::database::Table = crate::database::Table::Users;
}
impl Describable for User {
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}
impl Colorable for User {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for crate::database::flat_variants::User {
    type Filter = crate::database::filter_variants::UserFilter;
}
#[cfg(feature = "frontend")]
impl AllRecords for User {
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
impl User {
    /// Get the id attribute.
    pub fn get_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.id.as_ref()
    }

    /// Get the first_name attribute.
    pub fn get_first_name<E>(&self) -> &E
    where
        String: AsRef<E>,
    {
        self.first_name.as_ref()
    }

    /// Get the middle_name attribute.
    pub fn get_middle_name<E>(&self) -> Option<&E>
    where
        String: AsRef<E>,
    {
        self.middle_name.as_ref().map(|value| value.as_ref())
    }

    /// Get the last_name attribute.
    pub fn get_last_name<E>(&self) -> &E
    where
        String: AsRef<E>,
    {
        self.last_name.as_ref()
    }

    /// Get the description attribute.
    pub fn get_description<E>(&self) -> Option<&E>
    where
        String: AsRef<E>,
    {
        self.description.as_ref().map(|value| value.as_ref())
    }

    /// Get the organization_id attribute.
    pub fn get_organization_id<E>(&self) -> Option<&E>
    where
        i32: AsRef<E>,
    {
        self.organization_id.as_ref().map(|value| value.as_ref())
    }

    /// Get the created_at attribute.
    pub fn get_created_at<E>(&self) -> &E
    where
        chrono::NaiveDateTime: AsRef<E>,
    {
        self.created_at.as_ref()
    }

    /// Get the updated_at attribute.
    pub fn get_updated_at<E>(&self) -> &E
    where
        chrono::NaiveDateTime: AsRef<E>,
    {
        self.updated_at.as_ref()
    }

    /// Get the URL of the picture image.
    pub fn get_picture_as_url(&self) -> String {
        self.picture.guess_image_url().unwrap()
    }

    /// Get the picture attribute.
    pub fn get_picture<E>(&self) -> &E
    where
        crate::types::JPEG: AsRef<E>,
    {
        self.picture.as_ref()
    }

    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.first_name),
            match self.middle_name {
                Some(middle_name) => gluesql::core::ast_builder::text(middle_name),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::text(self.last_name),
            match self.description {
                Some(description) => gluesql::core::ast_builder::text(description),
                None => gluesql::core::ast_builder::null(),
            },
            match self.organization_id {
                Some(organization_id) => gluesql::core::ast_builder::num(organization_id),
                None => gluesql::core::ast_builder::null(),
            },
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::timestamp(self.updated_at.to_string()),
            gluesql::core::ast_builder::bytea(self.picture),
        ]
    }

    /// Insert the User into the database.
    ///
    /// * `connection` - The connection to the database.
    async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        Ok(table("users")
            .insert()
            .columns("id, first_name, middle_name, last_name, description, organization_id, created_at, updated_at, picture")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })?)
    }

    /// Get the User from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `connection` - The connection to the database.
    pub async fn get<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        let select_row = table("users")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, first_name, middle_name, last_name, description, organization_id, created_at, updated_at, picture")
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

    /// Delete the User from the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn delete<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        Self::delete_from_id(self.id, connection).await
    }

    /// Delete the User from the database by its ID.
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
        Ok(table("users")
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
        let mut update_row = table("users")
            .update()
            .set("id", gluesql::core::ast_builder::num(self.id))
            .set(
                "first_name",
                gluesql::core::ast_builder::text(self.first_name),
            )
            .set(
                "last_name",
                gluesql::core::ast_builder::text(self.last_name),
            )
            .set(
                "created_at",
                gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            )
            .set(
                "updated_at",
                gluesql::core::ast_builder::timestamp(self.updated_at.to_string()),
            )
            .set("picture", gluesql::core::ast_builder::bytea(self.picture));
        if let Some(middle_name) = self.middle_name {
            update_row =
                update_row.set("middle_name", gluesql::core::ast_builder::text(middle_name));
        }
        if let Some(description) = self.description {
            update_row =
                update_row.set("description", gluesql::core::ast_builder::text(description));
        }
        if let Some(organization_id) = self.organization_id {
            update_row = update_row.set(
                "organization_id",
                gluesql::core::ast_builder::num(organization_id),
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
    /// Get all User from the database.
    ///
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    pub async fn all<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        filter: Option<&crate::database::filter_variants::UserFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        let select_row = table("users")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, first_name, middle_name, last_name, description, organization_id, created_at, updated_at, picture")
            .order_by("picture DESC")
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
            first_name: match row.get("first_name").unwrap() {
                gluesql::prelude::Value::Str(first_name) => first_name.clone(),
                _ => unreachable!("Expected Str"),
            },
            middle_name: match row.get("middle_name").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Str(middle_name) => Some(middle_name.clone()),
                _ => unreachable!("Expected Str"),
            },
            last_name: match row.get("last_name").unwrap() {
                gluesql::prelude::Value::Str(last_name) => last_name.clone(),
                _ => unreachable!("Expected Str"),
            },
            description: match row.get("description").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Str(description) => Some(description.clone()),
                _ => unreachable!("Expected Str"),
            },
            organization_id: match row.get("organization_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(organization_id) => Some(organization_id.clone()),
                _ => unreachable!("Expected I32"),
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp"),
            },
            updated_at: match row.get("updated_at").unwrap() {
                gluesql::prelude::Value::Timestamp(updated_at) => updated_at.clone(),
                _ => unreachable!("Expected Timestamp"),
            },
            picture: match row.get("picture").unwrap() {
                gluesql::prelude::Value::Bytea(picture) => picture.clone().into(),
                _ => unreachable!("Expected Bytea"),
            },
        }
    }
}
