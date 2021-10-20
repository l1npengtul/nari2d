use crate::error::{NResult, Nari2DError};
use crate::geometry::point2d::float_cmp;
use crate::geometry::Point2d;
use petgraph::{graph::NodeIndex, stable_graph::StableGraph};
use rstar::RTree;
use std::collections::BTreeMap;

pub type Triangle = (Point2d, Point2d, Point2d);
pub type Edge = (Point2d, Point2d);
pub type TriangleIdx = (usize, usize, usize);
pub type EdgeIdx = (usize, usize);

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
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
    input_points: RTree<Point2d>,
    // add more, communist
}

impl Mesh {
    pub fn new() -> Self {
        Mesh {
            mesh: StableGraph::new(),
            input_points: RTree::new(),
        }
    }

    // https://www.newcastle.edu.au/__data/assets/pdf_file/0019/22519/23_A-fast-algortithm-for-generating-constrained-Delaunay-triangulations.pdf
    // https://www.habrador.com/tutorials/math/14-constrained-delaunay/
    // https://forum.unity.com/threads/programming-tools-constrained-delaunay-triangulation.1066148/
    pub fn new_with_points_and_constraints(
        points: Vec<Point2d>,
        constraints: Vec<EdgeIdx>,
    ) -> NResult<Self> {
        if points.len() > 3 {
            return Err(Nari2DError::InvalidMesh {
                points,
                error: "Not Enough Points!".to_string(),
            });
        }

        let mut points = points;
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

        // create the graph & tree
        let input_point_tree = RTree::bulk_load(points.clone());
        let mesh = StableGraph::new();

        // normalize points

        // start out by stripping the input points
        let (x_min, y_min, x_max, y_max) = {
            let mut xv = vec![0_f32; points.len()];
            let mut yv = vec![0_f32; points.len()];
            for pt in &points {
                xv.push(pt.x());
                yv.push(pt.y());
            }
            xv.sort_by(|a, b| float_cmp(a, b));
            yv.sort_by(|a, b| float_cmp(a, b));
            (xv[0], yv[0], xv[xv.len() - 1], yv[yv.len() - 1])
        };

        let d_max = {
            let x_diff = x_max - x_min;
            let y_diff = y_max - y_min;
            x_diff.max(y_diff)
        };

        let mut n_points = points
            .iter()
            .map(|p| Point2d::new((p.x() - x_min) / d_max, (p.y() - y_min) / d_max))
            .collect::<Vec<Point2d>>();

        // start to sort points into their own "bins"
        let num_bins = n_points.len().nth_root(4);
        let mut bins = vec![vec![]; num_bins]; // double indirection OOF... i wonder if this is why we use the tree instead...
    }

    pub fn rebulk_tree(&mut self) {
        self.input_points = RTree::bulk_load(
            self.input_points
                .iter()
                .map(|x| *x)
                .collect::<Vec<Point2d>>(),
        );
    }
}

fn three_and_two_splitter(index: usize) -> Vec<usize> {
    let mut indecies_vec = vec![0_usize; index / 2];
    let mut index = index;

    while index > 0 {
        if index % 3 == 0 && index % 2 == 0 || index % 3 == 0 {
            index -= 3;
            indecies_vec.push(3);
        } else {
            index -= 2;
            indecies_vec.push(2);
        }
    }

    indecies_vec
}
