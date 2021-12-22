//! See [Mesh](crate::mesh::Mesh).

use crate::{
    mesh::{ids::*, Mesh},
    point::TwoDimensionalPoint,
};
use num_traits::Zero;

/// # Vertex measures
impl<P: TwoDimensionalPoint> Mesh<P> {
    /// Returns the vertex position.
    pub fn vertex_position(&self, vertex_id: VertexID) -> P {
        self.connectivity_info.position(vertex_id)
    }

    /// Returns the normal of the vertex given as the average of the normals of the neighbouring faces.
    pub fn vertex_normal(&self, vertex_id: VertexID) -> P {
        let mut normal = P::new_point(P::SCALAR::zero(), P::SCALAR::zero());
        for halfedge_id in self.vertex_halfedge_iter(vertex_id) {
            if let Some(face_id) = self.walker_from_halfedge(halfedge_id).face_id() {
                normal += self.face_normal(face_id)
            }
        }
        normal.normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MeshBuilder;

    #[test]
    fn test_vertex_normal() {
        let mesh = MeshBuilder::new().subdivided_triangle().build().unwrap();
        let computed_normal = mesh.vertex_normal(VertexID::new(0));
        assert_eq!(0.0, computed_normal.x);
        assert_eq!(0.0, computed_normal.y);
        assert_eq!(1.0, computed_normal.z);
    }
}
