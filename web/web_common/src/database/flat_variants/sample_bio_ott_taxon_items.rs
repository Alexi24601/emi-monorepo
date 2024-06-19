//! This file is automatically generated, do not modify it directly.
use super::*;
#[derive(Eq, PartialEq, Debug, Clone, Copy, serde::Serialize, serde::Deserialize, Default)]
pub struct SampleBioOttTaxonItem {
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
    pub sample_id: uuid::Uuid,
    pub taxon_id: i32,
}

unsafe impl Send for SampleBioOttTaxonItem {}
unsafe impl Sync for SampleBioOttTaxonItem {}
impl Tabular for SampleBioOttTaxonItem {
    const TABLE: crate::database::Table = crate::database::Table::SampleBioOttTaxonItems;
}
impl Describable for SampleBioOttTaxonItem {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for SampleBioOttTaxonItem {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for crate::database::flat_variants::SampleBioOttTaxonItem {
    type Filter = crate::database::filter_variants::SampleBioOttTaxonItemFilter;
}
#[cfg(feature = "frontend")]
impl AllRecords for SampleBioOttTaxonItem {
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
impl SampleBioOttTaxonItem {
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

    /// Get the sample_id attribute.
    pub fn get_sample_id<E>(&self) -> &E
    where
        uuid::Uuid: AsRef<E>,
    {
        self.sample_id.as_ref()
    }

    /// Get the taxon_id attribute.
    pub fn get_taxon_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.taxon_id.as_ref()
    }

    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            gluesql::core::ast_builder::uuid(self.sample_id.to_string()),
            gluesql::core::ast_builder::num(self.taxon_id),
        ]
    }

    /// Insert the SampleBioOttTaxonItem into the database.
    ///
    /// * `connection` - The connection to the database.
    async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        Ok(table("sample_bio_ott_taxon_items")
            .insert()
            .columns("created_by, created_at, sample_id, taxon_id")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
            .map(|payload| match payload {
                gluesql::prelude::Payload::Insert(number_of_inserted_rows) => {
                    number_of_inserted_rows
                }
                _ => unreachable!("Payload must be an Insert"),
            })?)
    }

    /// Get the SampleBioOttTaxonItem from the database by its ID.
    ///
    /// * `( sample_id, taxon_id )` - The primary key(s) of the struct to check.
    /// * `connection` - The connection to the database.
    pub async fn get<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        (sample_id, taxon_id): (uuid::Uuid, i32),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        let select_row = table("sample_bio_ott_taxon_items")
            .select()
            .filter(col("sample_id").eq(sample_id.to_string()))
            .filter(col("taxon_id").eq(taxon_id.to_string()))
            .project("created_by, created_at, sample_id, taxon_id")
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

    /// Delete the SampleBioOttTaxonItem from the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn delete<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        Self::delete_from_id((self.sample_id, self.taxon_id), connection).await
    }

    /// Delete the SampleBioOttTaxonItem from the database by its ID.
    ///
    /// * `( sample_id, taxon_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    pub async fn delete_from_id<
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    >(
        (sample_id, taxon_id): (uuid::Uuid, i32),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        Ok(table("sample_bio_ott_taxon_items")
            .delete()
            .filter(col("sample_id").eq(sample_id.to_string()))
            .filter(col("taxon_id").eq(taxon_id.to_string()))
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
        table("sample_bio_ott_taxon_items")
            .update()
            .set(
                "created_by",
                gluesql::core::ast_builder::num(self.created_by),
            )
            .set(
                "created_at",
                gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            )
            .set(
                "sample_id",
                gluesql::core::ast_builder::uuid(self.sample_id.to_string()),
            )
            .set("taxon_id", gluesql::core::ast_builder::num(self.taxon_id))
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
    /// Get all SampleBioOttTaxonItem from the database.
    ///
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    pub async fn all<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        filter: Option<&crate::database::filter_variants::SampleBioOttTaxonItemFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        let select_row = table("sample_bio_ott_taxon_items")
            .select()
            .filter(filter.map_or_else(
                || {
                    gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true))
                        .into()
                },
                |filter| filter.as_filter_expression(),
            ))
            .project("created_by, created_at, sample_id, taxon_id")
            .order_by("taxon_id DESC, taxon_id DESC")
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
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32"),
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp"),
            },
            sample_id: match row.get("sample_id").unwrap() {
                gluesql::prelude::Value::Uuid(sample_id) => uuid::Uuid::from_u128(*sample_id),
                _ => unreachable!("Expected Uuid"),
            },
            taxon_id: match row.get("taxon_id").unwrap() {
                gluesql::prelude::Value::I32(taxon_id) => taxon_id.clone(),
                _ => unreachable!("Expected I32"),
            },
        }
    }
}
