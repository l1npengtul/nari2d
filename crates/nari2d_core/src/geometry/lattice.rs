use crate::error::NResult;
use crate::geometry::Point2d;
use smallvec::SmallVec;
use std::ops::{Deref, DerefMut, Index};

#[derive(
    Copy, Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize,
)]
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
#[derive(Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct Lattice {
    points: SmallVec<[StrengthPoint; 30]>,
    width: u32,
    height: u32,
}

impl Lattice {
    pub fn new(
        width: u32,
        height: u32,
        offset: Option<f32>,
        default_strength: Option<f32>,
    ) -> Self {
        let offset = match offset {
            Some(ofs) => ofs,
            None => 20_f32,
        };

        let default_strength = match default_strength {
            Some(def) => def,
            None => 1_f32,
        };

        let mut points: SmallVec<[StrengthPoint; 30]> =
            SmallVec::with_capacity((width * height) as usize);
        for x in 0..width {
            for y in 0..height {
                points.push(StrengthPoint::new(
                    Point2d::new((x as f32) * offset, (y as f32) * offset),
                    default_strength,
                ))
            }
        }

        Lattice {
            points,
            width,
            height,
        }
    }
}

impl Deref for Lattice {
    type Target = SmallVec<[StrengthPoint; 30]>;

    fn deref(&self) -> &Self::Target {
        &self.points
    }
}

impl DerefMut for Lattice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.points
    }
}

pub struct LatticeRowView<'a> {
    data: &'a [StrengthPoint],
    index: u32,
}

pub struct LatticeRowViewMut<'a> {
    data: &'a mut [StrengthPoint],
    index: u32,
}

pub struct LatticeColumnView<'a> {
    data: &'a [StrengthPoint],
    index: u32,
}

pub struct LatticeColumnViewMut<'a> {
    data: &'a mut [StrengthPoint],
    index: u32,
}
