use crate::geometry::Angle;
use crate::{
    error::{NResult, Nari2DError},
    geometry::Point2d,
};
use spade::{
    delaunay::{ConstrainedDelaunayTriangulation, FixedVertexHandle},
    kernels::FloatKernel,
};

// TODO: Run our own FloatKernel impl!
// TODO: REIMPL! https://sci-hub.se/https://www.sciencedirect.com/science/article/abs/pii/S0097849396000854
#[derive(Clone, Default)]
pub struct Mesh {
    source_mesh: ConstrainedDelaunayTriangulation<Point2d, FloatKernel>,
    mod_mesh: ConstrainedDelaunayTriangulation<Point2d, FloatKernel>,
    hull_points: Vec<Point2d>,
    center_point: Point2d,
}

impl Mesh {
    #[must_use]
    pub fn new() -> Self {
        Mesh {
            source_mesh: ConstrainedDelaunayTriangulation::with_tree_locate(),
            mod_mesh: ConstrainedDelaunayTriangulation::with_tree_locate(),
            hull_points: vec![],
            center_point: Point2d::zero(),
        }
    }

    /// # Errors
    pub fn from_hull(center: Point2d, points: Vec<Point2d>) -> NResult<Self> {
        let mut delaunay = ConstrainedDelaunayTriangulation::with_tree_locate();

        let mut previous_point_ref: Option<FixedVertexHandle> = None;
        for point in &points {
            match previous_point_ref {
                Some(pt) => {
                    let current_point_ref = delaunay.insert(*point);
                    if delaunay.can_add_constraint(pt, current_point_ref) {
                        delaunay.add_constraint(pt, current_point_ref);
                        previous_point_ref = Some(current_point_ref);
                        continue;
                    }
                    return Err(Nari2DError::InvalidMesh {
                        points,
                        error: "Intersecting Constraint".to_string(),
                    });
                }
                None => previous_point_ref = Some(delaunay.insert(*point)),
            }
        }

        if delaunay.is_degenerate() {
            return Err(Nari2DError::InvalidMesh {
                points,
                error: "Degenerate - Colinear".to_string(),
            });
        }

        Ok(Mesh {
            source_mesh: delaunay.clone(),
            mod_mesh: delaunay,
            hull_points: points,
            center_point: center,
        })
    }

    // https://www.sciencedirect.com/sdfe/reader/pii/S0925772101000475/pdf
    pub fn ruppert_refinement(&mut self, threshold: Angle, max_iter: Option<u32>) {
        let max_iter = max_iter.unwrap_or(5000);
    }
}
