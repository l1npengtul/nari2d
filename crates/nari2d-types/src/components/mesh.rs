use petgraph::Graph;
use crate::components::position::PositionComponent;

#[derive(Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct MeshComponent {
    points: Graph<PositionComponent, ()>,
}
