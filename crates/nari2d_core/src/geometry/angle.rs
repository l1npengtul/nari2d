use crate::geometry::point2d::Point2d;
use cgmath::{Angle as CGAngle, Rad};
use float_eq::float_eq;
use std::{
    cmp::Ordering,
    f32::consts::PI,
    fmt::{Display, Formatter},
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
};

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
pub struct Angle {
    int: Rad<f32>,
}

impl Angle {
    pub const ZERO: Angle = Angle { int: Rad(0_f32) };
    pub const PI: Angle = Angle { int: Rad(PI) };

    pub fn new(radians: f32) -> Self {
        Angle { int: Rad(radians) }
    }

    pub fn from_3_points(p1: &Point2d, p2: &Point2d, p3: &Point2d) -> Self {
        (Rad::atan2(p3.y - p1.y, p3.x - p1.x) - Rad::atan2(p2.y - p1.y, p2.x - p1.x)).into()
    }

    pub fn from_2_points(p1: &Point2d, p2: &Point2d) -> Self {
        Self {
            int: Rad::atan2(p2.y - p1.y, p2.x - p1.x),
        }
    }
}

impl Deref for Angle {
    type Target = Rad<f32>;

    fn deref(&self) -> &Self::Target {
        &self.int
    }
}

impl DerefMut for Angle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.int
    }
}

impl Display for Angle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_Rad", self.int.0)
    }
}

impl Default for Angle {
    fn default() -> Self {
        Self::ZERO
    }
}

impl PartialEq for Angle {
    fn eq(&self, other: &Self) -> bool {
        float_eq!(self.0, other.0, r2nd <= 2.0 * f32::EPSILON)
    }
}

impl Eq for Angle {}

impl PartialOrd for Angle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Angle {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl Hash for Angle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl From<Rad<f32>> for Angle {
    fn from(rad: Rad<f32>) -> Self {
        Angle { int: rad }
    }
}
impl From<Angle> for Rad<f32> {
    fn from(a: Angle) -> Self {
        a.int
    }
}
