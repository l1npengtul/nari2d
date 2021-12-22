//! See [Mesh](crate::mesh::Mesh).

use crate::{mesh::intersection::utility::*, point::TwoDimensionalPoint, prelude::*};

///
/// An enum describing the types of primitives.
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Primitive {
    /// Vertex
    Vertex(VertexID),
    /// Edge
    Edge(HalfEdgeID),
    /// Face
    Face(FaceID),
}

///
/// An enum describing the types of intersections.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Intersection<P: TwoDimensionalPoint> {
    /// The intersection occurs at a single point
    Point {
        /// The [primitive](crate::mesh::intersection::Primitive) (vertex, edge or face) that is intersected
        primitive: Primitive,
        /// The point where the intersection occurs
        point: P,
    },
    /// The intersection occurs at a line piece interval
    LinePiece {
        /// The [primitive](crate::mesh::intersection::Primitive) (vertex, edge or face) that is intersected at the first end point of the line piece where the intersection occurs
        primitive0: Primitive,
        /// The [primitive](crate::mesh::intersection::Primitive) (vertex, edge or face) that is intersected at the second end point of the line piece where the intersection occurs
        primitive1: Primitive,
        /// The first end point of the line piece where the intersection occurs
        point0: P,
        /// The second end point of the line piece where the intersection occurs
        point1: P,
    },
}

/// # Intersection
impl<P: TwoDimensionalPoint> Mesh<P> {
    ///
    /// Find the [intersection](crate::mesh::intersection::Intersection) between any face in the mesh and the given ray.
    /// If the ray intersects multiple faces, the face closest to the starting point in the direction of the ray is returned.
    /// If no faces are intersected, None is returned.
    ///
    pub fn ray_intersection(
        &self,
        ray_start_point: &P,
        ray_direction: &P,
    ) -> Option<Intersection<P>> {
        let mut current: Option<Intersection<P>> = None;
        for face_id in self.face_iter() {
            let new_intersection =
                self.face_ray_intersection(face_id, ray_start_point, ray_direction);
            if let Some(Intersection::Point { point, .. }) = new_intersection {
                let new_point = point;
                if let Some(Intersection::Point { point, .. }) = current {
                    if point.distance2(*ray_start_point) > new_point.distance2(*ray_start_point) {
                        current = new_intersection;
                    }
                } else {
                    current = new_intersection;
                }
            }
        }
        current
    }

    ///
    /// Find the [intersection](crate::mesh::intersection::Intersection) between the given face and ray.
    /// If the face is not intersected by the ray, None is returned.
    ///
    pub fn face_ray_intersection(
        &self,
        face_id: FaceID,
        ray_start_point: &P,
        ray_direction: &P,
    ) -> Option<Intersection<P>> {
        let p = self.vertex_position(self.walker_from_face(face_id).vertex_id().unwrap());
        let n = self.face_normal(face_id);

        plane_ray_intersection(ray_start_point, ray_direction, &p, &n).and_then(|parameter| {
            self.face_point_intersection_when_point_in_plane(
                face_id,
                &(ray_start_point + parameter * ray_direction),
            )
        })
    }

    ///
    /// Find the [intersection](crate::mesh::intersection::Intersection) between the given face and line piece.
    /// If the face is not intersected by the line piece, None is returned.
    ///
    /// Note: Intersections, where the line piece is in the plane spanned by the face, are not yet fully handled.
    ///
    pub fn face_line_piece_intersection(
        &self,
        face_id: FaceID,
        point0: &P,
        point1: &P,
    ) -> Option<Intersection<P>> {
        let p = self.vertex_position(self.walker_from_face(face_id).vertex_id().unwrap());
        let n = self.face_direction(face_id);

        plane_line_piece_intersection(&point0, &point1, &p, &n).and_then(|intersection| {
            match intersection {
                PlaneLinepieceIntersectionResult::LineInPlane => {
                    let intersection0 =
                        self.face_point_intersection_when_point_in_plane(face_id, point0);
                    let intersection1 =
                        self.face_point_intersection_when_point_in_plane(face_id, point1);
                    if let Some(Intersection::Point {
                        point: p0,
                        primitive: primitive0,
                    }) = intersection0
                    {
                        if let Some(Intersection::Point {
                            point: p1,
                            primitive: primitive1,
                        }) = intersection1
                        {
                            Some(Intersection::LinePiece {
                                primitive0,
                                primitive1,
                                point0: p0,
                                point1: p1,
                            })
                        } else {
                            intersection0 // TODO: Should return a Intersection::LinePiece instead of a Intersection::Point
                        }
                    } else {
                        intersection1 // TODO: Should return a Intersection::LinePiece instead of a Intersection::Point
                    }
                    // TODO: Handle case where the line piece intersects the face, but the end points are both outside
                }
                PlaneLinepieceIntersectionResult::P0InPlane => {
                    self.face_point_intersection_when_point_in_plane(face_id, point0)
                }
                PlaneLinepieceIntersectionResult::P1InPlane => {
                    self.face_point_intersection_when_point_in_plane(face_id, point1)
                }
                PlaneLinepieceIntersectionResult::Intersection(point) => {
                    self.face_point_intersection_when_point_in_plane(face_id, &point)
                }
            }
        })
    }

    ///
    /// Find the [intersection](crate::mesh::intersection::Intersection) between the given vertex and the point.
    /// If the vertex is not close to the point, None is returned.
    ///
    pub fn vertex_point_intersection(
        &self,
        vertex_id: VertexID,
        point: &P,
    ) -> Option<Intersection<P>> {
        let p = self.vertex_position(vertex_id);
        if (p - point).magnitude2() < SQR_MARGIN {
            Some(Intersection::Point {
                primitive: Primitive::Vertex(vertex_id),
                point: *point,
            })
        } else {
            None
        }
    }

    ///
    /// Find the [intersection](crate::mesh::intersection::Intersection) (the primitive is either a vertex or edge) between the given edge and the point.
    /// If the edge is not close to the point, None is returned.
    ///
    pub fn edge_point_intersection(
        &self,
        halfedge_id: HalfEdgeID,
        point: &P,
    ) -> Option<Intersection<P>> {
        let (v0, v1) = self.edge_vertices(halfedge_id);
        self.vertex_point_intersection(v0, point)
            .or_else(|| self.vertex_point_intersection(v1, point))
            .or_else(|| {
                if point_line_segment_distance(
                    point,
                    &self.vertex_position(v0),
                    &self.vertex_position(v1),
                ) < MARGIN
                {
                    let twin_id = self.walker_from_halfedge(halfedge_id).twin_id().unwrap();
                    Some(Intersection::Point {
                        primitive: Primitive::Edge(if twin_id < halfedge_id {
                            twin_id
                        } else {
                            halfedge_id
                        }),
                        point: *point,
                    })
                } else {
                    None
                }
            })
    }

    ///
    /// Find the [intersection](crate::mesh::intersection::Intersection) (the primitive is either a vertex, edge or face) between the given face and the point.
    /// If the face is not close to the point, None is returned.
    ///
    pub fn face_point_intersection(&self, face_id: FaceID, point: &P) -> Option<Intersection<P>> {
        let p = self.vertex_position(self.walker_from_face(face_id).vertex_id().unwrap());
        let n = self.face_normal(face_id);
        if n.dot(point - p).abs() > MARGIN {
            return None;
        }

        self.face_point_intersection_when_point_in_plane(face_id, point)
    }

    /// Assumes that the point lies in the plane spanned by the face
    fn face_point_intersection_when_point_in_plane(
        &self,
        face_id: FaceID,
        point: &P,
    ) -> Option<Intersection<P>> {
        // Test whether the intersection point is located at the edges or vertices of the face
        for halfedge_id in self.face_halfedge_iter(face_id) {
            if let Some(intersection) = self.edge_point_intersection(halfedge_id, point) {
                return Some(intersection);
            }
        }

        // Test whether the intersection point is located inside the face
        let (a, b, c) = self.face_positions(face_id);
        let coords = barycentric(point, &a, &b, &c);
        if 0.0 < coords.0
            && coords.0 < 1.0
            && 0.0 < coords.1
            && coords.1 < 1.0
            && 0.0 < coords.2
            && coords.2 < 1.0
        {
            Some(Intersection::Point {
                primitive: Primitive::Face(face_id),
                point: *point,
            })
        } else {
            None
        }
    }
}

mod utility {
    use crate::point::TwoDimensionalPoint;

    pub const MARGIN: f64 = 0.0000001;
    pub const SQR_MARGIN: f64 = MARGIN * MARGIN;

    #[derive(Debug, PartialEq)]
    pub enum PlaneLinepieceIntersectionResult<P: TwoDimensionalPoint> {
        P0InPlane,
        P1InPlane,
        LineInPlane,
        Intersection(P),
    }

    pub fn plane_line_piece_intersection<P: TwoDimensionalPoint>(
        p0: &P,
        p1: &P,
        p: &P,
        n: &P,
    ) -> Option<PlaneLinepieceIntersectionResult<P>> {
        let ap0 = *p0 - *p;
        let ap1 = *p1 - *p;

        let d0 = n.dot(ap0);
        let d1 = n.dot(ap1);

        if d0.abs() < MARGIN && d1.abs() < MARGIN {
            // p0 and p1 lies in the plane
            Some(PlaneLinepieceIntersectionResult::LineInPlane)
        } else if d0.abs() < MARGIN {
            // p0 lies in the plane
            Some(PlaneLinepieceIntersectionResult::P0InPlane)
        } else if d1.abs() < MARGIN {
            // p1 lies in the plane
            Some(PlaneLinepieceIntersectionResult::P1InPlane)
        } else if d0.signum() != d1.signum()
        // The edge intersects the plane
        {
            // Find intersection point:
            let p01 = *p1 - *p0;
            let t = n.dot(-ap0) / n.dot(p01);
            let point = p0 + p01 * t;
            Some(PlaneLinepieceIntersectionResult::Intersection(point))
        } else {
            None
        }
    }

    pub fn plane_ray_intersection<P: TwoDimensionalPoint>(
        ray_start_point: &P,
        ray_direction: &P,
        plane_point: &P,
        plane_normal: &P,
    ) -> Option<f64> {
        let denom = plane_normal.dot(*ray_direction);
        if denom.abs() >= MARGIN {
            let parameter = plane_normal.dot(plane_point - ray_start_point) / denom;
            if parameter >= 0.0 {
                Some(parameter)
            } else {
                None
            }
        } else {
            None
        }
    }

    // Compute barycentric coordinates (u, v, w) for
    // point p with respect to triangle (a, b, c)
    pub fn barycentric<P: TwoDimensionalPoint>(p: &P, a: &P, b: &P, c: &P) -> (f64, f64, f64) {
        let v0 = b - a;
        let v1 = c - a;
        let v2 = p - a;
        let d00 = v0.dot(v0);
        let d01 = v0.dot(v1);
        let d11 = v1.dot(v1);
        let d20 = v2.dot(v0);
        let d21 = v2.dot(v1);
        let denom = d00 * d11 - d01 * d01;
        let v = (d11 * d20 - d01 * d21) / denom;
        let w = (d00 * d21 - d01 * d20) / denom;
        let u = 1.0 - v - w;
        (u, v, w)
    }

    pub fn point_line_segment_distance<P: TwoDimensionalPoint>(point: &P, p0: &P, p1: &P) -> f64 {
        let v = p1 - p0;
        let w = point - p0;

        let c1 = w.dot(v);
        if c1 <= 0.0 {
            return w.magnitude();
        }

        let c2 = v.dot(v);
        if c2 <= c1 {
            return (point - p1).magnitude();
        }

        let b = c1 / c2;
        let pb = p0 + b * v;
        (point - &pb).magnitude()
    }
}
