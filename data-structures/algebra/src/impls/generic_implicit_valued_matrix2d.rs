//! Submodule proving a 2D matrix with implicit values.

use crate::traits::{
    ImplicitValuedMatrix, ImplicitValuedSparseMatrix, ImplicitValuedSparseRowIteraror, Matrix2D,
    Matrix2DRef, SparseMatrix, SparseMatrix2D, SparseValuedMatrix, ValuedMatrix, ValuedMatrix2D,
    ValuedSparseMatrix2D,
};

/// A 2D matrix with implicit values.
pub struct GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: Matrix2D,
    Map: Fn(M::Coordinates) -> Value,
{
    matrix: M,
    map: Map,
}

impl<M, Map, Value> GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: Matrix2D,
    Map: Fn(M::Coordinates) -> Value,
{
    /// Creates a new [`GenericImplicitValuedMatrix2D`].
    pub fn new(matrix: M, map: Map) -> Self {
        Self { matrix, map }
    }
}

impl<M, Map, Value> Matrix2D for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: Matrix2D,
    Map: Fn(M::Coordinates) -> Value,
{
    /// Type of the row index.
    type RowIndex = M::RowIndex;
    /// Type of the column index.
    type ColumnIndex = M::ColumnIndex;

    /// Returns the number of rows of the matrix.
    fn number_of_rows(&self) -> Self::RowIndex {
        self.matrix.number_of_rows()
    }

    /// Returns the number of columns of the matrix.
    fn number_of_columns(&self) -> Self::ColumnIndex {
        self.matrix.number_of_columns()
    }
}

impl<M, Map, Value> Matrix2DRef for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: Matrix2DRef,
    Map: Fn(M::Coordinates) -> Value,
{
    fn number_of_rows_ref(&self) -> &Self::RowIndex {
        self.matrix.number_of_rows_ref()
    }

    fn number_of_columns_ref(&self) -> &Self::ColumnIndex {
        self.matrix.number_of_columns_ref()
    }
}

impl<M, Map, Value> SparseMatrix for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: SparseMatrix2D,
    Map: Fn(M::Coordinates) -> Value,
{
    type SparseCoordinates<'a>
        = M::SparseCoordinates<'a>
    where
        Self: 'a;
    type SparseIndex = M::SparseIndex;

    fn number_of_defined_values(&self) -> Self::SparseIndex {
        self.matrix.number_of_defined_values()
    }

    fn sparse_coordinates(&self) -> Self::SparseCoordinates<'_> {
        self.matrix.sparse_coordinates()
    }
}

impl<M, Map, Value> SparseMatrix2D for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: SparseMatrix2D,
    Map: Fn(M::Coordinates) -> Value,
{
    type SparseColumns<'a>
        = M::SparseColumns<'a>
    where
        Self: 'a;
    type SparseRow<'a>
        = M::SparseRow<'a>
    where
        Self: 'a;
    type SparseRowSizes<'a>
        = M::SparseRowSizes<'a>
    where
        Self: 'a;
    type SparseRows<'a>
        = M::SparseRows<'a>
    where
        Self: 'a;
    type EmptyRowIndices<'a>
        = M::EmptyRowIndices<'a>
    where
        Self: 'a;
    type NonEmptyRowIndices<'a>
        = M::NonEmptyRowIndices<'a>
    where
        Self: 'a;

    fn sparse_row(&self, row: Self::RowIndex) -> Self::SparseRow<'_> {
        self.matrix.sparse_row(row)
    }

    fn sparse_columns(&self) -> Self::SparseColumns<'_> {
        self.matrix.sparse_columns()
    }

    fn sparse_rows(&self) -> Self::SparseRows<'_> {
        self.matrix.sparse_rows()
    }

    fn sparse_row_sizes(&self) -> Self::SparseRowSizes<'_> {
        self.matrix.sparse_row_sizes()
    }

    fn number_of_defined_values_in_row(&self, row: Self::RowIndex) -> Self::ColumnIndex {
        self.matrix.number_of_defined_values_in_row(row)
    }

    fn rank(&self, row: Self::RowIndex) -> Self::SparseIndex {
        self.matrix.rank(row)
    }

    fn empty_row_indices(&self) -> Self::EmptyRowIndices<'_> {
        self.matrix.empty_row_indices()
    }

    fn non_empty_row_indices(&self) -> Self::NonEmptyRowIndices<'_> {
        self.matrix.non_empty_row_indices()
    }

    fn number_of_empty_rows(&self) -> Self::RowIndex {
        self.matrix.number_of_empty_rows()
    }

    fn number_of_non_empty_rows(&self) -> Self::RowIndex {
        self.matrix.number_of_non_empty_rows()
    }
}

impl<M, Map, Value> ValuedMatrix for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: Matrix2D,
    Map: Fn(M::Coordinates) -> Value,
{
    type Value = Value;
}

impl<M: Matrix2D, Map, Value> ValuedMatrix2D for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: Matrix2D,
    Map: Fn(M::Coordinates) -> Value,
{
}

impl<M, Map, Value> ImplicitValuedMatrix for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: Matrix2D,
    Map: Fn(M::Coordinates) -> Value,
{
    fn implicit_value(&self, coordinates: &Self::Coordinates) -> Self::Value {
        (self.map)(*coordinates)
    }
}

impl<M, Map, Value> SparseValuedMatrix
    for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: SparseMatrix2D,
    Map: Fn(M::Coordinates) -> Value,
{
    type SparseValues<'a>
        = <Self as ImplicitValuedSparseMatrix>::SparseImplicitValues<'a>
    where
        Self: 'a;

    fn sparse_values(&self) -> Self::SparseValues<'_> {
        self.sparse_implicit_values()
    }
}

impl<M, Map, Value> ValuedSparseMatrix2D for GenericImplicitValuedMatrix2D<M, Map, Value>
where
    M: SparseMatrix2D,
    Map: Fn(M::Coordinates) -> Value,
{
    type SparseRowValues<'a>
        = ImplicitValuedSparseRowIteraror<'a, Self>
    where
        Self: 'a;

    fn sparse_row_values(&self, row: Self::RowIndex) -> Self::SparseRowValues<'_> {
        ImplicitValuedSparseRowIteraror::new(self, row)
    }
}
