//! See [Mesh](crate::mesh::Mesh).

use crate::{
    mesh::{ids::*, Mesh},
    point::TwoDimensionalPoint,
};

/// # Face measures
impl<P: TwoDimensionalPoint> Mesh<P> {
    /// Returns the positions of the face vertices.
    pub fn face_positions(&self, face_id: FaceID) -> (P, P, P) {
        let vertices = self.ordered_face_vertices(face_id);
        (
            self.vertex_position(vertices.0),
            self.vertex_position(vertices.1),
            self.vertex_position(vertices.2),
        )
    }

    /// Returns the unnormalized normal of the face.
    pub fn face_direction(&self, face_id: FaceID) -> P {
        let mut walker = self.walker_from_face(face_id);
        let p0 = self.vertex_position(walker.vertex_id().unwrap());
        walker.as_next();
        let v0 = self.vertex_position(walker.vertex_id().unwrap()) - p0;
        walker.as_next();
        let v1 = self.vertex_position(walker.vertex_id().unwrap()) - p0;

        v0.cross(v1)
    }

    /// Returns the normal of the face.
    pub fn face_normal(&self, face_id: FaceID) -> P {
        self.face_direction(face_id).normalize()
    }

    /// Returns the area of the face.
    pub fn face_area(&self, face_id: FaceID) -> f64 {
        0.5 * self.face_direction(face_id).magnitude()
    }

    /// Returns the center of the face given as the average of its vertex positions.
    pub fn face_center(&self, face_id: FaceID) -> P {
        let mut walker = self.walker_from_face(face_id);
        let p0 = self.vertex_position(walker.vertex_id().unwrap());
        walker.as_next();
        let p1 = self.vertex_position(walker.vertex_id().unwrap());
        walker.as_next();
        let p2 = self.vertex_position(walker.vertex_id().unwrap());

        (p0 + p1 + p2) / 3.0
    }
}
