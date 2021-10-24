use crate::geometry::bounds::Bounds;
use crate::geometry::{Angle, PointVec, Scale2d};
use crate::{
    error::{NResult, Nari2DError},
    geometry::{point2d::float_cmp, Point2d},
};
use ahash::{AHashMap, RandomState};
use petgraph::adj::DefaultIx;
use petgraph::prelude::GraphMap;
use petgraph::stable_graph::{NodeIndex, StableGraph};
use petgraph::Undirected;
use rstar::RTree;
use std::cmp::Ordering;
use std::collections::HashMap;

pub type Edge = (Point2d, Point2d);
pub type EdgeIdx = (usize, usize);

// temporary triangle structs
#[derive(Clone, Default, PartialOrd, PartialEq)]
struct TriangleList {
    pub(crate) adjacent: Vec<Option<usize>>,
    pub(crate) verts: Vec<usize>,
    pub(crate) points: Vec<Point2d>,
}

impl TriangleList {
    pub fn new(capacity: usize) -> Self {
        TriangleList {
            adjacent: Vec::with_capacity(capacity * 3),
            verts: Vec::with_capacity(capacity * 3),
            points: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, triangle: Triangle) {}
}

#[derive(Copy, Clone, PartialEq)]
struct Triangle {
    pub(crate) point_idx: [Point2d; 3],
    pub(crate) adj_tri_idx: [Option<usize>; 3],
}

impl Triangle {
    pub(crate) fn sort_ccw(&mut self) {
        // CCW is the left most point, then the highest point that isnt the first point
        self.point_idx.sort();
        // find second highest point
        let height_y_idx: usize = match float_cmp(&self.point_idx[1].y(), &self.point_idx[2].y()) {
            Ordering::Less => 2,
            Ordering::Equal => {
                // in case its equal its the rightmost point
                match float_cmp(&self.point_idx[1].x(), &self.point_idx[2].x()) {
                    Ordering::Less => 2,
                    Ordering::Equal => {
                        // this shouldn't happen at all!
                        dbg!("POINT EQUAL - WHAT???");
                        1
                    }
                    Ordering::Greater => 1,
                }
            }
            Ordering::Greater => 1,
        };

        if height_y_idx == 2 {
            self.point_idx.swap(1, 2);
        }
    }

    // CW is CCW but (a, b, c) => (a, c ,b)
    pub(crate) fn sort_cw(&mut self) {
        self.sort_ccw();
        self.point_idx.swap(1, 2);
    }
}

#[derive(Copy, Clone, Debug)]
struct RawTriangle {
    pub(crate) points: [Point2d; 3],
}

impl RawTriangle {
    pub(crate) fn to_raw(&self) -> [Point2d; 3] {
        self.points
    }

    pub(crate) fn to_vec(&self) -> PointVec {
        self.points.to_vec().into()
    }
}

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum TriEdgeIdx {
    First = 0,
    Second = 1,
    Third = 2,
}

struct TriangleEdge {
    pub(crate) triangle_idx: usize,
    pub(crate) local_tri_edge: TriEdgeIdx,
    pub(crate) edges_point_idx: [usize; 2],
}

struct Bins {
    bins: Vec<Vec<Point2d>>,
    n_bins: usize,
    grid_size: Point2d,
    bin_size: Point2d,
}

impl Bins {
    pub fn new(n_bins: usize, size: Scale2d) -> Self {
        Bins {
            bins: vec![vec![]; n_bins],
            n_bins,
            grid_size: size.into(),
            bin_size: (size / (n_bins as f32)).into(),
        }
    }

    pub fn push(&mut self, point: Point2d) {
        let i = ((0.99_f32 * (self.n_bins as f32) * point.y()) / self.grid_size.y()) as usize;
        let j = ((0.99_f32 * (self.n_bins as f32) * point.x()) / self.grid_size.x()) as usize;

        let mut b = if i % 2 == 0 {
            i * self.n_bins + j + 1
        } else {
            (i + self.n_bins) * self.n_bins - j
        };

        // normalize to zero
        b -= 1;

        self.bins[b].push(point);
    }

    pub fn get(&self, index: usize) -> Option<&Point2d> {
        self.bins.iter().flatten().nth(index)
    }

    pub fn get_bin(&self, bin: usize, index: usize) -> Option<&Point2d> {
        self.bins[bin].get(index)
    }

    pub fn to_flattened(self) -> Vec<Point2d> {
        self.bins.into_iter().flatten().collect::<Vec<Point2d>>()
    }
}

pub struct Mesh {
    mesh: StableGraph<Point2d, ()>,
    input_points: HashMap<Point2d, NodeIndex, RandomState>,
    // add more, communist
}

impl Mesh {
    pub fn new() -> Self {
        Mesh {
            mesh: StableGraph::new(),
            input_points: HashMap::default(),
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
        let mut mesh: StableGraph<Point2d, (), Undirected, DefaultIx> =
            StableGraph::with_capacity(points.len(), points.len() * 3);
        let mut input_points: HashMap<Point2d, NodeIndex, RandomState> =
            HashMap::with_capacity_and_hasher(points.len(), RandomState::new());

        let supertriangle = calculate_super_triangle(&points);
        supertriangle.to_vec().for_each(|(pt_from, pt_to)| {
            let from_id = match input_points.get(&pt_from) {
                Some(idx) => *idx,
                None => {
                    let id = mesh.add_node(pt_from);
                    input_points.insert(pt_from, id);
                    id
                }
            };

            let to_id = match input_points.get(&pt_to) {
                Some(idx) => *idx,
                None => {
                    let id = mesh.add_node(pt_to);
                    input_points.insert(pt_to, id);
                    id
                }
            };

            mesh.add_edge(from_id, to_id, ());
        });

        let mut bad_triangles = Vec::with_capacity(points.len());

        // boss makes a dollar
        // i make a dime
        // thats why my algorithms
        // run in exponential time
        // TODO: make O(n log n)
        for point in &points {
            // clear the triangle set
            bad_triangles.clear();
        }
    }
}

fn calculate_bounds(points: &Vec<Point2d>) -> Bounds {
    let mut min = Point2d::float_min();
    let mut max = Point2d::float_max();

    points.iter().for_each(|pt| {
        if pt.x() > max.x() {
            max.set_x(pt.x())
        }
        if pt.y() > max.y() {
            max.set_y(pt.y())
        }
        if pt.x() < min.x() {
            min.set_x(pt.x())
        }
        if pt.y() < min.y() {
            min.set_y(pt.y())
        }
    });

    Bounds::from_points(min, max)
}

fn calculate_super_triangle(points: &Vec<Point2d>) -> RawTriangle {
    let bounds = calculate_bounds(points);
    let length = Point2d::distance_to(
        &bounds.center(),
        Point2d::new(bounds.size_x(), bounds.size_y()),
    );

    let base_pt = Point2d::new(bounds.center_x(), bounds.center_y() + length);

    let point_a = base_pt.rotate(Angle::new(-120_f32), bounds.center());
    let point_b = base_pt;
    let point_c = base_pt.rotate(Angle::new(120_f32), bounds.center());
    RawTriangle {
        points: [point_a, point_b, point_c],
    }
}
