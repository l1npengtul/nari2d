//! See [Mesh](crate::mesh::Mesh).

use crate::{point::TwoDimensionalPoint, prelude::*};
use std::collections::HashSet;

/// # Connected components
impl<P: TwoDimensionalPoint> Mesh<P> {
    ///
    /// Finds the connected set of faces starting from the given face.
    ///
    pub fn connected_component(&self, start_face_id: FaceID) -> HashSet<FaceID> {
        self.connected_component_with_limit(start_face_id, &|_| false)
    }

    ///
    /// Finds all the sets of connected faces.
    ///
    pub fn connected_components(&self) -> Vec<HashSet<FaceID>> {
        self.connected_components_with_limit(&|_| false)
    }

    ///
    /// Finds the connected set of faces starting from the given face and limited by the given limit function.
    ///
    pub fn connected_component_with_limit(
        &self,
        start_face_id: FaceID,
        limit: &dyn Fn(HalfEdgeID) -> bool,
    ) -> HashSet<FaceID> {
        let mut component = HashSet::new();
        component.insert(start_face_id);
        let mut to_be_tested = vec![start_face_id];
        while let Some(test_face) = to_be_tested.pop() {
            for halfedge_id in self.face_halfedge_iter(test_face) {
                if !limit(halfedge_id) {
                    if let Some(face_id) =
                        self.walker_from_halfedge(halfedge_id).as_twin().face_id()
                    {
                        if !component.contains(&face_id) {
                            component.insert(face_id);
                            to_be_tested.push(face_id);
                        }
                    }
                }
            }
        }
        component
    }

    ///
    /// Finds all the sets of connected faces which are limited by the given limit function.
    ///
    pub fn connected_components_with_limit(
        &self,
        limit: &dyn Fn(HalfEdgeID) -> bool,
    ) -> Vec<HashSet<FaceID>> {
        let mut components: Vec<HashSet<FaceID>> = Vec::new();
        for face_id in self.face_iter() {
            if components
                .iter()
                .find(|com| com.contains(&face_id))
                .is_none()
            {
                components.push(self.connected_component_with_limit(face_id, limit));
            }
        }
        components
    }
}
