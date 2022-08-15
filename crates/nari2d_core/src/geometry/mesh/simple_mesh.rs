use crate::geometry::point2d::Point2d;
use crate::geometry::{
    mesh::{PointId, Triangle, TriangleId},
};
use slotmap::SlotMap;

pub struct SimpleMesh {
    points: SlotMap<PointId, Point2d>,
    triangles: SlotMap<TriangleId, Triangle>,
}

impl SimpleMesh {}
