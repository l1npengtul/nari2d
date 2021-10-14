use crate::components::{position::PositionComponent, rotation::RotationComponent};
use smallvec::SmallVec;

#[derive(Copy, Clone, PartialEq)]
pub struct Bone {
    start_position: PositionComponent,
    rotation: RotationComponent,
    length: f32,
    radius: f32,
    end_position: PositionComponent,
}

impl Default for Bone {
    fn default() -> Self {
        Bone {
            start_position: PositionComponent::default(),
            rotation: RotationComponent::default(),
            length: 1.0,
            radius: 1.0,
            end_position: PositionComponent::default(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct SkeletonComponent {
    bones: SmallVec<[Bone; 8]>,
}
