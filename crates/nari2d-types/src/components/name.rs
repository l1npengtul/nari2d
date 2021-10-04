use std::{
    borrow::{Cow, Borrow},
    ops::{Deref, DerefMut},
    fmt::{Display, Formatter}
};

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
