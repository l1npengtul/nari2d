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
pub struct Triangle {
    pub(crate) point_idx: [NodeIndex; 3],
}

impl Triangle {}

#[derive(Copy, Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
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

pub struct Mesh {
    mesh: StableGraph<Point2d, ()>,
    mesh_mod: StableGraph<Point2d, ()>,
    points_store: HashMap<Point2d, NodeIndex>,
    triangles: Vec<Triangle>,
    // add more, communist
}

impl Mesh {
    pub fn new() -> Self {
        Mesh {
            mesh: StableGraph::default(),
            mesh_mod: StableGraph::default(),
            points_store: HashMap::default(),
            triangles: vec![],
        }
    }

    // https://www.newcastle.edu.au/__data/assets/pdf_file/0019/22519/23_A-fast-algortithm-for-generating-constrained-Delaunay-triangulations.pdf
    // https://www.habrador.com/tutorials/math/14-constrained-delaunay/
    // https://forum.unity.com/threads/programming-tools-constrained-delaunay-triangulation.1066148/
    pub fn with_points_and_constraints(
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

        // create the meshes
        let mut mesh = StableGraph::with_capacity(
            points.len() + 3,
            (points.len() as f32 * 1.5_f32).round() as usize + 3,
        );
        let mut points_store = HashMap::with_capacity(points.len() + 3);
        let triangles = Vec::with_capacity((points.len() as f32 / 3_f32) as usize + 1);

        let supertriangle = calculate_super_triangle(&points);
        supertriangle.to_vec().for_each(|(pt_from, pt_to)| {
            let from_id = match points_store.get(&pt_from) {
                Some(idx) => idx.clone(),
                None => {
                    let id = mesh.add_node(pt_from);
                    points_store.insert(pt_from, id.clone());
                    id
                }
            };

            let to_id = match points_store.get(&pt_to) {
                Some(idx) => idx.clone(),
                None => {
                    let id = mesh.add_node(pt_to);
                    points_store.insert(pt_to, id.clone());
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
