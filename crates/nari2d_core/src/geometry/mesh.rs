// Modified from delaunator crate to support f32 types.

use crate::{
    error::{NResult, Nari2DError},
    geometry::Point2d,
};
use cdt::Error;
use delaunator::{triangulate, Point};
use rstar::RTree;
use std::ops::{Index, IndexMut};

pub struct EditableTriangleMesh {
    points: Vec<Point2d>,
    triangles: Vec<(usize, usize, usize)>,
    neighbours: Vec<(usize, usize, usize)>,
    constraints: Vec<usize>,
}

pub struct VertexTriangleMesh {
    points: Vec<Point2d>,
    triangles: Vec<(usize, usize, usize)>,
    constraints: Vec<usize>,
}

impl VertexTriangleMesh {
    pub fn new(points: Vec<Point2d>, edges: Vec<usize>) -> NResult<Self> {
        // pre clean points
        // dedup via relative epsilon
        let points_len_before = points.len();
        let mut points = points;
        points.sort();
        points.dedup();
        if points.len() != points_len_before {
            return Err(Nari2DError::MeshGenerationCleanup {
                points,
                error: "Duplicates within Epsilon".to_string(),
            });
        }

        let delaunator = points
            .iter()
            .map(|pt| (pt.x() as f64, pt.y() as f64))
            .collect::<Vec<(f64, f64)>>();

        let triangulation = match cdt::triangulate_with_edges(&delaunator, &edges) {
            Ok(tris) => tris,
            Err(why) => {
                return Err(Nari2DError::MeshTriangulation {
                    error: why.to_string(),
                })
            }
        };

        Ok(VertexTriangleMesh {
            points,
            triangles: triangulation,
            constraints: edges,
        })
    }

    pub fn re_triangulate(
        &mut self,
        new_points: Vec<Point2d>,
        new_edges: Option<Vec<usize>>,
    ) -> NResult<()> {
        let mut new_points = new_points;
        new_points.append(&mut self.points);
        new_points.sort();
        new_points.dedup();
        if new_points.len() != points_len_before {
            return Err(Nari2DError::MeshGenerationCleanup {
                points: new_points,
                error: "Duplicates within Epsilon".to_string(),
            });
        }

        let new_edges = match new_edges {
            Some(mut e) => {
                e.append(&mut self.constraints);
                e.sort();
                e.dedup();
                &e
            }
            None => &self.constraints,
        };

        let delaunator = new_points
            .iter()
            .map(|pt| (pt.x() as f64, pt.y() as f64))
            .collect::<Vec<(f64, f64)>>();

        let triangulation = match cdt::triangulate_with_edges(&delaunator, new_edges) {
            Ok(tris) => tris,
            Err(why) => {
                return Err(Nari2DError::MeshTriangulation {
                    error: why.to_string(),
                })
            }
        };

        self.constraints = new_edges.clone();
        self.triangles = triangulation;
        self.points = new_points;
        Ok(())
    }

    pub fn modify_point(&mut self, index: usize, new_point: Point2d) -> Option<Point2d> {
        match self.points.get_mut(index) {
            Some(_) => {
                let temp = self.points[index];
                self.points[index] = new_point;
                Some(temp)
            }
            None => None,
        }
    }
}

impl<'a> AsRef<TriangleMeshViewer<'a>> for VertexTriangleMesh {
    fn as_ref(&self) -> &'a TriangleMeshViewer {
        &TriangleMeshViewer {
            data: &self.points,
            index: 0,
        }
    }
}

impl<'a> AsMut<TriangleMeshViewerMut<'a>> for VertexTriangleMesh {
    fn as_mut(&mut self) -> &mut TriangleMeshViewerMut<'a> {
        &mut TriangleMeshViewerMut {
            data: &mut self.points,
            index: 0,
        }
    }
}

pub struct TriangleMeshViewerMut<'a> {
    data: &'a mut [Point2d],
    index: usize,
}

impl<'a> TriangleMeshViewerMut<'a> {
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<'a> Index<usize> for TriangleMeshViewerMut<'a> {
    type Output = Point2d;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<'a> IndexMut<usize> for TriangleMeshViewerMut<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<'a> Iterator for TriangleMeshViewerMut<'a> {
    type Item = Point2d;

    fn next(&mut self) -> Option<Self::Item> {
        let output = match self.data.get(self.index) {
            None => return None,
            Some(pt) => *pt,
        };

        self.index += 1;
        Some(output)
    }
}

impl<'a> IntoIterator for TriangleMeshViewerMut<'a> {
    type Item = Point2d;
    type IntoIter = std::vec::IntoIter<Point2d>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

pub struct TriangleMeshViewer<'a> {
    data: &'a [Point2d],
    index: usize,
}

impl<'a> TriangleMeshViewer<'a> {
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<'a> Index<usize> for TriangleMeshViewer<'a> {
    type Output = Point2d;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<'a> Iterator for TriangleMeshViewer<'a> {
    type Item = Point2d;

    fn next(&mut self) -> Option<Self::Item> {
        let output = match self.data.get(self.index) {
            None => return None,
            Some(pt) => *pt,
        };

        self.index += 1;
        Some(output)
    }
}

impl<'a> IntoIterator for TriangleMeshViewer<'a> {
    type Item = Point2d;
    type IntoIter = std::vec::IntoIter<Point2d>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

// algorithm from https://www.researchgate.net/publication/220868874_Concave_hull_A_k-nearest_neighbours_approach_for_the_computation_of_the_region_occupied_by_a_set_of_points
pub fn concave_hull(points: Vec<Point2d>) -> NResult<Vec<usize>> {
    let mut points = points;
    points.sort();
    points.dedup();

    let rtree = RTree::bulk_load(points.clone());
    // get lowest y by getting the point closest to f32::MIN
    let lowest_point = match rtree.nearest_neighbor(&[0_f32, f32::MIN]) {
        Some(pt) => *pt,
        None => {
            return Err(Nari2DError::MeshConcaveCalculation {
                points,
                error: "No lowest point!".to_string(),
            })
        }
    };
}
