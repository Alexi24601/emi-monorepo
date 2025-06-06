//! A trait defining a conversion between a source symbol and a destination
//! symbol.

use core::fmt::Debug;

use algebra::prelude::Symbol;

use crate::errors::builder::vocabulary::VocabularyBuilderError;

/// Trait defining a conversion between a source symbol and a destination
/// symbol.
pub trait Vocabulary: Debug + Clone {
    /// The source symbol.
    type SourceSymbol: Symbol;
    /// The destination symbol.
    type DestinationSymbol: Symbol;
    /// Iterator over the source symbols.
    type Sources<'a>: Iterator<Item = Self::SourceSymbol>
    where
        Self: 'a;
    /// Iterator over the destination symbols.
    type Destinations<'a>: Iterator<Item = Self::DestinationSymbol>
    where
        Self: 'a;

    /// Converts a source symbol into a destination symbol.
    fn convert(&self, source: &Self::SourceSymbol) -> Option<Self::DestinationSymbol>;

    /// Returns whether the vocabulary is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of entries in the vocabulary.
    fn len(&self) -> usize;

    /// Returns an iterator over the source symbols.
    fn sources(&self) -> Self::Sources<'_>;

    /// Returns an iterator over the destination symbols.
    fn destinations(&self) -> Self::Destinations<'_>;
}

/// Trait defining a conversion between a source symbol and a destination symbol
/// reference.
pub trait VocabularyRef: Vocabulary {
    /// Iterator over the references of the destination symbols.
    type DestinationRefs<'a>: Iterator<Item = &'a Self::DestinationSymbol>
    where
        Self: 'a;

    /// Converts a source symbol into a destination symbol.
    fn convert_ref(&self, source: &Self::SourceSymbol) -> Option<&Self::DestinationSymbol>;

    /// Returns an iterator over the references of the destination symbols.
    fn destination_refs(&self) -> Self::DestinationRefs<'_>;
}

/// Trait defining a bidirectional conversion between a source symbol and a
/// destination symbol.
pub trait BidirectionalVocabulary: Vocabulary {
    /// Converts a destination symbol into a source symbol.
    fn invert(&self, destination: &Self::DestinationSymbol) -> Option<Self::SourceSymbol>;
}

/// Trait defining a bidirectional conversion between a destination symbol and a
/// source symbol reference.
pub trait BidirectionalVocabularyRef: BidirectionalVocabulary + VocabularyRef {
    /// Iterator over the references of the source symbols.
    type SourceRefs<'a>: Iterator<Item = &'a Self::SourceSymbol>
    where
        Self: 'a;

    /// Converts a destination symbol into a source symbol.
    fn invert_ref(&self, destination: &Self::DestinationSymbol) -> Option<&Self::SourceSymbol>;

    /// Returns an iterator over the references of the source symbols.
    fn source_refs(&self) -> Self::SourceRefs<'_>;
}

/// Trait defining a growable vocabulary.
pub trait GrowableVocabulary: Vocabulary + Default {
    /// Creates a new growable vocabulary.
    fn new() -> Self;

    /// Creates a new growable vocabulary with the specified capacity.
    fn with_capacity(capacity: usize) -> Self;

    /// Adds a new entry to the vocabulary.
    ///
    /// # Arguments
    ///
    /// * `source` - The source symbol.
    /// * `destination` - The destination symbol.
    ///
    /// # Errors
    ///
    /// * If the source symbol is already in the vocabulary.
    /// * If the destination symbol is already in the vocabulary.
    fn add(
        &mut self,
        source: Self::SourceSymbol,
        destination: Self::DestinationSymbol,
    ) -> Result<(), VocabularyBuilderError<Self>>;
}
