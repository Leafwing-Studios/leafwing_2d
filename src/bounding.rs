//! Structs that bound regions that contain [`Positions`](crate::position::Position)

use crate::orientation::Direction;
use crate::position::{Coordinate, Position};
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
#[derive(Debug, Component, Clone, PartialEq, Eq)]
pub struct AxisAlignedBoundingBox<C: Coordinate> {
    /// The left extent of the bounding box
    pub low_x: C,
    /// The bottom extent of the bounding box
    pub low_y: C,
    /// The right extent of the bounding box
    pub high_x: C,
    /// The top extent of the bounding box
    pub high_y: C,
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
            low_x: C::default(),
            low_y: C::default(),
            high_x: C::default(),
            high_y: C::default(),
        };

        for position in positions.into_iter() {
            if position.x < aabb.low_x {
                aabb.low_x = position.x;
            } else if position.x > aabb.high_x {
                aabb.high_x = position.x;
            }

            if position.y < aabb.low_y {
                aabb.low_y = position.y;
            } else if position.y > aabb.high_y {
                aabb.high_y = position.y;
            }
        }

        aabb
    }

    fn contains(&self, position: Position<Self::C>) -> bool {
        (self.low_x < position.x)
            & (self.low_y < position.y)
            & (self.high_x > position.x)
            & (self.high_y > position.y)
    }

    fn intersects(&self, other: Self) -> Intersects {
        if (self.low_x > other.high_x)
            | (other.low_x > self.high_x)
            | (self.low_y > other.high_y)
            | (other.low_y > self.high_y)
        {
            Intersects::No
        } else {
            Intersects::Yes
        }
    }

    fn clamp(&self, position: Position<Self::C>) -> Position<Self::C> {
        let mut new_position = position;

        if position.x < self.low_x {
            new_position.x = self.low_x;
        } else if position.x > self.high_x {
            new_position.x = self.high_x;
        }

        if position.y < self.low_y {
            new_position.y = self.low_y;
        } else if position.y > self.high_y {
            new_position.y = self.high_y;
        }

        new_position
    }
}

impl<C: Coordinate> AxisAlignedBoundingBox<C> {
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
#[derive(Debug, Component, Clone, PartialEq)]
pub struct OrientedBoundingBox<C: Coordinate> {
    center: Position<C>,
    local_x: Direction,
    local_y: Direction,
    halfwidth_extent_x: C,
    halfwidth_extent_y: C,
}

/// A 2-dimensional convex hull with coordinate type C
#[derive(Debug, Component, Clone)]
pub struct ConvexHull<C: Coordinate> {
    hull_points: Vec<Position<C>>,
}
