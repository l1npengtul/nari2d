use crate::components::position::PositionComponent;
use petgraph::Graph;

// oh god how the fuck will i do this AHHHHH
// i need to somehow make a dynamic mesh that deforms an image

#[derive(Clone, Debug, Default)]
pub struct MeshComponent {
    points: Graph<PositionComponent, ()>,
}
