//! Submodule for inserting the missing instrument models which
//! are present in the Directus database but not in the Portal database.

use diesel_async::AsyncPgConnection;

use super::get_user;
use crate::codegen::{Brand as DirectusBrand, InstrumentModel as DirectusInstrumentModel};
use core_structures::{
    Brand as PortalBrand, InstrumentModel as PortalInstrumentModel,
    InstrumentType as PortalInstrumentType,
};
use web_common_traits::database::{Insertable, InsertableVariant, Loadable};
use web_common_traits::prelude::Builder;

/// Inserts missing instrument models into the portal database.
///
/// # Arguments
///
/// * `directus_conn` - A mutable reference to the Directus database connection.
/// * `portal_conn` - A mutable reference to the portal database connection.
///
/// # Errors
///
/// * If the insertion fails, an error of type `error::Error` is returned.
///
pub async fn insert_missing_instrument_models(
    directus_conn: &mut AsyncPgConnection,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let directus_instrument_models = DirectusInstrumentModel::load_all(directus_conn).await?;
    for directus_instrument_model in directus_instrument_models {
        if PortalInstrumentModel::from_name(
            &directus_instrument_model.instrument_model,
            portal_conn,
        )
        .await?
        .is_some()
        {
            continue;
        }
        let directus_brand = directus_instrument_model.brand(directus_conn).await?;
        let portal_brand = PortalBrand::from_name(&directus_brand.brand, portal_conn)
            .await?
            .ok_or_else(|| crate::error::Error::UnknownBrand(Box::from(directus_brand.clone())))?;
        let directus_instrument_type =
            directus_instrument_model.instrument_type(directus_conn).await?;
        let portal_instrument_type = PortalInstrumentType::from_name(
            directus_instrument_type.instrument_type.as_ref().ok_or_else(|| {
                crate::error::Error::MissingInstrumentTypeName(Box::from(
                    directus_instrument_type.clone(),
                ))
            })?,
            portal_conn,
        )
        .await?
        .ok_or_else(|| {
            crate::error::Error::UnknownInstrumentType(Box::from(directus_instrument_type.clone()))
        })?;
        let directus_created_by =
            directus_instrument_model.user_created(directus_conn).await?.ok_or_else(|| {
                crate::error::Error::InstrumentModelWithMissingUser(Box::from(
                    directus_instrument_model.clone(),
                ))
            })?;
        let directus_updated_by = directus_instrument_model
            .user_updated(directus_conn)
            .await?
            .unwrap_or_else(|| directus_created_by.clone());
        let created_at = directus_instrument_model.date_created.ok_or_else(|| {
            crate::error::Error::MissingDate(
                "instrument_models".to_owned(),
                "date_created".to_owned(),
            )
        })?;
        let updated_at = directus_instrument_model.date_updated.unwrap_or(created_at);

        let portal_created_by = get_user(&directus_created_by, directus_conn, portal_conn).await?;
        let portal_updated_by = get_user(&directus_updated_by, directus_conn, portal_conn).await?;

        let portal_instrument_model = PortalInstrumentModel::new()
            .name(directus_instrument_model.instrument_model)?
            .brand_id(portal_brand.id)?
            .instrument_type_id(portal_instrument_type.id)?
            .created_by(portal_created_by.id)?
            .updated_by(portal_updated_by.id)?
            .updated_at(updated_at)?
            .created_at(created_at)?
            .build()?
            .insert(portal_conn).await?;
    }
    Ok(())
}
