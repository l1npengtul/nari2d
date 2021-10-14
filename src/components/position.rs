use euclid::Point2D;
use std::ops::{Deref, DerefMut};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PositionComponent {
    position: Point2D<f32, f32>,
}

impl PositionComponent {
    #[must_use]
    pub fn new(x: f32, y: f32) -> Self {
        PositionComponent {
            position: Point2D::new(x, y),
        }
    }

    #[must_use]
    pub fn from_tuple(pos: (f32, f32)) -> Self {
        PositionComponent {
            position: Point2D::new(pos.0, pos.1),
        }
    }

    #[must_use]
    pub fn from_array(pos: [f32; 2]) -> Self {
        PositionComponent::from_tuple((pos[0], pos[1]))
    }

    #[must_use]
    pub fn position_x(&self) -> f32 {
        self.position.x
    }

    #[must_use]
    pub fn position_y(&self) -> f32 {
        self.position.y
    }

    #[must_use]
    pub fn to_tuple(&self) -> (f32, f32) {
        (self.position_x(), self.position_y())
    }

    #[must_use]
    pub fn to_array(&self) -> [f32; 2] {
        [self.position_x(), self.position_y()]
    }
}

impl Deref for PositionComponent {
    type Target = Point2D<f32, f32>;

    fn deref(&self) -> &Self::Target {
        &self.position
    }
}

impl DerefMut for PositionComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.position
    }
}

impl Default for PositionComponent {
    fn default() -> Self {
        PositionComponent::new(0_f32, 0_f32)
    }
}

impl From<(f32, f32)> for PositionComponent {
    fn from(pos: (f32, f32)) -> Self {
        PositionComponent::from_tuple(pos)
    }
}

impl From<[f32; 2]> for PositionComponent {
    fn from(pos: [f32; 2]) -> Self {
        PositionComponent::from_array(pos)
    }
}

impl From<Point2D<f32, f32>> for PositionComponent {
    fn from(point: Point2D<f32, f32>) -> Self {
        PositionComponent::from(point.to_array())
    }
}
