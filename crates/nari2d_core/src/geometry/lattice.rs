use crate::collections::point_store::PointStore;
use crate::geometry::mesh::PointRef;
use crate::geometry::Point2d;
use simple_grid::Grid;
use std::ops::{Deref, DerefMut};

#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct StrengthPoint {
    point: Point2d,
    strength: f32,
}

impl StrengthPoint {
    pub fn new<P: Into<Point2d>>(point: P, strength: f32) -> Self {
        Self {
            point: point.into(),
            strength,
        }
    }

    pub fn point(&self) -> Point2d {
        self.point
    }

    pub fn strength(&self) -> f32 {
        self.strength
    }

    pub fn set_strength(&mut self, new_strength: f32) {
        self.strength = new_strength;
    }
}

impl Deref for StrengthPoint {
    type Target = Point2d;

    fn deref(&self) -> &Self::Target {
        &self.point
    }
}

impl DerefMut for StrengthPoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.point
    }
}

impl AsRef<Point2d> for StrengthPoint {
    fn as_ref(&self) -> &Point2d {
        &self.point
    }
}

impl AsMut<Point2d> for StrengthPoint {
    fn as_mut(&mut self) -> &mut Point2d {
        &mut self.point
    }
}

impl From<Point2d> for StrengthPoint {
    fn from(pt: Point2d) -> Self {
        Self {
            point: pt,
            strength: 0_f32,
        }
    }
}

// Smallvec based flat storage of a lattice, column-row.
// [ C1R1. C1R2, C1R3, ... C4R1, C4R2, ... ]
// Start position is the first element
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Lattice {
    points: PointStore<PointRef, StrengthPoint>,
    grid: Grid<PointRef>,
}

impl Lattice {
    pub fn new() -> Self {
        let points = PointStore::from(vec![
            StrengthPoint::new(Point2d::new(-20.0, 20.0), 1.0),
            StrengthPoint::new(Point2d::new(20.0, 20.0), 1.0),
            StrengthPoint::new(Point2d::new(20.0, -20.0), 1.0),
            StrengthPoint::new(Point2d::new(-20.0, -20.0), 1.0),
        ]);

        let grid = Grid::new(2, 2, points.indices().map(|x| *x).collect());

        Lattice { points, grid }
    }
}
