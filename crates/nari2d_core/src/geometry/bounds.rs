use crate::geometry::{Point2d, Scale2d};

//  ______________A
// |        _B__/ |
// |      C       |
// |_____________|
//
// C = center
// B = extend

#[derive(Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct Bounds {
    center: Point2d,
    extend: Scale2d,
}

impl Bounds {
    #[inline]
    pub fn new(center: Point2d, top_right: Point2d) -> Self {
        let extend: Scale2d = (center - top_right).into();
        Bounds { center, extend }
    }

    #[inline]
    pub fn with_scale(center: Point2d, scale: Scale2d) -> Self {
        Bounds {
            center,
            extend: scale,
        }
    }

    #[inline]
    pub fn from_points(min: Point2d, max: Point2d) -> Self {
        let center = (min + max) / 2_f64;
        let extend: Scale2d = (max - center).into();

        Bounds { center, extend }
    }

    #[inline]
    pub fn center(&self) -> Point2d {
        self.center
    }

    #[inline]
    pub fn center_x(&self) -> f64 {
        self.center.x()
    }

    #[inline]
    pub fn center_y(&self) -> f64 {
        self.center.y()
    }
    #[inline]
    pub fn extends(&self) -> Scale2d {
        self.extend
    }

    #[inline]
    pub fn extends_x(&self) -> f64 {
        self.extend.x()
    }

    #[inline]
    pub fn extends_y(&self) -> f64 {
        self.extend.y()
    }

    #[inline]
    pub fn size(&self) -> Scale2d {
        self.extend * 2_f64
    }

    #[inline]
    pub fn size_x(&self) -> f64 {
        self.extends_x() * 2_f64
    }

    #[inline]
    pub fn size_y(&self) -> f64 {
        self.extends_y() * 2_f64
    }

    #[inline]
    pub fn set_center(&mut self, new_center: Point2d) {
        self.center = new_center;
    }

    #[inline]
    pub fn set_center_x(&mut self, new_x: f64) {
        self.center.set_x(new_x)
    }

    #[inline]
    pub fn set_center_y(&mut self, new_y: f64) {
        self.center.set_y(new_y)
    }

    #[inline]
    pub fn set_extend(&mut self, new_scale: Scale2d) {
        self.extend = new_scale;
    }

    #[inline]
    pub fn set_extend_x(&mut self, new_x: f64) {
        self.extend.set_x(new_x)
    }

    #[inline]
    pub fn set_extend_y(&mut self, new_y: f64) {
        self.extend.set_y(new_y)
    }
}
