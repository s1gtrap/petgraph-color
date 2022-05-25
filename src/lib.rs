use std::collections::HashMap;

use petgraph::graph::{IndexType, NodeIndex};
use petgraph::visit::{EdgeRef, GraphBase, IntoEdgeReferences, NodeCount};

pub struct Alloc<'a, G>(&'a G, based::Iter)
where
    G: GraphBase,
    &'a G: IntoEdgeReferences;

impl<'a, G> Alloc<'a, G>
where
    G: GraphBase,
    &'a G: IntoEdgeReferences,
{
    pub fn exhaustive(g: &'a G) -> Self
    where
        G: NodeCount,
    {
        Alloc(g, based::Iter::new(g.node_count(), 2))
    }
}

impl<'a, G> Iterator for Alloc<'a, G>
where
    G: GraphBase,
    &'a G: IntoEdgeReferences,
    <G as GraphBase>::NodeId: IndexType,
    <&'a G as GraphBase>::NodeId: IndexType,
{
    type Item = HashMap<NodeIndex<G::NodeId>, usize>;

    fn next(&mut self) -> Option<Self::Item> {
        'outer: for i in self.1.by_ref() {
            for e in self.0.edge_references() {
                if i.digits()[e.source().index()] == i.digits()[e.target().index()] {
                    continue 'outer;
                }
            }
            return Some(
                i.digits()
                    .iter()
                    .enumerate()
                    .map(|(i, d)| (NodeIndex::new(i), *d))
                    .collect(),
            );
        }
        None
    }
}

#[test]
fn test_alloc_exhaustive() {
    use petgraph::graph::Graph;

    assert_eq!(
        Alloc::exhaustive(&Graph::<(), ()>::from_edges(&[(0, 1)]))
            .take(1)
            .collect::<Vec<_>>(),
        vec![HashMap::from([
            (NodeIndex::new(0), 1),
            (NodeIndex::new(1), 0),
        ])],
    );
    assert_eq!(
        Alloc::exhaustive(&Graph::<(), ()>::from_edges(&[(0, 1), (1, 2)]))
            .take(1)
            .collect::<Vec<_>>(),
        vec![HashMap::from([
            (NodeIndex::new(0), 0),
            (NodeIndex::new(1), 1),
            (NodeIndex::new(2), 0),
        ])],
    );
    assert_eq!(
        Alloc::exhaustive(&Graph::<(), ()>::from_edges(&[(0, 1), (0, 2), (1, 2)]))
            .take(1)
            .collect::<Vec<_>>(),
        vec![HashMap::from([
            (NodeIndex::new(0), 2),
            (NodeIndex::new(1), 1),
            (NodeIndex::new(2), 0),
        ])],
    );
}
