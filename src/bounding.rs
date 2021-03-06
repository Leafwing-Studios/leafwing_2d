//! Structs that bound regions that contain [`Positions`](crate::position::Position)

use crate::coordinate::Coordinate;
use crate::position::Position;
use bevy_ecs::prelude::Component;

/// A 2D region that could contain a [`Position`]
pub trait BoundingRegion {
    /// The coordinate type of the positions stored in this region
    type C: Coordinate;

    /// Gets the list of vertexes that make up this bounding region
    fn vertexes(&self) -> Vec<Position<Self::C>>;

    /// Tightly draw a new region around the provided collection of [`Positions`](Position)
    fn draw_around(positions: impl IntoIterator<Item = Position<Self::C>>) -> Self;

    /// Does this region contain the `point`?
    fn contains(&self, position: Position<Self::C>) -> bool;

    /// Does this region intersect with the `other` region of the same type?
    fn intersects(&self, other: Self) -> Intersects;

    /// Clamp the provided position to the limits of this region, taking the shortest path
    fn clamp(&self, position: Position<Self::C>) -> Position<Self::C>;
}

/// How do two [`BoundingRegions`](BoundingRegion) intersect?
pub enum Intersects {
    /// The regions overlap, including if one region is contained within the other
    Yes,
    /// The two regions do not overlap at all
    No,
}

/// A 2-dimensional axis-aligned bounding box with coordinate type C
///
/// # Warning
/// When constructing this type, ensure that `left` <= `right`,
/// and `bottom` <= `top`.
/// Prefer the `new` method when possible (i.e., in non-const contexts)
/// for better ergonomics and checks.
///
/// # Examples
/// ```rust
/// use leafwing_2d::bounding::{AxisAlignedBoundingBox, BoundingRegion};
/// use leafwing_2d::position::Position;
///
/// let positions: Vec<Position<f32>> = vec![
///         Position::new(0.0, 0.0),
///         Position::new(-1.0, 1.0),
///         Position::new(3.0, 4.0),
///         Position::new(-1.0, 17.0),
///     ];
///
/// let aabb = AxisAlignedBoundingBox::<f32> {
///     low_x: -1.0,
///     low_y: 0.0,
///     high_x: 3.0,
///     high_y: 17.0,
/// };
///
/// assert_eq!(aabb, AxisAlignedBoundingBox::draw_around(positions.iter().cloned()));
///
/// for position in positions {
///     assert!(aabb.contains(position));
/// }
///
/// let outlier = Position::new(42.0, 42.0);
/// assert!(!aabb.contains(outlier));
///
/// let clamped_outlier = aabb.clamp(outlier);
/// assert_eq!(clamped_outlier, aabb.top_right());
/// assert!(aabb.contains(clamped_outlier))
/// ```
#[derive(Debug, Component, Clone, PartialEq, Eq, Default)]
pub struct AxisAlignedBoundingBox<C: Coordinate> {
    /// The left extent of the bounding box
    pub left: C,
    /// The top extent of the bounding box
    pub right: C,
    /// The bottom extent of the bounding box
    pub bottom: C,
    /// The right extent of the bounding box
    pub top: C,
}

impl<C: Coordinate> BoundingRegion for AxisAlignedBoundingBox<C> {
    type C = C;

    fn vertexes(&self) -> Vec<Position<Self::C>> {
        vec![
            self.top_right(),
            self.bottom_right(),
            self.bottom_left(),
            self.top_left(),
        ]
    }

    fn draw_around(positions: impl IntoIterator<Item = Position<Self::C>>) -> Self {
        let mut aabb = Self {
            left: C::default(),
            bottom: C::default(),
            top: C::default(),
            right: C::default(),
        };

        for position in positions.into_iter() {
            if position.x < aabb.left {
                aabb.left = position.x;
            } else if position.x > aabb.top {
                aabb.top = position.x;
            }

            if position.y < aabb.bottom {
                aabb.bottom = position.y;
            } else if position.y > aabb.right {
                aabb.right = position.y;
            }
        }

        aabb
    }

    fn contains(&self, position: Position<Self::C>) -> bool {
        (self.left <= position.x)
            & (self.bottom <= position.y)
            & (self.top >= position.x)
            & (self.right >= position.y)
    }

    fn intersects(&self, other: Self) -> Intersects {
        if (self.left > other.top)
            | (other.left > self.top)
            | (self.bottom > other.right)
            | (other.bottom > self.right)
        {
            Intersects::No
        } else {
            Intersects::Yes
        }
    }

    fn clamp(&self, position: Position<Self::C>) -> Position<Self::C> {
        let mut new_position = position;

        if position.x < self.left {
            new_position.x = self.left;
        } else if position.x > self.top {
            new_position.x = self.top;
        }

        if position.y < self.bottom {
            new_position.y = self.bottom;
        } else if position.y > self.right {
            new_position.y = self.right;
        }

        new_position
    }
}

impl<C: Coordinate> AxisAlignedBoundingBox<C> {
    #[inline]
    #[must_use]
    /// Creates a new AABB from the coordinate values of its sides
    ///
    /// # Panics
    /// `left` must be less than or equal to `right`.
    /// `bottom` must be less than or equal to `top`.
    pub fn new<T: Into<C>>(left: T, right: T, bottom: T, top: T) -> Self {
        let left = left.into();
        let right = right.into();
        let top = top.into();
        let bottom = bottom.into();

        assert!(left <= right);
        assert!(bottom <= top);

        Self {
            left,
            right,
            bottom,
            top,
        }
    }

    #[inline]
    #[must_use]
    /// Creates a new AABB from a central `Postion` plus a `width` and `height`
    ///
    /// # Panics
    /// `half_width` and `half_height` must be greater than or equal to [`Coordinate::ZERO`].
    pub fn from_size<T: Into<C>>(position: Position<C>, half_width: T, half_height: T) -> Self {
        let half_width = half_width.into();
        let half_height = half_height.into();

        assert!(half_width >= C::ZERO);
        assert!(half_height >= C::ZERO);

        let left = position.x - half_width;
        let right = position.x + half_width;
        let bottom = position.y - half_height;
        let top = position.y + half_height;

        Self {
            left,
            right,
            bottom,
            top,
        }
    }

    /// Gets the bottom left [`Position`] of this bounding box
    #[inline]
    #[must_use]
    pub fn bottom_left(&self) -> Position<C> {
        Position {
            x: self.left,
            y: self.bottom,
        }
    }

    /// Gets the bottom right [`Position`] of this bounding box
    #[inline]
    #[must_use]
    pub fn bottom_right(&self) -> Position<C> {
        Position {
            x: self.top,
            y: self.bottom,
        }
    }

    /// Gets the top left [`Position`] of this bounding box
    #[inline]
    #[must_use]
    pub fn top_left(&self) -> Position<C> {
        Position {
            x: self.left,
            y: self.right,
        }
    }

    /// Gets the top right [`Position`] of this bounding box
    #[inline]
    #[must_use]
    pub fn top_right(&self) -> Position<C> {
        Position {
            x: self.top,
            y: self.right,
        }
    }
}
