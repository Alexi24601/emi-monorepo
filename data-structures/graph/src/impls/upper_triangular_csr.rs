//! Submodule implementing Edges-related traits for
//! [`SquaredUpperTriangularCSR2D`].

use algebra::prelude::*;

use crate::{errors::builder::edges::EdgesBuilderError, prelude::*};

impl<
    SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
    Idx: PositiveInteger + TryFromUsize + IntoUsize + TryFrom<SparseIndex>,
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
    SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
    Idx: PositiveInteger + TryFromUsize + IntoUsize + TryFrom<SparseIndex>,
> GrowableEdges for UpperTriangularCSR2D<SparseIndex, Idx>
{
    type GrowableMatrix = Self;
    type Error = EdgesBuilderError<Self>;

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
