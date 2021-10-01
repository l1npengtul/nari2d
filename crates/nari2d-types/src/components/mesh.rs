use crate::components::PositionComponent;
use petgraph::Graph;

#[derive(Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct MeshComponent {
    points: Graph<PositionComponent, ()>,
}
