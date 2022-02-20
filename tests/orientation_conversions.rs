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
    const ROTATION_TOL: Rotation = Rotation::new(5);
    const QUAT_TOL: f32 = 0.1;

    dbg!(target_position);

    let target_vec3 = target_position.into();
    let origin = Position::<f32>::default();
    let mut origin_transform = Transform::default();

    origin_transform.look_at(target_vec3, Vec3::Z);

    let direction = origin.direction_to(target_position);
    let rotation = origin.rotation_to(target_position).unwrap();
    let quat = origin_transform.rotation;

    let rotation_direction = Direction::from(rotation);
    let direction_rotation = Rotation::try_from(direction).unwrap();
    let direction_quat = Quat::try_from(direction).unwrap();
    let rotation_quat = Quat::try_from(rotation).unwrap();
    let quat_direction = Direction::from(quat);
    let quat_rotation = Rotation::try_from(quat).unwrap();

    dbg!(direction);
    dbg!(rotation_direction);
    dbg!(quat_direction);

    dbg!(rotation);
    dbg!(direction_rotation);
    dbg!(quat_rotation);

    dbg!(quat);
    dbg!(direction_quat);
    dbg!(rotation_quat);

    assert!(direction.distance(rotation_direction).unwrap() <= ROTATION_TOL);
    assert!(direction.distance(quat_direction).unwrap() <= ROTATION_TOL);

    assert!(rotation.distance(direction_rotation) <= ROTATION_TOL);
    assert!(rotation.distance(quat_rotation) <= ROTATION_TOL);

    assert!(quat.abs_diff_eq(direction_quat, QUAT_TOL));
    assert!(quat.abs_diff_eq(rotation_quat, QUAT_TOL));
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
