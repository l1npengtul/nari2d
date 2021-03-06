use euclid::Vector2D;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ScaleComponent {
    scale: Vector2D<f32, f32>,
}

impl ScaleComponent {
    #[must_use]
    pub fn new(sx: f32, sy: f32) -> Self {
        ScaleComponent {
            scale: Vector2D::new(sx, sy),
        }
    }

    #[must_use]
    pub fn from_tuple(scale: (f32, f32)) -> Self {
        ScaleComponent {
            scale: Vector2D::new(scale.0, scale.1),
        }
    }

    #[must_use]
    pub fn from_array(scale: [f32; 2]) -> Self {
        ScaleComponent::from_tuple((scale[0], scale[1]))
    }

    #[must_use]
    pub fn scale_x(&self) -> f32 {
        self.scale.x
    }

    #[must_use]
    pub fn scale_y(&self) -> f32 {
        self.scale.y
    }
}

impl Default for ScaleComponent {
    fn default() -> Self {
        ScaleComponent::new(1.0, 1.0)
    }
}
