use crate::database::*;
use std::rc::Rc;

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct NestedLoginProvider {
    pub inner: crate::database::flat_variants::LoginProvider,
    pub icon: crate::database::flat_variants::FontAwesomeIcon,
    pub color: crate::database::flat_variants::Color,
}

unsafe impl Send for NestedLoginProvider {}
unsafe impl Sync for NestedLoginProvider {}
impl NestedLoginProvider {
    /// Convert the flat struct to the nested struct.
    ///
    /// # Arguments
    /// * `flat_variant` - The flat struct.
    /// * `connection` - The database connection.
    pub(crate) fn from_flat(
        flat_variant: LoginProvider,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::prelude::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        Ok(Self {
            icon: crate::database::flat_variants::FontAwesomeIcon::get(
                flat_variant.icon_id,
                connection,
            )?,
            color: crate::database::flat_variants::Color::get(flat_variant.color_id, connection)?,
            inner: flat_variant,
        })
    }
    /// Check whether the user can view the struct.
    pub fn can_view(&self) -> Result<bool, web_common::api::ApiError> {
        self.inner.can_view()
    }
    /// Check whether the user can view the struct associated to the provided ids.
    pub fn can_view_by_id() -> Result<bool, web_common::api::ApiError> {
        LoginProvider::can_view_by_id()
    }
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_viewable(
        filter: Option<&web_common::database::filter_variants::LoginProviderFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        LoginProvider::all_viewable(filter, limit, offset, connection)?
            .into_iter()
            .map(|flat_variant| Self::from_flat(flat_variant, connection))
            .collect()
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_viewable_sorted(
        filter: Option<&web_common::database::filter_variants::LoginProviderFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        LoginProvider::all_viewable_sorted(filter, limit, offset, connection)?
            .into_iter()
            .map(|flat_variant| Self::from_flat(flat_variant, connection))
            .collect()
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    pub fn get(
        id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        LoginProvider::get(id, connection)
            .and_then(|flat_variant| Self::from_flat(flat_variant, connection))
    }
    /// Get the struct from the database by its color_id.
    ///
    /// * `color_id` - The color_id of the struct to get.
    /// * `connection` - The connection to the database.
    pub fn from_color_id(
        color_id: &i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        LoginProvider::from_color_id(color_id, connection)
            .and_then(|flat_variant| Self::from_flat(flat_variant, connection))
    }
    /// Get the struct from the database by its icon_id.
    ///
    /// * `icon_id` - The icon_id of the struct to get.
    /// * `connection` - The connection to the database.
    pub fn from_icon_id(
        icon_id: &i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        LoginProvider::from_icon_id(icon_id, connection)
            .and_then(|flat_variant| Self::from_flat(flat_variant, connection))
    }
    /// Get the struct from the database by its name.
    ///
    /// * `name` - The name of the struct to get.
    /// * `connection` - The connection to the database.
    pub fn from_name(
        name: &str,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        LoginProvider::from_name(name, connection)
            .and_then(|flat_variant| Self::from_flat(flat_variant, connection))
    }
    /// Check whether the user can update the struct.
    pub fn can_update(&self) -> Result<bool, web_common::api::ApiError> {
        self.inner.can_update()
    }
    /// Check whether the user can update the struct associated to the provided ids.
    pub fn can_update_by_id() -> Result<bool, web_common::api::ApiError> {
        LoginProvider::can_update_by_id()
    }
    /// Check whether the user can admin the struct.
    pub fn can_admin(&self) -> Result<bool, web_common::api::ApiError> {
        self.inner.can_admin()
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    pub fn can_admin_by_id() -> Result<bool, web_common::api::ApiError> {
        LoginProvider::can_admin_by_id()
    }
}
impl From<web_common::database::nested_variants::NestedLoginProvider>
    for crate::database::nested_variants::NestedLoginProvider
{
    fn from(item: web_common::database::nested_variants::NestedLoginProvider) -> Self {
        Self {
            inner: crate::database::flat_variants::LoginProvider::from(item.inner.as_ref().clone()),
            icon: crate::database::flat_variants::FontAwesomeIcon::from(item.icon.as_ref().clone()),
            color: crate::database::flat_variants::Color::from(item.color.as_ref().clone()),
        }
    }
}
impl From<crate::database::nested_variants::NestedLoginProvider>
    for web_common::database::nested_variants::NestedLoginProvider
{
    fn from(item: crate::database::nested_variants::NestedLoginProvider) -> Self {
        Self {
            inner: Rc::from(web_common::database::flat_variants::LoginProvider::from(
                item.inner,
            )),
            icon: Rc::from(web_common::database::flat_variants::FontAwesomeIcon::from(
                item.icon,
            )),
            color: Rc::from(web_common::database::flat_variants::Color::from(item.color)),
        }
    }
}
