use crate::geometry::half_edge::{
    index::{FaceIndex, HalfEdgeIndex, IndexMap, VertexIndex},
    value::{Face, HPoint, HalfEdge},
};
use crate::geometry::Point2d;
use parking_lot::Mutex;
use std::sync::Arc;
use test::RunIgnored::No;

pub type AtomicLockIdxMap<I, V> = Arc<Mutex<IndexMap<I, V>>>;

#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq)]
pub struct HalfEdgeMesh {
    points: AtomicLockIdxMap<VertexIndex, HPoint>,
    halfedges: AtomicLockIdxMap<HalfEdgeIndex, HalfEdge>,
    faces: AtomicLockIdxMap<FaceIndex, Face>,
}

impl HalfEdgeMesh {
    pub fn new() -> Self {
        HalfEdgeMesh {
            points: Arc::new(Mutex::new(IndexMap::new())),
            halfedges: Arc::new(Mutex::new(IndexMap::new())),
            faces: Arc::new(Mutex::new(IndexMap::new())),
        }
    }

    pub fn from_raw_parts(
        verticies: AtomicLockIdxMap<VertexIndex, HPoint>,
        halfedges: AtomicLockIdxMap<HalfEdgeIndex, HalfEdge>,
        faces: AtomicLockIdxMap<FaceIndex, Face>,
    ) -> Self {
        HalfEdgeMesh {
            points: verticies,
            halfedges,
            faces,
        }
    }

    pub fn len_verticies(&self) -> usize {
        self.points.lock().len()
    }

    pub fn len_edges(&self) -> usize {
        self.len_halfedges() / 2
    }

    pub fn len_halfedges(&self) -> usize {
        self.halfedges.lock().len()
    }

    pub fn len_faces(&self) -> usize {
        self.faces.lock().len()
    }

    pub fn len(&self) -> (usize, usize, usize) {
        (self.len_verticies(), self.len_halfedges(), self.len_faces())
    }

    pub fn create_point(&self, position: Point2d) -> VertexIndex {
        self.points.lock().insert(HPoint {
            halfedge: None,
            position,
        })
    }

    pub fn create_face(
        &self,
        vertex_1: VertexIndex,
        vertex_2: VertexIndex,
        vertex_3: VertexIndex,
    ) -> FaceIndex {
    }
}
