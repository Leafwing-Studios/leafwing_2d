# Release Notes

## Version 0.1

### Enhancements

- Added `Direction` (a normalized `Vec2`) and `Rotation` (an angle from midnight) to make it easier to work with rotations in 2D
  - See the `DirectionPartitioning` trait for methods and types for converting analog inputs into discrete outcomes
- Added `Position<T>`, a 2-dimensional coordinate type
  - See `DiscreteCoordinate` trait for additional specialized methods and premade types for square and hex grids
- Added `TwoDimBundle` and `TwoDimPlugin` for working with 2D geometry within Bevy and synchronizing it with `Transform`
- Added `AxisAlignedBoundingBox` and `OrientedBoundingBox` for simple collision checking and clamping in 2D
