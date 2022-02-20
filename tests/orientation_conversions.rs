use bevy::math::{Quat, Vec3};
use bevy::transform::components::Transform;
use leafwing_2d::orientation::*;
use leafwing_2d::position::Position;

#[test]
fn rotation_wrapping() {
    let rotation = Rotation::new(42);

    assert_eq!(Rotation::new(Rotation::FULL_CIRCLE), Rotation::default());
    assert_eq!(rotation + Rotation::new(Rotation::FULL_CIRCLE), rotation);
    assert_eq!(rotation - Rotation::new(Rotation::FULL_CIRCLE), rotation);
    assert_eq!(
        rotation + 9001.0 * Rotation::new(Rotation::FULL_CIRCLE),
        rotation
    );
}

#[test]
fn rotation_from_degrees() {
    assert_eq!(Rotation::from_degrees(0.0).deci_degrees(), 0);
    assert_eq!(Rotation::from_degrees(65.0).deci_degrees(), 650);
    assert_eq!(Rotation::from_degrees(-90.0).deci_degrees(), 2700);
    assert_eq!(Rotation::from_degrees(360.0).deci_degrees(), 0);
}

#[test]
fn rotation_from_radians() {
    use core::f32::consts::TAU;

    assert_eq!(Rotation::from_radians(0.0).deci_degrees(), 0);
    assert_eq!(Rotation::from_radians(TAU / 6.0).deci_degrees(), 600);
    // Floating point math is not exact :(
    assert_eq!(Rotation::from_radians(-TAU / 4.0).deci_degrees(), 2699);
    assert_eq!(Rotation::from_radians(TAU).deci_degrees(), 0);
}

#[test]
fn direction_rotation_conversion() {
    assert_eq!(
        Direction::from(Rotation::from_degrees(0.0)),
        Direction::NORTH
    );
    assert_eq!(
        Direction::from(Rotation::from_degrees(45.0)),
        Direction::NORTHEAST
    );
    assert_eq!(
        Direction::from(Rotation::from_degrees(-90.0)),
        Direction::WEST
    );
    assert_eq!(
        Direction::from(Rotation::from_degrees(360.0)),
        Direction::NORTH
    );

    let neutral_result: Result<Rotation, NearlySingularConversion> = Direction::NEUTRAL.try_into();
    assert!(neutral_result.is_err());
}

fn assert_quaternion_conversion_correct(target_position: Position<f32>) {
    let target_vec3 = target_position.into();
    let origin = Position::<f32>::default();
    let mut origin_transform = Transform::default();

    origin_transform.look_at(target_vec3, Vec3::Z);

    let direction = origin.direction_to(target_position);
    let rotation = origin.rotation_to(target_position).unwrap();
    let quat = origin_transform.rotation;

    assert_eq!(Direction::from(rotation), direction);
    assert_eq!(Quat::try_from(direction).unwrap(), quat);
    assert_eq!(Quat::from(rotation), quat);
    assert_eq!(
        Direction::from(Quat::try_from(direction).unwrap()),
        direction
    );
    assert_eq!(Rotation::try_from(Quat::from(rotation)).unwrap(), rotation);
}

#[test]
fn quaternion_conversion() {
    // Cardinal directions
    assert_quaternion_conversion_correct(Position::new(0.0, 1.0));
    assert_quaternion_conversion_correct(Position::new(0.0, -1.0));
    assert_quaternion_conversion_correct(Position::new(1.0, 0.0));
    assert_quaternion_conversion_correct(Position::new(-1.0, 0.0));

    // Offset directions
    assert_quaternion_conversion_correct(Position::new(1.0, 1.0));
    assert_quaternion_conversion_correct(Position::new(1.0, -1.0));
    assert_quaternion_conversion_correct(Position::new(-1.0, 1.0));
    assert_quaternion_conversion_correct(Position::new(1.0, -1.0));

    // Scaled values
    assert_quaternion_conversion_correct(Position::new(0.01, 0.01));
    assert_quaternion_conversion_correct(Position::new(1000.0, 1000.0));

    // Arbitrary values
    assert_quaternion_conversion_correct(Position::new(47.8, 0.03));
    assert_quaternion_conversion_correct(Position::new(-4001.0, 432.7));
}
