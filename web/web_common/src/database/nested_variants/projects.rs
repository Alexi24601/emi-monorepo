//! This module contains the nested structs for the database tables.
//!
//! This file is automatically generated. Do not write anything here.

use std::rc::Rc;

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct NestedProject {
    pub inner: Rc<crate::database::flat_variants::Project>,
    pub state: Rc<crate::database::nested_variants::NestedProjectState>,
    pub icon: Rc<crate::database::flat_variants::FontAwesomeIcon>,
    pub color: Rc<crate::database::flat_variants::Color>,
    pub parent_project: Option<Rc<crate::database::flat_variants::Project>>,
    pub created_by: Rc<crate::database::nested_variants::NestedUser>,
    pub updated_by: Rc<crate::database::nested_variants::NestedUser>,
}

unsafe impl Send for NestedProject {}
unsafe impl Sync for NestedProject {}
impl crate::database::Tabular for NestedProject {
    const TABLE: crate::database::Table = crate::database::Table::Projects;
}
impl crate::database::Filtrable for NestedProject {
    type Filter = crate::database::filter_variants::ProjectFilter;
}
impl crate::database::Describable for NestedProject {
    fn description(&self) -> Option<&str> {
        self.inner.description()
    }
}
impl crate::database::Colorable for NestedProject {
    fn color(&self) -> Option<&str> {
        Some(self.color.name.as_str())
    }
}
#[cfg(feature = "frontend")]
impl crate::database::AllRecords for NestedProject {
    fn all_records<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        filter: Option<&<Self as crate::database::Filtrable>::Filter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> impl std::future::Future<Output = Result<Vec<Self>, crate::api::ApiError>> {
        Self::all(filter, limit, offset, connection)
    }
}
#[cfg(feature = "frontend")]
impl NestedProject {
    /// Convert the flat struct to the nested struct.
    ///
    /// # Arguments
    /// * `flat_variant` - The flat struct.
    /// * `connection` - The database connection.
    pub(crate) async fn from_flat(
        flat_variant: crate::database::flat_variants::Project,
        connection: &mut gluesql::prelude::Glue<
            impl gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
        >,
    ) -> Result<Self, crate::api::ApiError> {
        Ok(Self {
            state: Rc::from(
                crate::database::nested_variants::NestedProjectState::get(
                    flat_variant.state_id,
                    connection,
                )
                .await?
                .unwrap(),
            ),
            icon: Rc::from(
                crate::database::flat_variants::FontAwesomeIcon::get(
                    flat_variant.icon_id,
                    connection,
                )
                .await?
                .unwrap(),
            ),
            color: Rc::from(
                crate::database::flat_variants::Color::get(flat_variant.color_id, connection)
                    .await?
                    .unwrap(),
            ),
            parent_project: if let Some(parent_project_id) = flat_variant.parent_project_id {
                crate::database::flat_variants::Project::get(parent_project_id, connection)
                    .await?
                    .map(Rc::from)
            } else {
                None
            },
            created_by: Rc::from(
                crate::database::nested_variants::NestedUser::get(
                    flat_variant.created_by,
                    connection,
                )
                .await?
                .unwrap(),
            ),
            updated_by: Rc::from(
                crate::database::nested_variants::NestedUser::get(
                    flat_variant.updated_by,
                    connection,
                )
                .await?
                .unwrap(),
            ),
            inner: Rc::from(flat_variant),
        })
    }
    /// Get the id attribute.
    pub fn get_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_id()
    }
    /// Get the name attribute.
    pub fn get_name<E>(&self) -> &E
    where
        String: AsRef<E>,
    {
        self.inner.get_name()
    }
    /// Get the description attribute.
    pub fn get_description<E>(&self) -> &E
    where
        String: AsRef<E>,
    {
        self.inner.get_description()
    }
    /// Whether the Project is public.
    pub fn is_public<E>(&self) -> &E
    where
        bool: AsRef<E>,
    {
        self.inner.is_public()
    }
    /// Get the state_id attribute.
    pub fn get_state_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_state_id()
    }
    /// Get the icon_id attribute.
    pub fn get_icon_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_icon_id()
    }
    /// Get the color_id attribute.
    pub fn get_color_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_color_id()
    }
    /// Get the parent_project_id attribute.
    pub fn get_parent_project_id<E>(&self) -> Option<&E>
    where
        i32: AsRef<E>,
    {
        self.inner.get_parent_project_id()
    }
    /// Get the budget attribute.
    pub fn get_budget<E>(&self) -> Option<&E>
    where
        f64: AsRef<E>,
    {
        self.inner.get_budget()
    }
    /// Get the expenses attribute.
    pub fn get_expenses<E>(&self) -> Option<&E>
    where
        f64: AsRef<E>,
    {
        self.inner.get_expenses()
    }
    /// Get the created_by attribute.
    pub fn get_created_by<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_created_by()
    }
    /// Get the created_at attribute.
    pub fn get_created_at<E>(&self) -> &E
    where
        chrono::NaiveDateTime: AsRef<E>,
    {
        self.inner.get_created_at()
    }
    /// Get the updated_by attribute.
    pub fn get_updated_by<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_updated_by()
    }
    /// Get the updated_at attribute.
    pub fn get_updated_at<E>(&self) -> &E
    where
        chrono::NaiveDateTime: AsRef<E>,
    {
        self.inner.get_updated_at()
    }
    /// Get the expected_end_date attribute.
    pub fn get_expected_end_date<E>(&self) -> Option<&E>
    where
        chrono::NaiveDateTime: AsRef<E>,
    {
        self.inner.get_expected_end_date()
    }
    /// Get the end_date attribute.
    pub fn get_end_date<E>(&self) -> Option<&E>
    where
        chrono::NaiveDateTime: AsRef<E>,
    {
        self.inner.get_end_date()
    }
    /// Get the Project from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `connection` - The connection to the database.
    pub async fn get<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, crate::api::ApiError> {
        if let Some(flat_variant) =
            crate::database::flat_variants::Project::get(id, connection).await?
        {
            Ok(Some(Self::from_flat(flat_variant, connection).await?))
        } else {
            Ok(None)
        }
    }
    /// Delete the Project from the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn delete<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        self.inner.as_ref().clone().delete(connection).await
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
        crate::database::flat_variants::Project::delete_from_id(id, connection).await
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
        let mut projects = Vec::new();
        for flat_variant in
            crate::database::flat_variants::Project::all(filter, limit, offset, connection)
                .await?
                .into_iter()
        {
            projects.push(Self::from_flat(flat_variant, connection).await?);
        }
        Ok(projects)
    }
    /// Update or insert the record in the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn update_or_insert<
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    >(
        &self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        crate::database::nested_variants::NestedProjectState::update_or_insert(
            self.state.as_ref(),
            connection,
        )
        .await?;
        crate::database::flat_variants::FontAwesomeIcon::update_or_insert(
            self.icon.as_ref(),
            connection,
        )
        .await?;
        crate::database::flat_variants::Color::update_or_insert(self.color.as_ref(), connection)
            .await?;
        if let Some(parent_project) = self.parent_project.as_ref() {
            crate::database::flat_variants::Project::update_or_insert(parent_project, connection)
                .await?;
        }
        crate::database::nested_variants::NestedUser::update_or_insert(
            self.created_by.as_ref(),
            connection,
        )
        .await?;
        crate::database::nested_variants::NestedUser::update_or_insert(
            self.updated_by.as_ref(),
            connection,
        )
        .await?;
        crate::database::flat_variants::Project::update_or_insert(self.inner.as_ref(), connection)
            .await
    }
}
