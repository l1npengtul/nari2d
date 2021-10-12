use std::{
    borrow::{Borrow, Cow},
    fmt::{Display, Formatter},
    ops::{Deref, DerefMut},
};

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct NodeInfoComponent {
    name: String,
    z_index: u32,
    visibility: bool,
}

impl NodeInfoComponent {
    pub fn new<S: ToString>(name: S, child_index: u32) -> Self {
        let name = name.to_string();
        NodeInfoComponent {
            name,
            z_index: child_index,
            visibility: true,
        }
    }

    pub fn name(&self) -> &str {
        self.name.borrow()
    }

    pub fn name_owned(&self) -> String {
        self.name.clone()
    }

    pub fn set_name<S: ToString>(&mut self, new_name: S) {
        self.name = new_name.to_string()
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

impl Display for NodeInfoComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.name)
    }
}
