use bevy::math::Quat;
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
fn direction_to_quat() {
    use core::f32::consts::TAU;

    Quat::from_rotation_z(0.0).assert_approx_eq(Direction::NORTH.into());
    Quat::from_rotation_z(TAU / 4.0).assert_approx_eq(Direction::EAST.into());
    Quat::from_rotation_z(TAU / 2.0).assert_approx_eq(Direction::SOUTH.into());
    Quat::from_rotation_z(3.0 * TAU / 4.0).assert_approx_eq(Direction::WEST.into());
}

#[test]
fn quat_to_direction() {
    use core::f32::consts::TAU;

    Direction::NORTH.assert_approx_eq(Quat::from_rotation_z(0.0).into());
    Direction::EAST.assert_approx_eq(Quat::from_rotation_z(TAU / 4.0).into());
    Direction::SOUTH.assert_approx_eq(Quat::from_rotation_z(TAU / 2.0).into());
    Direction::WEST.assert_approx_eq(Quat::from_rotation_z(3.0 * TAU / 4.0).into());
}

#[test]
fn rotation_to_quat() {
    use core::f32::consts::TAU;

    Quat::from_rotation_z(0.0).assert_approx_eq(Rotation::NORTH.into());
    Quat::from_rotation_z(TAU / 4.0).assert_approx_eq(Rotation::EAST.into());
    Quat::from_rotation_z(TAU / 2.0).assert_approx_eq(Rotation::SOUTH.into());
    Quat::from_rotation_z(3.0 * TAU / 4.0).assert_approx_eq(Rotation::WEST.into());
}

#[test]
fn quat_to_rotation() {
    use core::f32::consts::TAU;

    Rotation::NORTH.assert_approx_eq(Quat::from_rotation_z(0.0).into());
    Rotation::EAST.assert_approx_eq(Quat::from_rotation_z(TAU / 4.0).into());
    Rotation::SOUTH.assert_approx_eq(Quat::from_rotation_z(TAU / 2.0).into());
    Rotation::WEST.assert_approx_eq(Quat::from_rotation_z(3.0 * TAU / 4.0).into());
}

#[test]
fn direction_rotation_conversion() {
    Direction::NORTH.assert_approx_eq(Direction::from(Rotation::new(0)));
    Direction::NORTHEAST.assert_approx_eq(Direction::from(Rotation::new(450)));
    Direction::WEST.assert_approx_eq(Direction::from(Rotation::new(2700)));
    Direction::NORTH.assert_approx_eq(Direction::from(Rotation::new(3600)));
}

fn assert_conversions_match(target_position: Position<f32>) {
    dbg!(target_position);

    let origin = Position::<f32>::default();

    let direction = origin.direction_to(target_position).unwrap();
    let rotation = origin.rotation_to(target_position).unwrap();
    let quat = Quat::from_rotation_z(rotation.into_radians());

    let direction_from_rotation = Direction::from(rotation);
    let direction_from_quat = Direction::from(quat);

    direction.assert_approx_eq(direction_from_rotation);
    direction.assert_approx_eq(direction_from_quat);

    let rotation_from_direction = Rotation::from(direction);
    let rotation_from_quat = Rotation::from(quat);

    rotation.assert_approx_eq(rotation_from_direction);
    rotation.assert_approx_eq(rotation_from_quat);

    let quat_from_direction = Quat::from(direction);
    let quat_from_rotation = Quat::from(rotation);

    quat.assert_approx_eq(quat_from_direction);
    quat.assert_approx_eq(quat_from_rotation);
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
