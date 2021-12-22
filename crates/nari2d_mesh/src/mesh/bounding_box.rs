//! See [Mesh](crate::mesh::Mesh).

use crate::mesh::math::*;
use crate::mesh::Mesh;
use crate::point::{Scalar, TwoDimensionalPoint};

/// # Bounding box
impl<P: TwoDimensionalPoint> Mesh<P> {
    /// Returns minimum and maximum coordinates of the axis aligned bounding box of the mesh.
    pub fn extreme_coordinates(&self) -> (P, P) {
        let mut min_coordinates = P::new_point(P::SCALAR::min(), P::SCALAR::min());
        let mut max_coordinates = P::new_point(P::SCALAR::max(), P::SCALAR::max());
        for vertex_id in self.vertex_iter() {
            let position = self.vertex_position(vertex_id);
            for i in 0..3 {
                min_coordinates[i] = min_coordinates[i].min(position[i]);
                max_coordinates[i] = max_coordinates[i].max(position[i]);
            }
        }
        (min_coordinates, max_coordinates)
    }

    /// Returns the center of the smallest axis aligned box which contains the entire mesh, ie. the axis aligned bounding box.
    pub fn axis_aligned_bounding_box_center(&self) -> P {
        let (min_coord, max_coord) = self.extreme_coordinates();
        0.5 * (max_coord + min_coord)
    }

    /// Returns the smallest axis aligned box which contains the entire mesh, ie. the axis aligned bounding box.
    pub fn axis_aligned_bounding_box(&self) -> Mesh<P> {
        let (min_coord, max_coord) = self.extreme_coordinates();
        let mut mesh = crate::MeshBuilder::new().cube().build().unwrap();
        let scale = 0.5 * (max_coord - min_coord);
        mesh.non_uniform_scale(scale.x, scale.y, scale.z);
        let translation = 0.5 * (max_coord + min_coord);
        mesh.translate(translation);
        mesh
    }
}
