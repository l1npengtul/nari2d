use euclid::{Angle, Rotation2D, UnknownUnit};
use std::ops::{Deref, DerefMut};

#[derive(Copy, Clone, PartialEq)]
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

impl From<Rotation2D<f32, UnknownUnit, UnknownUnit>> for RotationComponent {
    fn from(rotation: Rotation2D<f32, UnknownUnit, UnknownUnit>) -> Self {
        RotationComponent::new(rotation.angle.to_radians())
    }
}
