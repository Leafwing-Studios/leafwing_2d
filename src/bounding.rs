//! Structs that bound regions that contain [`Positions`](crate::position::Position)

use crate::position::Position;
use bevy_ecs::prelude::Component;

/// A 2D region that could contain a [`Position`]
pub trait BoundingRegion {
    /// The coordinate type of the positions stored in this region
    type C;

    /// Gets the list of vertexes that make up this bounding region
    fn vertexes(&self) -> Vec<Position<Self::C>>;

    /// Tightly draw a new region around the provided collection of [`Positions`](Position)
    fn draw_around(positions: impl IntoIterator<Item = Position<Self::C>>) -> Self;

    /// Does this region contain the `point`?
    fn contains(&self, position: Position<Self::C>) -> bool;

    /// Does this region intersect with the `other_region` of the same type?
    fn intersects(&self, other_region: Self) -> Intersection;

    /// Clamp the provided position to the limits of this region, taking the shortest path
    fn clamp(&self, position: Position<Self::C>) -> Position<Self::C>;
}

/// How do two [`BoundingRegions`](BoundingRegion) intersect?
pub enum Intersection {
    /// The edges of the two regions intersect, and some but not all of the regions are shared
    Yes,
    /// The two regions do not overlap at all
    No,
    /// The first region is fully contained by the second region
    SelfInOther,
    /// The second region is fully contained by the first region
    OtherInSelf,
}

/// A 2-dimensional axis-aligned bounding box with coordinate type C
#[derive(Debug, Component, Clone, PartialEq, Eq)]
pub struct AxisAlignedBoundingBox<C: Clone> {
    /// The left extent of the bounding box
    pub low_x: C,
    /// The bottom extent of the bounding box
    pub low_y: C,
    /// The right extent of the bounding box
    pub high_x: C,
    /// The top extent of the bounding box
    pub high_y: C,
}

impl<C: Clone> AxisAlignedBoundingBox<C> {
    /// Gets the bottom left [`Position`] of this bounding box
    #[inline]
    #[must_use]
    pub fn bottom_left(&self) -> Position<C> {
        Position {
            x: self.low_x.clone(),
            y: self.low_y.clone(),
        }
    }

    /// Gets the bottom right [`Position`] of this bounding box
    #[inline]
    #[must_use]
    pub fn bottom_right(&self) -> Position<C> {
        Position {
            x: self.high_x.clone(),
            y: self.low_y.clone(),
        }
    }

    /// Gets the top left [`Position`] of this bounding box
    #[inline]
    #[must_use]
    pub fn top_left(&self) -> Position<C> {
        Position {
            x: self.low_x.clone(),
            y: self.high_y.clone(),
        }
    }

    /// Gets the top_right [`Position`] of this bounding box
    #[inline]
    #[must_use]
    pub fn top_right(&self) -> Position<C> {
        Position {
            x: self.high_x.clone(),
            y: self.high_y.clone(),
        }
    }
}

/// A 2-dimensional oriented bounding box with coordinate type C
#[derive(Debug, Component, Clone, PartialEq, Eq)]
pub struct OrientedBoundingBox<C> {
    low_x: C,
    low_y: C,
    high_x: C,
    high_y: C,
}

/// A 2-dimensional convex hull with coordinate type C
#[derive(Debug, Component, Clone)]
pub struct ConvexHull<C> {
    hull_points: Vec<Position<C>>,
}
