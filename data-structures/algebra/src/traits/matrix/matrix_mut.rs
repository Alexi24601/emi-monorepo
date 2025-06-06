//! Submodule defining the `MatrixMut` trait.

use super::{Matrix, SparseMatrix};

/// Trait defining a mutable matrix.
pub trait MatrixMut: Matrix + Default {
    /// Type of the entry of the matrix.
    /// In a matrix with values, this is generally a tuple of the coordinates
    /// and the value, while in a matrix without values, this is generally
    /// the coordinates.
    type Entry;

    /// The type of error that can be returned when adding an entry.
    type Error: core::error::Error;

    /// Sets the value at the given entry.
    ///
    /// # Arguments
    ///
    /// * `entry`: The entry to set.
    ///
    /// # Errors
    ///
    /// Returns an error if the entry cannot be added. Possible reasons include:
    /// - The entries are not provided in the expected order.
    /// - The entry is out of bounds.
    /// - The entry is already defined.
    fn add(&mut self, entry: Self::Entry) -> Result<(), Self::Error>;
}

/// Trait defining a bi-dimensional mutable matrix.
pub trait SparseMatrixMut: MatrixMut + SparseMatrix {
    /// Type describing the shape of the matrix.
    type MinimalShape: core::fmt::Debug + Copy;

    /// Creates a new matrix with the given capacity, using the given shape.
    ///
    /// # Arguments
    ///
    /// * `shape` - The shape of the matrix.
    /// * `number_of_values` - The number of values.
    fn with_sparse_shaped_capacity(
        shape: Self::MinimalShape,
        number_of_values: Self::SparseIndex,
    ) -> Self;

    /// Creates a new matrix with the given capacity and unknown shape.
    ///
    /// # Arguments
    ///
    /// * `number_of_values` - The number of values.
    fn with_sparse_capacity(number_of_values: Self::SparseIndex) -> Self;
}
