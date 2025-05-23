//! Submodule providing a definition of a CSR matrix.
use crate::prelude::*;

#[derive(Clone)]
/// A compressed sparse row matrix.
pub struct UpperTriangularCSR2D<SparseIndex, Idx> {
    /// The underlying CSR matrix.
    csr: SquareCSR2D<SparseIndex, Idx>,
}

impl<SparseIndex, Idx: PositiveInteger + IntoUsize> Matrix
    for UpperTriangularCSR2D<SparseIndex, Idx>
where
    SquareCSR2D<SparseIndex, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    type Coordinates = (Idx, Idx);

    fn shape(&self) -> Vec<usize> {
        vec![self.number_of_rows().into_usize(), self.number_of_columns().into_usize()]
    }
}

impl<SparseIndex, Idx: IntoUsize + PositiveInteger> Matrix2D
    for UpperTriangularCSR2D<SparseIndex, Idx>
where
    SquareCSR2D<SparseIndex, Idx>: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    type RowIndex = Idx;
    type ColumnIndex = Idx;

    fn number_of_rows(&self) -> Self::RowIndex {
        self.csr.number_of_rows()
    }

    fn number_of_columns(&self) -> Self::ColumnIndex {
        self.csr.number_of_columns()
    }
}

impl<SparseIndex, Idx: IntoUsize + PositiveInteger> SquareMatrix
    for UpperTriangularCSR2D<SparseIndex, Idx>
where
    SquareCSR2D<SparseIndex, Idx>: SquareMatrix<Index = Idx>,
{
    type Index = Idx;

    fn order(&self) -> Self::Index {
        self.csr.order()
    }
}

impl<SparseIndex: PositiveInteger + IntoUsize, Idx: IntoUsize + PositiveInteger> SparseSquareMatrix
    for UpperTriangularCSR2D<SparseIndex, Idx>
where
    Self: SquareMatrix<Index = Idx>,
    SquareCSR2D<SparseIndex, Idx>: SparseSquareMatrix<Index = Idx, SparseIndex = SparseIndex>,
{
    fn number_of_defined_diagonal_values(&self) -> Self::Index {
        self.csr.number_of_defined_diagonal_values()
    }
}

impl<SparseIndex, Idx> AsRef<CSR2D<SparseIndex, Idx, Idx>>
    for UpperTriangularCSR2D<SparseIndex, Idx>
{
    fn as_ref(&self) -> &CSR2D<SparseIndex, Idx, Idx> {
        self.csr.as_ref()
    }
}

impl<SparseIndex: Zero, Idx: Zero> Default for UpperTriangularCSR2D<SparseIndex, Idx> {
    fn default() -> Self {
        Self { csr: SquareCSR2D::default() }
    }
}

impl<SparseIndex: IntoUsize, Idx: PositiveInteger + IntoUsize + Zero> SparseMatrixMut
    for UpperTriangularCSR2D<SparseIndex, Idx>
where
    Self: SparseMatrix<SparseIndex = SparseIndex, Coordinates = (Idx, Idx)>
        + MatrixMut<Entry = Self::Coordinates>,
    SquareCSR2D<SparseIndex, Idx>: SparseMatrixMut<SparseIndex = SparseIndex, MinimalShape = Idx>,
{
    type MinimalShape = Idx;

    fn with_sparse_capacity(number_of_values: Self::SparseIndex) -> Self {
        Self { csr: SquareCSR2D::with_sparse_capacity(number_of_values) }
    }

    fn with_sparse_shaped_capacity(
        shape: Self::MinimalShape,
        number_of_values: Self::SparseIndex,
    ) -> Self {
        Self { csr: SquareCSR2D::with_sparse_shaped_capacity(shape, number_of_values) }
    }
}

impl<SparseIndex: PositiveInteger + IntoUsize, Idx: PositiveInteger + IntoUsize> SparseMatrix
    for UpperTriangularCSR2D<SparseIndex, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    SquareCSR2D<SparseIndex, Idx>:
        SparseMatrix<Coordinates = Self::Coordinates, SparseIndex = SparseIndex>,
{
    type SparseIndex = <SquareCSR2D<SparseIndex, Idx> as SparseMatrix>::SparseIndex;
    type SparseCoordinates<'a>
        = <SquareCSR2D<SparseIndex, Idx> as SparseMatrix>::SparseCoordinates<'a>
    where
        Self: 'a;

    fn sparse_coordinates(&self) -> Self::SparseCoordinates<'_> {
        self.csr.sparse_coordinates()
    }

    fn is_empty(&self) -> bool {
        self.csr.is_empty()
    }
}

impl<SparseIndex: PositiveInteger + IntoUsize, Idx: PositiveInteger + IntoUsize> SizedSparseMatrix
    for UpperTriangularCSR2D<SparseIndex, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    SquareCSR2D<SparseIndex, Idx>:
        SizedSparseMatrix<Coordinates = Self::Coordinates, SparseIndex = SparseIndex>,
{
    fn number_of_defined_values(&self) -> Self::SparseIndex {
        self.csr.number_of_defined_values()
    }
}

impl<SparseIndex: PositiveInteger + IntoUsize, Idx: PositiveInteger + IntoUsize>
    RankSelectSparseMatrix for UpperTriangularCSR2D<SparseIndex, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    SquareCSR2D<SparseIndex, Idx>:
        RankSelectSparseMatrix<Coordinates = Self::Coordinates, SparseIndex = SparseIndex>,
{
    fn rank(&self, coordinates: &Self::Coordinates) -> Self::SparseIndex {
        self.csr.rank(coordinates)
    }

    fn select(&self, sparse_index: Self::SparseIndex) -> Self::Coordinates {
        self.csr.select(sparse_index)
    }
}

impl<SparseIndex: PositiveInteger + IntoUsize, Idx: PositiveInteger + IntoUsize> SparseMatrix2D
    for UpperTriangularCSR2D<SparseIndex, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    SquareCSR2D<SparseIndex, Idx>:
        SparseMatrix2D<RowIndex = Idx, ColumnIndex = Idx, SparseIndex = SparseIndex>,
{
    type SparseRow<'a>
        = <SquareCSR2D<SparseIndex, Idx> as SparseMatrix2D>::SparseRow<'a>
    where
        Self: 'a;
    type SparseColumns<'a>
        = <SquareCSR2D<SparseIndex, Idx> as SparseMatrix2D>::SparseColumns<'a>
    where
        Self: 'a;
    type SparseRows<'a>
        = <SquareCSR2D<SparseIndex, Idx> as SparseMatrix2D>::SparseRows<'a>
    where
        Self: 'a;
    type EmptyRowIndices<'a>
        = <SquareCSR2D<SparseIndex, Idx> as SparseMatrix2D>::EmptyRowIndices<'a>
    where
        Self: 'a;
    type NonEmptyRowIndices<'a>
        = <SquareCSR2D<SparseIndex, Idx> as SparseMatrix2D>::NonEmptyRowIndices<'a>
    where
        Self: 'a;

    fn sparse_row(&self, row: Self::RowIndex) -> Self::SparseRow<'_> {
        self.csr.sparse_row(row)
    }

    fn sparse_columns(&self) -> Self::SparseColumns<'_> {
        self.csr.sparse_columns()
    }

    fn sparse_rows(&self) -> Self::SparseRows<'_> {
        self.csr.sparse_rows()
    }

    fn empty_row_indices(&self) -> Self::EmptyRowIndices<'_> {
        self.csr.empty_row_indices()
    }

    fn non_empty_row_indices(&self) -> Self::NonEmptyRowIndices<'_> {
        self.csr.non_empty_row_indices()
    }

    fn number_of_empty_rows(&self) -> Self::RowIndex {
        self.csr.number_of_empty_rows()
    }

    fn number_of_non_empty_rows(&self) -> Self::RowIndex {
        self.csr.number_of_non_empty_rows()
    }
}

impl<SparseIndex: PositiveInteger + IntoUsize, Idx: PositiveInteger + IntoUsize>
    SizedRowsSparseMatrix2D for UpperTriangularCSR2D<SparseIndex, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    SquareCSR2D<SparseIndex, Idx>:
        SizedRowsSparseMatrix2D<RowIndex = Idx, ColumnIndex = Idx, SparseIndex = SparseIndex>,
{
    type SparseRowSizes<'a>
        = <SquareCSR2D<SparseIndex, Idx> as SizedRowsSparseMatrix2D>::SparseRowSizes<'a>
    where
        Self: 'a;

    fn number_of_defined_values_in_row(&self, row: Self::RowIndex) -> Self::ColumnIndex {
        self.csr.number_of_defined_values_in_row(row)
    }

    fn sparse_row_sizes(&self) -> Self::SparseRowSizes<'_> {
        self.csr.sparse_row_sizes()
    }
}

impl<SparseIndex: PositiveInteger + IntoUsize, Idx: PositiveInteger + IntoUsize> SizedSparseMatrix2D
    for UpperTriangularCSR2D<SparseIndex, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    SquareCSR2D<SparseIndex, Idx>:
        SizedSparseMatrix2D<RowIndex = Idx, ColumnIndex = Idx, SparseIndex = SparseIndex>,
{
    fn rank_row(&self, row: Idx) -> Self::SparseIndex {
        self.csr.rank_row(row)
    }

    fn select_column(&self, sparse_index: Self::SparseIndex) -> Self::ColumnIndex {
        self.csr.select_column(sparse_index)
    }

    fn select_row(&self, sparse_index: Self::SparseIndex) -> Self::RowIndex {
        self.csr.select_row(sparse_index)
    }
}

impl<SparseIndex: PositiveInteger + IntoUsize, Idx: PositiveInteger + IntoUsize> MatrixMut
    for UpperTriangularCSR2D<SparseIndex, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    SquareCSR2D<SparseIndex, Idx>: MatrixMut<Entry = Self::Coordinates, Error = MutabilityError<SquareCSR2D<SparseIndex, Idx>>>
        + Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
{
    type Entry = Self::Coordinates;
    type Error = crate::error::MutabilityError<Self>;

    fn add(&mut self, (row, column): Self::Entry) -> Result<(), Self::Error> {
        if row > column {
            return Err(MutabilityError::OutOfBounds((row, column)));
        }
        self.csr.add((row, column))?;

        Ok(())
    }
}

impl<
        SparseIndex: PositiveInteger + IntoUsize,
        Idx: PositiveInteger + IntoUsize + TryFrom<SparseIndex>,
    > TransposableMatrix2D<SquareCSR2D<SparseIndex, Idx>> for UpperTriangularCSR2D<SparseIndex, Idx>
where
    Self: Matrix2D<RowIndex = Idx, ColumnIndex = Idx>,
    SquareCSR2D<SparseIndex, Idx>:
        TransposableMatrix2D<SquareCSR2D<SparseIndex, Idx>, RowIndex = Idx, ColumnIndex = Idx>,
{
    fn transpose(&self) -> SquareCSR2D<SparseIndex, Idx> {
        self.csr.transpose()
    }
}

impl<
        SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
        Idx: PositiveInteger + IntoUsize + TryFromUsize + TryFrom<SparseIndex>,
    > Symmetrize<SymmetricCSR2D<SparseIndex, Idx>> for UpperTriangularCSR2D<SparseIndex, Idx>
where
    Self: SparseSquareMatrix<Index = Idx, SparseIndex = SparseIndex>,
    SquareCSR2D<SparseIndex, Idx>:
        SparseMatrix2D<RowIndex = Idx, ColumnIndex = Idx, SparseIndex = SparseIndex>,
{
    fn symmetrize(&self) -> SymmetricCSR2D<SparseIndex, Idx> {
        // We initialize the transposed matrix.
        let number_of_expected_column_indices = (self.number_of_defined_values().into_usize()
            - self.number_of_defined_diagonal_values().into_usize())
            * 2
            + self.number_of_defined_diagonal_values().into_usize();
        let mut symmetric: CSR2D<SparseIndex, Idx, Idx> = CSR2D {
            offsets: vec![SparseIndex::ZERO; self.order().into_usize() + 1],
            number_of_columns: self.order(),
            number_of_rows: self.order(),
            column_indices: vec![Idx::ZERO; number_of_expected_column_indices],
            number_of_non_empty_rows: Idx::ZERO,
        };

        // First, we proceed to compute the number of elements in each column.
        for (row, column) in self.sparse_coordinates() {
            // TODO! IF YOU INITIALIZE OFFSETS WITH THE OUT BOUND DEGREES, THERE IS NO NEED
            // FOR ALL OF THE SPARSE ROW ACCESSES!
            symmetric.offsets[row.into_usize() + 1] += SparseIndex::ONE;
            symmetric.offsets[column.into_usize() + 1] += SparseIndex::from(row != column);
        }

        // Then, we compute the prefix sum of the degrees to get the offsets.
        let mut prefix_sum = SparseIndex::ZERO;
        for offset in &mut symmetric.offsets {
            prefix_sum += *offset;
            symmetric.number_of_non_empty_rows +=
                if *offset > SparseIndex::ZERO { Idx::ONE } else { Idx::ZERO };
            *offset = prefix_sum;
        }

        // Finally, we fill the column indices.
        let mut degree = vec![SparseIndex::ZERO; self.order().into_usize()];
        for (row, column) in self.sparse_coordinates() {
            let edges: Vec<(Idx, Idx)> = if row == column {
                vec![(row, column)]
            } else {
                vec![(row, column), (column, row)]
            };
            for (i, j) in edges {
                let current_degree: &mut SparseIndex = &mut degree[i.into_usize()];
                let index = *current_degree + symmetric.offsets[i.into_usize()];
                symmetric.column_indices[index.into_usize()] = j;
                *current_degree += SparseIndex::ONE;
            }
        }

        debug_assert_eq!(
            symmetric.number_of_defined_values().into_usize(),
            number_of_expected_column_indices,
            "The number of inserted values is not the expected one. Original number of values: {}. Diagonals: {}",
            self.number_of_defined_values(),
            self.number_of_defined_diagonal_values()
        );

        SymmetricCSR2D {
            csr: SquareCSR2D {
                csr: symmetric,
                number_of_diagonal_values: self.number_of_defined_diagonal_values(),
            },
        }
    }
}
