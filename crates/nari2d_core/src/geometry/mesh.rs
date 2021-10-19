use crate::error::{NResult, Nari2DError};
use crate::geometry::Point2d;
use petgraph::{graph::NodeIndex, stable_graph::StableGraph};
use std::collections::BTreeMap;
use std::convert::TryFrom;

pub type Triangle = (Point2d, Point2d, Point2d);
pub type Edge = (Point2d, Point2d);
pub type TriangleIdx = (usize, usize, usize);
pub type EdgeIdx = (usize, usize);

#[derive(Copy, Clone, Debug, Hash, PartialOrd, PartialEq)]
pub enum PossibleGeometricFirstPass {
    Tri(Triangle),
    Edge(Edge),
}

impl From<[Point2d; 2]> for PossibleGeometricFirstPass {
    fn from(arr: [Point2d; 2]) -> Self {
        PossibleGeometricFirstPass::Edge((arr[0], arr[1]))
    }
}

impl From<[Point2d; 3]> for PossibleGeometricFirstPass {
    fn from(arr: [Point2d; 3]) -> Self {
        PossibleGeometricFirstPass::Tri((arr[0], arr[1], arr[2]))
    }
}

pub struct Mesh {
    mesh: StableGraph<Point2d, ()>,
    input_points: BTreeMap<Point2d, NodeIndex>,
    // add more, communist
}

impl Mesh {
    pub fn new() -> Self {
        Mesh {
            mesh: StableGraph::new(),
            input_points: BTreeMap::new(),
        }
    }

    pub fn new_with_points_and_constraints(
        points: Vec<Point2d>,
        constraints: Vec<EdgeIdx>,
    ) -> NResult<Self> {
        let constraints = {
            let mut edges = Vec::with_capacity(constraints.len());
            for edge_index in constraints {
                let real_edge: Edge = match (points.get(edge_index.0), points.get(edge_index.1)) {
                    (Some(v), Some(b)) => (*v, *b),
                    (_, _) => {
                        return Err(Nari2DError::InvalidMesh {
                            points,
                            error: "Invalid Edge Constraints".to_string(),
                        })
                    }
                };
                edges.push(real_edge)
            }
            edges
        };

        // sort the mesh
    }

    pub(crate) fn divide_points(
        points: Vec<Point2d>,
    ) -> (
        Vec<PossibleGeometricFirstPass>,
        Vec<PossibleGeometricFirstPass>,
    ) {
        let mut first_pass_vec = Vec::with_capacity(points.len());
    }
}

fn three_and_two_factorization(index: usize) -> Vec<usize> {
    let mut indecies_vec = vec![0_usize; index / 2];
    let mut index = index;

    while index > 0 {
        if index % 3 == 0 {
            indecies_vec.append(&mut vec![3_usize; index / 3]);
            break;
        }
        // more
    }

    indecies_vec
}
