//! A trait defining a Deletable table entry.

/// The Deletable trait
pub trait Deletable {
    #[cfg(feature = "backend")]
    /// Deletes the row in a table.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * Returns an error if the row cannot be deleted.
    ///
    fn delete<'a>(
        &'a self,
        conn: &'a mut crate::prelude::DBConn,
    ) -> impl std::future::Future<Output = Result<usize, diesel::result::Error>> + 'a;
}
