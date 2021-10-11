use crate::components::position::PositionComponent;

// The Vec is flattened width-wise, i.e. Vec< [row 1; width] * height times >
#[derive(Clone, Debug, PartialEq)]
pub struct LatticeComponent {
    width: u32,
    height: u32,
    // CLARITY: We're not storing components, but reusing the struct.
    position: PositionComponent,
    points: Vec<PositionComponent>,
}

impl Default for LatticeComponent {
    fn default() -> Self {
        let points = vec![
            PositionComponent::from((0_f32, 0_f32)),
            PositionComponent::from((-1_f32, 0_f32)),
            PositionComponent::from((-2_f32, 0_f32)),
            PositionComponent::from((0_f32, -1_f32)),
            PositionComponent::from((-1_f32, -1_f32)),
            PositionComponent::from((-2_f32, -1_f32)),
            PositionComponent::from((0_f32, 0_f32)),
            PositionComponent::from((-1_f32, -2_f32)),
            PositionComponent::from((-2_f32, -2_f32)),
        ];

        LatticeComponent {
            width: 3,
            height: 3,
            position: PositionComponent::default(),
            points,
        }
    }
}
