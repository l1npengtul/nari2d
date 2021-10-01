use euclid::{Angle, Point2D, Rotation2D, UnknownUnit, Vector2D};
use petgraph::Graph;
use smallvec::SmallVec;
use std::{
    borrow::{Borrow, BorrowMut, Cow},
    fmt::{Display, Formatter},
    ops::{Deref, DerefMut},
};

pub mod lattice;
pub mod light;
pub mod mesh;
pub mod particle_emitter;
pub mod physics;
pub mod skeleton;
pub mod sprite;

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct VisibilityComponent {
    visibility: bool,
}

impl VisibilityComponent {
    pub fn new() -> Self {
        VisibilityComponent::default()
    }

    pub fn is_visible(&self) -> bool {
        self.visibility
    }

    pub fn set_visibility(&mut self, new_vis: bool) {
        self.visibility = new_vis;
    }

    pub fn toggle_visibility(&mut self) {
        self.visibility = !self.visibility;
    }
}

impl Default for VisibilityComponent {
    fn default() -> Self {
        VisibilityComponent { visibility: true }
    }
}

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct NameComponent {
    name: Cow<'static, str>,
}

impl NameComponent {
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        let name = Cow::from(name.as_ref());
        NameComponent { name }
    }

    pub fn name(&self) -> &str {
        self.name.borrow()
    }

    pub fn set_name<S: AsRef<str>>(&mut self, new_name: S) {
        self.name = Cow::from(new_name.as_ref());
    }
}

impl Deref for NameComponent {
    type Target = &'static str;

    fn deref(&self) -> &Self::Target {
        self.name.borrow()
    }
}

impl DerefMut for NameComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.name.borrow()
    }
}

impl Display for NameComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.name)
    }
}

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct PositionComponent {
    position: Point2D<f32, f32>,
}

impl PositionComponent {
    pub fn new(x: f32, y: f32) -> Self {
        PositionComponent {
            position: Point2D::new(x, y),
        }
    }

    pub fn from_tuple(pos: (f32, f32)) -> Self {
        PositionComponent {
            position: Point2D::new(pos.0, pos.1),
        }
    }

    pub fn from_array(pos: [f32; 2]) -> Self {
        PositionComponent::from_tuple((pos[0], pos[1]))
    }

    pub fn position_x(&self) -> f32 {
        self.position.x
    }

    pub fn position_y(&self) -> f32 {
        self.position.y
    }

    pub fn to_tuple(&self) -> (f32, f32) {
        (self.position_x(), self.position_y())
    }

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

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct ScaleComponent {
    scale: Vector2D<f32, f32>,
}

impl ScaleComponent {
    pub fn new(sx: f32, sy: f32) -> Self {
        ScaleComponent {
            scale: Vector2D::new(sx, sy),
        }
    }

    pub fn from_tuple(scale: (f32, f32)) -> Self {
        ScaleComponent {
            scale: Vector2D::new(scale.0, scale.1),
        }
    }

    pub fn from_array(scale: [f32; 2]) -> Self {
        ScaleComponent::from_tuple((scale[0], scale[1]))
    }

    pub fn scale_x(&self) -> f32 {
        self.sx
    }

    pub fn scale_y(&self) -> f32 {
        self.sy
    }
}

impl Default for ScaleComponent {
    fn default() -> Self {
        ScaleComponent::new(1.0, 1.0)
    }
}

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct RotationComponent {
    rotation: Rotation2D<f32, UnknownUnit, UnknownUnit>,
}

impl RotationComponent {
    pub fn new(radians: f32) -> Self {
        RotationComponent {
            rotation: Rotation2D::radians(radians),
        }
    }

    pub fn from_degrees(degrees: f32) -> Self {
        let angle = Angle::degrees(degrees);
        RotationComponent {
            rotation: Rotation2D::new(angle),
        }
    }

    pub fn radians(&self) -> f32 {
        self.rotation.angle.to_radians()
    }

    pub fn as_degrees(&self) -> f32 {
        self.rotation.angle.to_degrees()
    }
}

impl Deref for RotationComponent {
    type Target = Rotation2D<f32, UnknownUnit, UnknownUnit>;

    fn deref(&self) -> &Self::Target {
        &self.rotation
    }
}

impl DerefMut for RotationComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rotation
    }
}

impl Default for RotationComponent {
    fn default() -> Self {
        RotationComponent {
            rotation: Rotation2D::identity(),
        }
    }
}
