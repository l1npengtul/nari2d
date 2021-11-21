use crate::geometry::Point2d;
use core::cell::Cell;

pub struct HalfMesh {}

#[derive(Copy)]
pub struct HalfEdge {
    vertex: *mut Cell<HalfVertex>,
    face: *mut Cell<HalfFace>,
    next: *mut Cell<HalfEdge>,
    previous: *mut Cell<HalfEdge>,
    pair: *mut Cell<HalfEdge>,
}

#[derive(Copy)]
pub struct HalfVertex {
    point: Point2d,
    edge: *mut Cell<HalfEdge>,
}

#[derive(Copy)]
pub struct HalfFace {
    edge: *mut Cell<HalfEdge>,
}
