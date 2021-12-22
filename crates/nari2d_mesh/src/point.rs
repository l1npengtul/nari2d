use core::{fmt::Debug, hash::Hash};
use num_traits::Num;

pub trait TwoDimensionalPoint: Copy + Clone + Debug + Hash + PartialEq + PartialOrd {
    type SCALAR: Scalar;

    fn new_point(x: Self::SCALAR, y: Self::SCALAR) -> Self;

    fn x(&self) -> Self::SCALAR;
    fn set_x(&mut self, new_x: Self::SCALAR);

    fn y(&self) -> Self::SCALAR;
    fn set_y(&mut self, new_y: Self::SCALAR);

    fn to_tuple(&self) -> (Self::SCALAR, Self::SCALAR) {
        (self.x(), self.y())
    }

    fn to_array(&self) -> [Self::SCALAR; 2] {
        [self.x(), self.y()]
    }
}

pub trait Scalar: Num + Copy + Clone + Debug + PartialOrd + PartialEq {
    fn min() -> Self;
    fn max() -> Self;
}
