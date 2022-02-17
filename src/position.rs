//! 2-dimensional coordinates

use derive_more::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
use num_traits::NumAssign;
use std::{fmt::Debug, ops::*};

/// A 2-dimensional coordinate
///
/// The underlying data type `T` can be modified to control
/// whether the coordinate system is hexagonal vs. grid,
/// continuous or discrete and so on.
#[derive(Default, Clone, Copy, Debug, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign)]
pub struct Position<T: Copy + NumAssign> {
    /// The first coordinate of the position, typically the x-axis
    pub x: T,
    /// The first coordinate of the position, typically the y-axis
    pub y: T,
}

mod basic_operations {
    use super::*;

    impl<T: Copy + NumAssign> Add<Position<T>> for Position<T> {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl<T: Copy + NumAssign> Sub<Position<T>> for Position<T> {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }

    impl<T: Copy + NumAssign> Mul<T> for Position<T> {
        type Output = Position<T>;

        fn mul(self, rhs: T) -> Self::Output {
            Self {
                x: self.x * rhs,
                y: self.y * rhs,
            }
        }
    }

    impl<T: Copy + NumAssign> Div<T> for Position<T> {
        type Output = Position<T>;

        fn div(self, rhs: T) -> Self::Output {
            Self {
                x: self.x / rhs,
                y: self.y / rhs,
            }
        }
    }

    impl<T: Copy + NumAssign> Rem<T> for Position<T> {
        type Output = Position<T>;

        fn rem(self, rhs: T) -> Self::Output {
            Self {
                x: self.x % rhs,
                y: self.y % rhs,
            }
        }
    }

    impl<T: Copy + NumAssign> Rem<Position<T>> for Position<T> {
        type Output = Position<T>;

        fn rem(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x % rhs.x,
                y: self.y % rhs.y,
            }
        }
    }
}

// When relevant, z-values are simply ignored
mod conversions {
    use super::*;
    use bevy::math::{Vec2, Vec3};
    use bevy::transform::components::{GlobalTransform, Transform};

    impl<T: Copy + NumAssign + From<f32>> From<Vec2> for Position<T> {
        fn from(vec: Vec2) -> Self {
            Self {
                x: vec.x.into(),
                y: vec.y.into(),
            }
        }
    }

    impl<T: Copy + NumAssign + From<f32>> From<Vec3> for Position<T> {
        fn from(vec: Vec3) -> Self {
            Self {
                x: vec.x.into(),
                y: vec.y.into(),
            }
        }
    }

    impl<T: Copy + NumAssign + From<f32>> From<Transform> for Position<T> {
        fn from(transform: Transform) -> Self {
            Self {
                x: transform.translation.x.into(),
                y: transform.translation.y.into(),
            }
        }
    }

    impl<T: Copy + NumAssign + From<f32>> From<GlobalTransform> for Position<T> {
        fn from(transform: GlobalTransform) -> Self {
            Self {
                x: transform.translation.x.into(),
                y: transform.translation.y.into(),
            }
        }
    }
}
