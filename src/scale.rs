//! Tools to change the size / scale of 2D objects

use crate::coordinate::Coordinate;
use bevy_ecs::prelude::Component;

/// The relative size of a 2D object
///
/// Use a [`AxisAlignedBoundingBox`] as a component for a measure of absolute size
#[derive(Component, Clone, Debug)]
pub struct Scale<C: Coordinate>(pub C::Data);

mod ops {
    use core::ops::{Div, Mul};

    use super::Scale;
    use crate::coordinate::Coordinate;

    impl<C: Coordinate> Mul<C::Data> for Scale<C> {
        type Output = Scale<C>;

        fn mul(self, rhs: C::Data) -> Self::Output {
            Scale(self.0 * rhs)
        }
    }

    impl<C: Coordinate> Div<C::Data> for Scale<C> {
        type Output = Scale<C>;

        fn div(self, rhs: C::Data) -> Self::Output {
            Scale(self.0 / rhs)
        }
    }
}
