use crate::{asset::AssetID, components::position::PositionComponent};
use petgraph::Graph;
use std::borrow::Cow;

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Modulation(u8, u8, u8, u8);

// oh god how the fuck will i do this AHHHHH
// i need to somehow make a dynamic mesh that deforms an image

// http://www.geosensor.net/papers/duckham08.PR.pdf
// https://en.wikipedia.org/wiki/Delaunay_triangulation
// https://www.cs.cmu.edu/~quake/tripaper/triangle2.html
// https://www.newcastle.edu.au/__data/assets/pdf_file/0017/22508/13_A-fast-algorithm-for-constructing-Delaunay-triangulations-in-the-plane.pdf
// [pain]

#[derive(Clone, Debug, Default)]
pub struct Mesh {
    points: Graph<PositionComponent, ()>,
}

#[derive(Clone, Debug)]
pub struct TextureComponent {
    height: u32,
    width: u32,
    transparency: f32,
    modulate: Modulation,
    self_modulate: Modulation,
    data_id: AssetID,
    image_data: Cow<'static, [u8]>,
    base_mesh: Mesh,
    transform_mesh: Mesh,
}
