use crate::geometry::orientation::Orientation;
use cgmath::{MetricSpace, Point2};
use robust::{orient2d, Coord};
use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
};

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
pub struct Point2d {
    int: Point2<f32>,
}

impl Point2d {
    pub const ZERO: Point2d = Point2d {
        int: Point2 { x: 0_f32, y: 0_f32 },
    };
    pub const UNIT: Point2d = Point2d {
        int: Point2 { x: 1_f32, y: 1_f32 },
    };

    pub fn new(x: f32, y: f32) -> Self {
        Point2d {
            int: Point2 { x, y },
        }
    }

    // MAJOR FIXME: USE ROBUST!!!!

    // taken fromhttps://github.com/mourner/delaunator-rs/blob/master/src/lib.rs:73
    // i have no fucking idea what this does
    pub fn circumdelta(p1: &Point2d, p2: &Point2d, p3: &Point2d) -> Self {
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        let ex = p3.x - p1.x;
        let ey = p3.y - p1.y;

        let bl = dx.powi(2) + dy.powi(2);
        let cl = ex.powi(2) + ey.powi(2);
        let d = 0.5 / (dx * ey - dy * ex);

        let x = (ey * bl - dy * cl) * d;
        let y = (dx * cl - ex * bl) * d;
        Self {
            int: Point2 { x, y },
        }
    }

    pub fn circumcenter(p1: &Point2d, p2: &Point2d, p3: &Point2d) -> Self {
        let mut pt = Point2d::circumdelta(p1, p2, p3);
        pt.x += p1.x;
        pt.y += p1.y;
        pt.into()
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn line_intersect(a: (&Point2d, &Point2d), b: (&Point2d, &Point2d)) -> bool {
        let o1 = Point2d::orientation(a.0, a.1, b.0);
        let o2 = Point2d::orientation(a.0, a.1, b.1);
        let o3 = Point2d::orientation(b.0, b.1, a.0);
        let o4 = Point2d::orientation(b.0, b.1, a.1);

        if o1 != o2 && o3 != o4 {
            true
        } else if o1.is_colinear() && b.0.is_on_segment(a) {
            true
        } else if o2.is_colinear() && b.1.is_on_segment(a) {
            true
        } else if o3.is_colinear() && a.0.is_on_segment(b) {
            true
        } else if o4.is_colinear() && a.1.is_on_segment(b) {
            true
        } else {
            false
        }
    }

    pub fn orientation(a: &Point2d, b: &Point2d, c: &Point2d) -> Orientation {
        orient2d(a.into(), b.into(), c.into()).into()
    }

    pub fn is_on_segment(&self, segment: (&Point2d, &Point2d)) -> bool {
        if self.x <= f32::max(segment.0.x, segment.1.x)
            && self.x >= f32::min(segment.0.x, segment.1.x)
            && self.y <= f32::max(segment.0.y, segment.1.y)
            && self.y >= f32::min(segment.0.y, segment.1.y)
        {
            true
        } else {
            false
        }
    }

    pub fn is_inside(&self, polygon: &[Point2d]) -> bool {
        if polygon.len() < 3 {
            return false;
        }

        let extreme = Point2d::new(f32::INFINITY, self.y);
        let mut decrease = 0;

        let mut count = 0;
        let mut i = 0;
        loop {
            let next = (i + 1) % polygon.len();
            if polygon.get(i).map(Point2d::y) == Some(self.y) {
                decrease += 1;
            }

            let a = some_or_continue!(polygon.get(i));
            let b = some_or_continue!(polygon.get(next));

            if Point2d::line_intersect((a, b), (&self, &extreme)) {
                if Point2d::orientation(a, &self, b) == Orientation::Colinear {
                    return self.is_on_segment((a, b));
                }
                count += 1;
            }

            i = next;

            if i == 0 {
                break;
            }
        }

        count -= decrease;

        return count % 2 == 1;
    }

    pub fn distance(&self, other: &Point2d) -> f32 {
        self.int.distance(other.int)
    }

    pub fn distance2(&self, other: &Point2d) -> f32 {
        self.int.distance2(other.int)
    }

    pub fn circumradius2(p1: &Point2d, p2: &Point2d, p3: &Point2d) -> f32 {
        let circumdelta = Point2d::circumdelta(p1, p2, p3);
        circumdelta.x().powi(2) + circumdelta.y().powi(2)
    }
}

impl Deref for Point2d {
    type Target = Point2<f32>;

    fn deref(&self) -> &Self::Target {
        &self.int
    }
}

impl DerefMut for Point2d {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.int
    }
}

impl Display for Point2d {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Default for Point2d {
    fn default() -> Self {
        Self::ZERO
    }
}

impl PartialEq for Point2d {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point2d {}

impl PartialOrd for Point2d {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point2d {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.x.total_cmp(&other.x) {
            Ordering::Equal => self.y.total_cmp(&other.y),
            ord => ord,
        }
    }
}

impl Hash for Point2d {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
    }
}

impl From<Point2<f32>> for Point2d {
    fn from(p2: Point2<f32>) -> Self {
        Point2d { int: p2 }
    }
}

impl From<Point2d> for Point2<f32> {
    fn from(p2: Point2d) -> Self {
        p2.int
    }
}

impl From<Point2d> for Coord<f32> {
    fn from(p2d: Point2d) -> Self {
        Coord {
            x: p2d.x(),
            y: p2d.y(),
        }
    }
}

impl From<&Point2d> for Coord<f32> {
    fn from(p2d: &Point2d) -> Self {
        Coord {
            x: p2d.x(),
            y: p2d.y(),
        }
    }
}

#[cfg(feature = "edit")]
impl rstar::Point for Point2d {
    type Scalar = f32;

    const DIMENSIONS: usize = 2;

    fn generate(generator: impl FnMut(usize) -> Self::Scalar) -> Self {
        Self::new(generator(0), generator(1))
    }

    fn nth(&self, index: usize) -> Self::Scalar {
        match index {
            0 => self.x,
            1 => self.y,
            _ => unreachable!(),
        }
    }

    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => unreachable!(),
        }
    }
}
