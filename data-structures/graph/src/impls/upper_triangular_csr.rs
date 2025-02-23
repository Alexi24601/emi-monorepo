//! Submodule implementing Edges-related traits for [`SquaredUpperTriangularCSR2D`].

use crate::prelude::*;
use algebra::prelude::*;

impl<
        SparseIndex: PositiveInteger + IntoUsize,
        Idx: PositiveInteger + TryFromUsize + IntoUsize + From<SparseIndex>,
    > Edges for UpperTriangularCSR2D<SparseIndex, Idx>
{
    type Edge = <Self as Matrix>::Coordinates;
    type SourceNodeId = Idx;
    type DestinationNodeId = Idx;
    type EdgeId = SparseIndex;
    type Matrix = Self;

    fn matrix(&self) -> &Self::Matrix {
        self
    }
}

impl<
        SparseIndex: PositiveInteger + IntoUsize,
        Idx: PositiveInteger + TryFromUsize + IntoUsize + From<SparseIndex>,
    > GrowableEdges for UpperTriangularCSR2D<SparseIndex, Idx>
{
    type GrowableMatrix = Self;
    type Error = algebra::error::MutabilityError<Self>;

    fn matrix_mut(&mut self) -> &mut Self::GrowableMatrix {
        self
    }

    fn with_capacity(number_of_edges: Self::EdgeId) -> Self {
        <Self as SparseMatrixMut>::with_sparse_capacity(number_of_edges)
    }

    fn with_shaped_capacity(
        shape: <Self::GrowableMatrix as SparseMatrixMut>::MinimalShape,
        number_of_edges: Self::EdgeId,
    ) -> Self {
        <Self as SparseMatrixMut>::with_sparse_shaped_capacity(shape, number_of_edges)
    }
}

impl<
        SparseIndex: PositiveInteger + IntoUsize,
        Idx: PositiveInteger + TryFromUsize + IntoUsize + From<SparseIndex>,
    > DirectedEdges for UpperTriangularCSR2D<SparseIndex, Idx>
{
    type DirectedMatrix = Self;
    type NodeId = Idx;

    fn number_of_self_loops(&self) -> Idx {
        self.number_of_defined_diagonal_values()
    }
}

