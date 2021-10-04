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
