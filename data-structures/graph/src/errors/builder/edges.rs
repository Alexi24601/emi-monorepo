//! Error enumeration for the edges builder.

use core::fmt::Debug;

use algebra::{
    impls::{MutabilityError, SymmetricCSR2D, UpperTriangularCSR2D},
    prelude::{IntoUsize, PositiveInteger, TryFromUsize},
};

use crate::traits::{Edges, EdgesBuilderOptions};

/// Enum representing the possible errors that can occur when building a graph.
pub enum EdgesBuilderError<E: Edges> {
    /// Error that occurs when building a edges.
    BuilderError(common_traits::builder::BuilderError<EdgesBuilderOptions>),
    /// Whether the expected number of edges was not reached or it was
    /// overreached.
    NumberOfEdges {
        /// The expected number of edges.
        expected: E::EdgeId,
        /// The actual number of edges.
        actual: E::EdgeId,
    },
    /// An error occurred while building the underlying matrix.
    MatrixError(MutabilityError<E::Matrix>),
}

impl<V: Edges> From<common_traits::builder::BuilderError<EdgesBuilderOptions>>
    for EdgesBuilderError<V>
{
    fn from(e: common_traits::builder::BuilderError<EdgesBuilderOptions>) -> Self {
        EdgesBuilderError::BuilderError(e)
    }
}

impl<E: Edges> Debug for EdgesBuilderError<E> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        <Self as core::fmt::Display>::fmt(self, f)
    }
}

impl<V: Edges> core::error::Error for EdgesBuilderError<V> {}

impl<V: Edges> core::fmt::Display for EdgesBuilderError<V> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            EdgesBuilderError::BuilderError(e) => write!(f, "{e}"),
            EdgesBuilderError::NumberOfEdges { expected, actual } => {
                write!(f, "Expected number of edges: {expected}, actual number of edges: {actual}")
            }
            EdgesBuilderError::MatrixError(e) => write!(f, "{e}"),
        }
    }
}

impl<V: Edges> From<MutabilityError<V::Matrix>> for EdgesBuilderError<V> {
    fn from(e: MutabilityError<V::Matrix>) -> Self {
        EdgesBuilderError::MatrixError(e)
    }
}

impl<
    SparseIndex: PositiveInteger + IntoUsize + TryFromUsize,
    Idx: PositiveInteger + IntoUsize + TryFromUsize + TryFrom<SparseIndex>,
> From<EdgesBuilderError<UpperTriangularCSR2D<SparseIndex, Idx>>>
    for EdgesBuilderError<SymmetricCSR2D<SparseIndex, Idx>>
{
    fn from(e: EdgesBuilderError<UpperTriangularCSR2D<SparseIndex, Idx>>) -> Self {
        match e {
            EdgesBuilderError::BuilderError(e) => EdgesBuilderError::BuilderError(e),
            EdgesBuilderError::NumberOfEdges { expected, actual } => {
                EdgesBuilderError::NumberOfEdges { expected, actual }
            }
            EdgesBuilderError::MatrixError(e) => EdgesBuilderError::MatrixError(e.into()),
        }
    }
}
