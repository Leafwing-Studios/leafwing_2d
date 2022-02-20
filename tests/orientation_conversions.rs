use bevy::math::{Quat, Vec3};
use bevy::transform::components::Transform;
use leafwing_2d::orientation::*;
use leafwing_2d::position::Position;

const ROTATION_TOL: Rotation = Rotation::new(5);
const QUAT_TOL: f32 = 0.1;

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
fn orientation_alignment() {
    let due_north: Position<f32> = Position::new(0.0, 1.0);
    let origin = Position::default();

    let rotation = origin.rotation_to(due_north).unwrap();
    let direction = origin.direction_to(due_north).unwrap();

    assert_eq!(rotation, Rotation::NORTH);
    assert_eq!(direction, Direction::NORTH);
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
    assert!(
        Direction::NORTH
            .distance(Direction::from(Rotation::new(0)))
            .unwrap()
            <= ROTATION_TOL
    );

    assert!(
        Direction::NORTHEAST
            .distance(Direction::from(Rotation::new(450)))
            .unwrap()
            <= ROTATION_TOL
    );

    assert!(
        Direction::WEST
            .distance(Direction::from(Rotation::new(2700)))
            .unwrap()
            <= ROTATION_TOL
    );

    assert!(
        Direction::NORTH
            .distance(Direction::from(Rotation::new(3600)))
            .unwrap()
            <= ROTATION_TOL
    );
}

fn assert_rotation_quat_conversions_match(radians: f32) {
    assert_eq!(
        Quat::from_rotation_z(radians),
        Quat::from(Rotation::from_radians(radians))
    );

    assert_eq!(
        Rotation::from(Quat::from_rotation_z(radians)),
        Rotation::from_radians(radians)
    );
}

#[test]
fn rotation_quat_conversion() {
    use core::f32::consts::TAU;

    assert_rotation_quat_conversions_match(0.0);
    assert_rotation_quat_conversions_match(TAU / 4.0);
    assert_rotation_quat_conversions_match(TAU / 2.0);
    assert_rotation_quat_conversions_match(3.0 * TAU / 4.0);

    assert_rotation_quat_conversions_match(TAU / 6.0);
    assert_rotation_quat_conversions_match(-TAU / 6.0);
}

fn assert_direction_quat_conversions_match(radians: f32) {
    assert_eq!(
        Quat::from_rotation_z(radians),
        Quat::from(Direction::from(Rotation::from_radians(radians)))
    );

    assert_eq!(
        Direction::from(Quat::from_rotation_z(radians)),
        Direction::from(Rotation::from_radians(radians))
    );
}

#[test]
fn direction_quat_conversion() {
    use core::f32::consts::TAU;

    assert_direction_quat_conversions_match(0.0);
    assert_direction_quat_conversions_match(TAU / 4.0);
    assert_direction_quat_conversions_match(TAU / 2.0);
    assert_direction_quat_conversions_match(3.0 * TAU / 4.0);

    assert_direction_quat_conversions_match(TAU / 6.0);
    assert_direction_quat_conversions_match(-TAU / 6.0);
}

fn assert_conversions_match(target_position: Position<f32>) {
    dbg!(target_position);

    let target_vec3 = target_position.into();
    let origin = Position::<f32>::default();
    let mut origin_transform = Transform::default();

    origin_transform.look_at(target_vec3, Vec3::Z);

    let direction = origin.direction_to(target_position).unwrap();
    let rotation = origin.rotation_to(target_position).unwrap();
    let quat = origin_transform.rotation;

    let rotation_direction = Direction::from(rotation);
    let direction_rotation = Rotation::from(direction);
    let direction_quat = Quat::from(direction);
    let rotation_quat = Quat::from(rotation);
    let quat_direction = Direction::from(quat);
    let quat_rotation = Rotation::from(quat);

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
fn holistic_conversions() {
    // Cardinal directions
    assert_conversions_match(Position::new(0.0, 1.0));
    assert_conversions_match(Position::new(0.0, -1.0));
    assert_conversions_match(Position::new(1.0, 0.0));
    assert_conversions_match(Position::new(-1.0, 0.0));

    // Offset directions
    assert_conversions_match(Position::new(1.0, 1.0));
    assert_conversions_match(Position::new(1.0, -1.0));
    assert_conversions_match(Position::new(-1.0, 1.0));
    assert_conversions_match(Position::new(1.0, -1.0));

    // Scaled values
    assert_conversions_match(Position::new(0.01, 0.01));
    assert_conversions_match(Position::new(1000.0, 1000.0));

    // Arbitrary values
    assert_conversions_match(Position::new(47.8, 0.03));
    assert_conversions_match(Position::new(-4001.0, 432.7));
}
