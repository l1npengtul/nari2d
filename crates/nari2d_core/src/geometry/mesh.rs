use crate::error::{Nari2DError, NResult};
use crate::geometry::Point2d;
use ahash::RandomState;
use std::collections::HashMap;

pub type Triangle = (usize, usize, usize);

pub struct Mesh {
    points: Vec<Point2d>,
    points_mod: Vec<Point2d>,
    constraints: Vec<usize>,
    triangles: Vec<Triangle>,
    lookup_table: HashMap<Point2d, usize, RandomState>,
}

impl Mesh {
    pub fn new(points: Vec<Point2d>, constraints: Vec<usize>) -> NResult<Self> {
        // load the points into the hashmap
        let mut lookup_table = HashMap::with_capacity(points.len());
        for (index, point) in points.iter().enumerate() {
            if let Some(_) = lookup_table.insert(*point, index) {
                return Err(Nari2DError::InvalidMesh { points, error: format!("Point {} added again!", point) })
            }
        }

        let triangulation = cdt::triangulate_with_edges()
    }
}
