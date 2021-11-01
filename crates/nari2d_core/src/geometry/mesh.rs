use crate::{
    error::{NResult, Nari2DError},
    geometry::Point2d,
};
use ahash::RandomState;
use std::collections::HashMap;
use cdt::Error;

pub type Triangle = (usize, usize, usize);
pub type Edge = (usize, usize);

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Mesh {
    points_mod: Vec<Point2d>,
    constraints: Vec<Edge>,
    triangulation: Vec<Triangle>,
    lookup_table: HashMap<Point2d, usize, RandomState>,
}

impl Mesh {
    /// # Errors
    pub fn new(input_points: Vec<Point2d>, constraints: Vec<Edge>) -> NResult<Self> {
        let mut input_points = input_points;
        input_points.sort();
        input_points.dedup();

        let pts = input_points
            .as_slice()
            .iter()
            .map(|pt| {
                let (x, y) = (f64::from(pt.x()), f64::from(pt.y()));
                (x, y)
            })
            .collect::<Vec<(f64, f64)>>();

        let triangulation = match cdt::triangulate_with_edges(&pts, constraints.as_slice()) {
            Ok(tri) => tri as Vec<Triangle>,
            Err(why) => {
                return Err(Nari2DError::InvalidMesh {
                    points: input_points,
                    error: why.to_string(),
                })
            }
        };

        let mut lookup_table =
            HashMap::with_capacity_and_hasher(input_points.len(), RandomState::new());
        for triangle in &triangulation {
            lookup_table.insert(input_points[triangle.0], triangle.0);
            lookup_table.insert(input_points[triangle.1], triangle.1);
            lookup_table.insert(input_points[triangle.2], triangle.2);
        }

        Ok(Mesh {
            points_mod: lookup_table.keys().into_iter().copied().collect(),
            constraints,
            triangulation,
            lookup_table,
        })
    }

    pub fn recalculate(&mut self, points: Vec<Point2d>) -> NResult<()> {
        // keep copies
        let rollback =
    }
}

// input must be sorted and dedup'd
fn concave_hull(points: &Vec<Point2d>) -> NResult<Vec<Edge>> {
    // first triangulate the points
    let point_array = points.iter().map(|pt| (pt.x() as f64, pt.y() as f64)).collect::<&[(f64, f64)]>();
    let triangulation = match cdt::triangulate_points(point_array) {
        Ok(tri) => {
            // make edges
            tri.into_iter().map(|triangle| {
                [
                    (triangle[0], triangle[1]),
                    (triangle[1], triangle[2]),
                    (triangle[2], triangle[0]),
                ]
            }).flatten().collect::<Vec<Edge>>()
        }
        Err(why) => {
            return Err(Nari2DError::InvalidMesh { points: points.clone(), error: format!("Concave Hull Error: {}", why.to_string()) })
        }
    };


}