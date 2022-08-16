#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
#[cfg_attr(feature = "serde_impl", derive(serde::Serialize, serde::Deserialize))]
pub enum Orientation {
    CounterClockWise,
    ClockWise,
    Colinear,
}

impl Orientation {
    pub fn is_colinear(&self) -> bool {
        self == &Orientation::Colinear
    }
    pub fn is_counter_clock_wise(&self) -> bool {
        self == &Orientation::CounterClockWise
    }
    pub fn is_clockwise(&self) -> bool {
        self == &Orientation::ClockWise
    }
}

impl From<f64> for Orientation {
    fn from(v: f64) -> Self {
        if v.is_sign_negative() {
            Orientation::ClockWise
        } else if v.is_sign_positive() {
            Orientation::CounterClockWise
        } else {
            Orientation::Colinear
        }
    }
}
