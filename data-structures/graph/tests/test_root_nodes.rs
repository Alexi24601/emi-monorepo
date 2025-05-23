//! Test submodule for the `RootNodes` trait.

use algebra::impls::SquareCSR2D;
use graph::{
    prelude::{
        Builder, DiEdgesBuilder, DiGraph, GenericMonoplexMonopartiteGraphBuilder,
        GenericVocabularyBuilder, MonopartiteGraph, MonoplexGraph, RootNodes,
    },
    traits::{
        EdgesBuilder, MonopartiteGraphBuilder, MonoplexGraphBuilder, VocabularyBuilder,
        topological_sorting::TopologicalSortingError,
    },
};
use sorted_vec::prelude::SortedVec;

#[test]
fn test_no_root_nodes() -> Result<(), Box<dyn std::error::Error>> {
    let nodes: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
    let edges: Vec<(usize, usize)> =
        vec![(1, 2), (1, 3), (2, 1), (2, 3), (3, 4), (4, 5), (5, 0), (5, 1), (5, 3)];
    let nodes: SortedVec<usize> = GenericVocabularyBuilder::default()
        .expected_number_of_symbols(nodes.len())
        .symbols(nodes.into_iter().enumerate())
        .build()?;
    let edges: SquareCSR2D<usize, usize> = DiEdgesBuilder::default()
        .expected_number_of_edges(edges.len())
        .expected_shape(nodes.len())
        .edges(edges.into_iter())
        .build()?;
    let graph: DiGraph<usize> =
        GenericMonoplexMonopartiteGraphBuilder::default().nodes(nodes).edges(edges).build()?;

    assert_eq!(graph.number_of_nodes(), 6);
    assert_eq!(graph.number_of_edges(), 9);

    assert_eq!(graph.root_nodes(), Vec::new(), "There should be no root nodes");
    assert_eq!(
        graph.topological_sort_from_roots().unwrap_err(),
        TopologicalSortingError::UnreachableNodes
    );

    Ok(())
}

#[test]
fn test_root_nodes() -> Result<(), Box<dyn std::error::Error>> {
    let nodes: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
    let edges: Vec<(usize, usize)> = vec![(1, 2), (1, 3), (2, 3), (3, 4), (4, 5)];
    let nodes: SortedVec<usize> = GenericVocabularyBuilder::default()
        .expected_number_of_symbols(nodes.len())
        .symbols(nodes.into_iter().enumerate())
        .build()
        .unwrap();
    let edges: SquareCSR2D<usize, usize> = DiEdgesBuilder::default()
        .expected_number_of_edges(edges.len())
        .expected_shape(nodes.len())
        .edges(edges.into_iter())
        .build()
        .unwrap();
    let graph: DiGraph<usize> = GenericMonoplexMonopartiteGraphBuilder::default()
        .nodes(nodes)
        .edges(edges)
        .build()
        .unwrap();

    assert_eq!(graph.number_of_nodes(), 6);
    assert_eq!(graph.number_of_edges(), 5);

    assert_eq!(graph.root_nodes(), vec![0, 1]);
    assert_eq!(graph.topological_sort_from_roots()?, vec![0, 1, 2, 3, 4, 5],);

    Ok(())
}
