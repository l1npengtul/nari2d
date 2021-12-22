//!
//! Module containing [MeshBuilder](crate::mesh_builder::MeshBuilder) which has functionality to build a new [Mesh](crate::mesh::Mesh) instance.
//!

use crate::point::Scalar;
use crate::{mesh::Mesh, point::TwoDimensionalPoint};
use num_traits::Num;
use std::fmt::Debug;

/// MeshBuilder errors.
#[derive(Debug)]
pub enum MeshBuilderError {
    /// Returned when the positions haven't been specified before calling the build function.
    NoPositionsSpecified {
        /// Error reason.
        message: String,
    },
    /// Invalid file format
    InvalidFile {
        /// Error reason.
        message: String,
    },
}

///
/// `MeshBuilder` contains functionality to build a mesh from either raw data (indices, positions, normals)
/// or from simple geometric shapes (box, icosahedron, cylinder, ..) or from file source (.obj).
///
#[derive(Debug, Default)]
pub struct MeshBuilder<S: Scalar> {
    indices: Option<Vec<u32>>,
    positions: Option<Vec<S>>,
}

impl<S: Num + Copy + Clone + Debug + PartialOrd + PartialEq> MeshBuilder<S> {
    /// Creates a new [MeshBuilder](crate::mesh_builder::MeshBuilder) instance.
    pub fn new() -> Self {
        MeshBuilder {
            indices: None,
            positions: None,
        }
    }

    ///
    /// Set the indices of each face, where the indices of face `x` is `(i0, i1, i2) = (indices[3*x], indices[3*x+1], indices[3*x+2])`.
    ///
    /// # Examples
    /// ```
    /// # use tri_mesh::mesh_builder::{MeshBuilder, MeshBuilderError};
    /// #
    /// # fn main() -> Result<(), Box<MeshBuilderError>> {
    /// let indices: Vec<u32> = vec![0, 1, 2,  0, 2, 3,  0, 3, 1];
    /// let positions: Vec<f64> = vec![0.0, 0.0, 0.0,  1.0, 0.0, -0.5,  -1.0, 0.0, -0.5, 0.0, 0.0, 1.0];
    /// let mesh = MeshBuilder::new().with_indices(indices).with_positions(positions).build()?;
    ///
    /// assert_eq!(mesh.no_faces(), 3);
    /// assert_eq!(mesh.no_vertices(), 4);
    ///
    /// #   mesh.is_valid().unwrap();
    /// #   Ok(())
    /// # }
    /// ```
    ///
    pub fn with_indices(mut self, indices: Vec<u32>) -> Self {
        self.indices = Some(indices);
        self
    }

    ///
    /// Set the positions of each vertex, where the position of vertex `x` is `(x, y, z) = (positions[3*x], positions[3*x+1], positions[3*x+2])`;
    ///
    /// # Examples
    ///
    /// Build from positions (note: Use [merge_overlapping_primitives](crate::mesh::Mesh::merge_overlapping_primitives) if you want to merge
    /// unconnected but overlapping parts of the mesh):
    /// ```
    /// # use tri_mesh::mesh_builder::{MeshBuilder, MeshBuilderError};
    /// #
    /// # fn main() -> Result<(), Box<MeshBuilderError>> {
    /// let positions: Vec<f64> = vec![0.0, 0.0, 0.0,  1.0, 0.0, -0.5,  -1.0, 0.0, -0.5,
    ///                                    0.0, 0.0, 0.0,  -1.0, 0.0, -0.5, 0.0, 0.0, 1.0,
    ///                                    0.0, 0.0, 0.0,  0.0, 0.0, 1.0,  1.0, 0.0, -0.5];
    /// let mesh = MeshBuilder::new().with_positions(positions).build()?;
    ///
    /// assert_eq!(mesh.no_faces(), 3);
    /// assert_eq!(mesh.no_vertices(), 9);
    ///
    /// #   mesh.is_valid().unwrap();
    /// #   Ok(())
    /// # }
    /// ```
    ///
    pub fn with_positions(mut self, positions: Vec<S>) -> Self {
        self.positions = Some(positions);
        self
    }

    ///
    /// Builds the mesh. Returns the mesh if the definition is valid and otherwise an error.
    ///
    /// # Errors
    ///
    /// If no positions are specified, [NoPositionsSpecified](crate::mesh_builder::Error::NoPositionsSpecified) error is returned.
    ///
    pub fn build<P: TwoDimensionalPoint>(self) -> Result<Mesh<P>, MeshBuilderError> {
        let positions = self
            .positions
            .ok_or(MeshBuilderError::NoPositionsSpecified {
                message: format!("Did you forget to specify the vertex positions?"),
            })?;
        let indices = self
            .indices
            .unwrap_or((0..positions.len() as u32 / 2).collect());
        Ok(Mesh::new(indices, positions))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_obj() {
        let source = "o Cube
        v 1.000000 -1.000000 -1.000000
        v 1.000000 -1.000000 1.000000
        v -1.000000 -1.000000 1.000000
        v -1.000000 -1.000000 -1.000000
        v 1.000000 1.000000 -1.000000
        v 0.999999 1.000000 1.000001
        v -1.000000 1.000000 1.000000
        v -1.000000 1.000000 -1.000000
        f 1 2 3
        f 1 3 4
        f 5 8 7
        f 5 7 6
        f 1 5 6
        f 1 6 2
        f 2 6 7
        f 2 7 3
        f 3 7 8
        f 3 8 4
        f 5 1 4
        f 5 4 8"
            .to_string();

        let mesh = MeshBuilder::new().with_obj(source).build().unwrap();

        assert_eq!(mesh.no_faces(), 12);
        assert_eq!(mesh.no_vertices(), 8);

        mesh.is_valid().unwrap();
    }
}
