pub struct TriangleMesh {
    triangles: Vec<usize>,
    halfedges: Vec<usize>,
    hull: Vec<usize>,
    constraints: Vec<usize>,
}

impl TriangleMesh {
    pub fn new(n_tris: usize) -> Self {
        let max_tris = if n_tris > 2 { 2 * n - 5 } else { 0 };

        Self {
            triangles: Vec::with_capacity(max_tris * 3),
            halfedges: Vec::with_capacity(max_tris * 3),
            hull: vec![],
            constraints: vec![],
        }
    }
}
