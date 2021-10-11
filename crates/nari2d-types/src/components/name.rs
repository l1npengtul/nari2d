use std::{
    borrow::{Borrow, Cow},
    fmt::{Display, Formatter},
    ops::{Deref, DerefMut},
};

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct NameComponent {
    name: String,
}

impl NameComponent {
    pub fn new<S: ToString>(name: S) -> Self {
        let name = name.to_string();
        NameComponent { name }
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
}

impl Display for NameComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.name)
    }
}
