//! Traits to definite units that define distances

use crate::position::Position;
pub use leafwing_2d_macros::TrivialCoordinate;
use std::{fmt::Debug, ops::*};

/// A type that can be used as a coordinate type for [`Position`]
///
/// Typically, you will want to use one of a few strategies for your [`Coordinate`] type:
/// - if you would like continuous coordinates for smooth movement, use one of the types provided in [`continuous`](crate::continuous)
/// - if you're working with a grid-like position system, use one of the types provided in [`discrete`](crate::discrete)
///   - the [`DiscreteCoordinate`](crate::discrete::DiscreteCoordinate) trait provides other useful functionality for your game!
/// - if you have unusual needs (such as extremely large worlds or tight memory constraints),
/// implement [`Coordinate`] and optionally [`DiscreteCoordinate`](discrete_coordinates::DiscreteCoordinate) for your own type
///    - if you are simply wrapping a single value, strongly consider deriving [`TrivialCoordinate`] to reduce the required boilerplate
///    - in this type, you can wrap your own [`f64`], [`u8`], fixed-point number type or so on.
///
/// # Conversion
///
/// The conversion between [`Coordinate`] types and [`f32`]
/// (as might be used in [`Transform`](bevy_transform::components::Transform)) is commonly scaled.
///
/// For discrete coordinates, this ensures that all distinct values are representable,
/// even as the precision of the [`f32`] decreases.
/// To account for this, write custom [`From<f32>`] and [`Into<f32>`] impls for your coordinate type.
/// Remember to multiply by [`Coordinate::COORD_TO_TRANSFORM`] when converting into [`f32`],
/// and divide by it when converting back!
pub trait Coordinate:
    Copy
    + Debug
    + Default
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Self, Output = Self>
    + MulAssign
    + Div<Self, Output = Self>
    + DivAssign
    + Rem<Output = Self>
    + RemAssign
    + PartialOrd
    + Send
    + Sync
    + From<f32>
    + Into<f32>
    + From<Self::Data>
    + Into<Self::Data>
    + Mul<Self::Data, Output = Self>
    + Div<Self::Data, Output = Self>
    + 'static
{
    /// The underlying numeric storage type (e.g. `f32`, `i8` or so on)
    type Data: Copy
        + Add<Output = <Self as Coordinate>::Data>
        + Sub<Output = <Self as Coordinate>::Data>
        + Mul<Output = <Self as Coordinate>::Data>
        + Div<Output = <Self as Coordinate>::Data>
        + Send
        + Sync
        + Debug
        + Default
        + 'static;

    /// The ratio between 1 unit in this coordinate system to 1 unit of [`Transform.translation`](bevy_transform::components::Transform)
    const COORD_TO_TRANSFORM: f32;

    /// The minimum representable value
    const MIN: Self;

    /// The maximum representable value
    const MAX: Self;

    /// Adding or subtracting 0 does not change the value
    ///
    /// This should be equal to the value returned by [`Default::default()`]
    const ZERO: Self;

    /// Adding or subtracting 0 does not change the value
    ///
    /// This should be equal to the value returned by [`Default::default()`]
    const DATA_ZERO: Self::Data;

    /// Multiplying or dividing data by this 1 does not change the value
    const DATA_ONE: Self::Data;

    /// The (0, 0) cell [`Position`]
    const ORIGIN: Position<Self> = Position {
        x: Self::ZERO,
        y: Self::ZERO,
    };

    /// Checks that [`Coordinate::MIN`] and [`Coordinate::MAX`] can be converted to and from `f32` in a reasonable fashion
    ///
    /// This should not panic, and the min value should be less than the max in both forms.
    fn assert_bounds_okay(&self) {
        assert!(Self::MIN < Self::MAX);

        let float_min: f32 = Self::MIN.into();
        let float_max: f32 = Self::MAX.into();

        assert!(float_min < float_max);
    }

    /// Computes the signed round-trip error of converting from an [`f32`] to the [`Coordinate`] type and back again
    ///
    /// The difference is computed as `final - initial`.
    #[must_use]
    fn round_trip_float_error(float: f32) -> f32 {
        let coordinate: Self = float.into();
        let round_trip_float: f32 = coordinate.into();

        round_trip_float - float
    }

    /// Computes the signed round-trip error of converting from the [`Coordinate`] type to an [`f32] and back again
    ///
    /// The difference is computed as `final - initial`.
    /// For [`DiscreteCoordinates`](crate::discrete::DiscreteCoordinate), this should be 0 for all values across the range.
    #[must_use]
    fn round_trip_coordinate_error(self) -> Self {
        let float: f32 = self.into();
        let round_trip_coordinate: Self = float.into();

        round_trip_coordinate - self
    }
}

/// A helper trait for [`Coordinate`] types that simply wrap a single number-like value
///
/// Use `#derive(TrivialCoordinate)]` to implement all of the trivial trait bounds on [`Coordinate`] for you.
/// All types that impl [`TrivialCoordinate`] + [`Into<f32>`] + [`TryFrom<f32>`]
/// with a [`FloatCoordinateConversionError`] error type are automatically [`Coordinate`] types.
///
/// # Example
/// ```rust
/// use leafwing_2d::position::{Coordinate, Trivial Coordinate};
///
/// // This cause `TinyCoordinate` to automatically have the right trait impls
/// // (e.g. Copy, Add, Debug and so on) due to blanket impls on `TrivialCoordinate`.
/// #[derive(TrivialCoordinate`)]
/// struct TinyCoordinate(u8);
///
/// // We need a way to convert to and from Transform.translation
/// // from our coordinate system.
/// // In this example, we're choosing to scale our grid over a large fraction of the usable space of f32.
/// const MAX_TRANSLATION: f32 = 1E+9;
/// const SCALE_FACTOR: f32 = MAX_TRANSLATION / (u8::MAX as f32);
///
/// // This automatically gets us the `Into<f32>` impl we need
/// impl From<TinyCoordinate> for f32 {
///     fn from(coordinate: TinyCoordinate) -> f32 {
///         (coordinate as f32) * SCALE_FACTOR
///     }
/// }
///
/// // Be careful to ensure that these methods are the inverse of each other!
/// impl TryFrom<f32> for TinyCoordinate {
///     type Error = FloatCoordinateConversionError;
///
///     fn from(float: f32) -> Result<TinyCoordinate, FloatCoordinateConversionError> {
///         if float <= MAX_TRANSLATION {
///            let integer = ((coordinate as f32) / SCALE_FACTOR).round() as u8;
///            Ok(TinyCoordinate(integer))
///         } else {
///             Err(FloatCoordinateConversionError(float))
///         }
///     }
/// }
///
/// // Congratulations: it's a `Coordinate`!
/// let position: Position<TrivialCoordinate> = Position::new(0,0);
/// ```
pub trait TrivialCoordinate: Sized {
    /// The underlying number-like type that is wrapped
    ///
    /// This should match [`Coordinate::Data`].
    type Wrapped: Clone + Copy + Debug + Add + Sub + Mul + Div + Rem + Default;

    /// Creates a new coodinate with the underlying `value`;
    #[must_use]
    fn new(value: Self::Wrapped) -> Self;

    /// The underlying value stored
    #[must_use]
    fn value(&self) -> Self::Wrapped;

    /// Set this coordinate to `value`
    #[must_use]
    #[inline]
    fn set(&mut self, value: Self::Wrapped) {
        *self = Self::new(value);
    }
}
