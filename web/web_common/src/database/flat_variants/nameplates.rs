//! This file is automatically generated, do not modify it directly.
use super::*;
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Nameplate {
    pub id: i32,
    pub barcode: String,
    pub project_id: i32,
    pub category_id: i32,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: chrono::NaiveDateTime,
    pub geolocation: crate::types::Point,
}

unsafe impl Send for Nameplate {}
unsafe impl Sync for Nameplate {}
impl Tabular for Nameplate {
    const TABLE: crate::database::Table = crate::database::Table::Nameplates;
}
impl Describable for Nameplate {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for Nameplate {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for crate::database::flat_variants::Nameplate {
    type Filter = crate::database::filter_variants::NameplateFilter;
}
#[cfg(feature = "frontend")]
impl AllRecords for Nameplate {
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
impl Nameplate {
    /// Get the id attribute.
    pub fn get_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.id.as_ref()
    }

    /// Get the barcode attribute.
    pub fn get_barcode<E>(&self) -> &E
    where
        String: AsRef<E>,
    {
        self.barcode.as_ref()
    }

    /// Get the project_id attribute.
    pub fn get_project_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.project_id.as_ref()
    }

    /// Get the category_id attribute.
    pub fn get_category_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.category_id.as_ref()
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

    /// Get the geolocation attribute.
    pub fn get_geolocation<E>(&self) -> &E
    where
        crate::types::Point: AsRef<E>,
    {
        self.geolocation.as_ref()
    }

    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.barcode),
            gluesql::core::ast_builder::num(self.project_id),
            gluesql::core::ast_builder::num(self.category_id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::num(self.updated_by),
            gluesql::core::ast_builder::timestamp(self.updated_at.to_string()),
            gluesql::core::ast_builder::function::point(
                gluesql::core::ast_builder::num(self.geolocation.x),
                gluesql::core::ast_builder::num(self.geolocation.y),
            ),
        ]
    }

    /// Insert the Nameplate into the database.
    ///
    /// * `connection` - The connection to the database.
    async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        Ok(table("nameplates")
            .insert()
            .columns("id, barcode, project_id, category_id, created_by, created_at, updated_by, updated_at, geolocation")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
             .map(|payload| match payload {
                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,
                 _ => unreachable!("Payload must be an Insert"),
             })?)
    }

    /// Get the Nameplate from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `connection` - The connection to the database.
    pub async fn get<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        let select_row = table("nameplates")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, barcode, project_id, category_id, created_by, created_at, updated_by, updated_at, geolocation")
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

    /// Delete the Nameplate from the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn delete<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        Self::delete_from_id(self.id, connection).await
    }

    /// Delete the Nameplate from the database by its ID.
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
        Ok(table("nameplates")
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
        table("nameplates")
            .update()
            .set("id", gluesql::core::ast_builder::num(self.id))
            .set("barcode", gluesql::core::ast_builder::text(self.barcode))
            .set(
                "project_id",
                gluesql::core::ast_builder::num(self.project_id),
            )
            .set(
                "category_id",
                gluesql::core::ast_builder::num(self.category_id),
            )
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
            )
            .set(
                "geolocation",
                gluesql::core::ast_builder::function::point(
                    gluesql::core::ast_builder::num(self.geolocation.x),
                    gluesql::core::ast_builder::num(self.geolocation.y),
                ),
            )
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
    /// Get all Nameplate from the database.
    ///
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    pub async fn all<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        filter: Option<&crate::database::filter_variants::NameplateFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        let select_row = table("nameplates")
            .select()
            .filter(filter.map_or_else(|| gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true)).into(), |filter| filter.as_filter_expression()))
           .project("id, barcode, project_id, category_id, created_by, created_at, updated_by, updated_at, geolocation")
            .order_by("geolocation DESC")
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
            barcode: match row.get("barcode").unwrap() {
                gluesql::prelude::Value::Str(barcode) => barcode.clone(),
                _ => unreachable!("Expected Str"),
            },
            project_id: match row.get("project_id").unwrap() {
                gluesql::prelude::Value::I32(project_id) => project_id.clone(),
                _ => unreachable!("Expected I32"),
            },
            category_id: match row.get("category_id").unwrap() {
                gluesql::prelude::Value::I32(category_id) => category_id.clone(),
                _ => unreachable!("Expected I32"),
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
            geolocation: match row.get("geolocation").unwrap() {
                gluesql::prelude::Value::Point(geolocation) => geolocation.clone().into(),
                _ => unreachable!("Expected Bytea"),
            },
        }
    }
}
