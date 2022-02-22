use bevy::prelude::*;
use core::fmt::Debug;
use leafwing_2d::orientation::Direction;
use leafwing_2d::prelude::*;

trait AppExtension {
    fn assert_component_eq<C: Component + PartialEq + Debug>(&mut self, value: &C);

    fn set_component<C: Component + PartialEq + Debug + Clone>(&mut self, value: C);

    fn assert_orientation_approx_eq<C: Component + Orientation>(&mut self, value: C);

    fn assert_positionlike_approx_eq<C: Component + Positionlike>(&mut self, value: C);
}

impl AppExtension for App {
    fn assert_component_eq<C: Component + PartialEq + Debug>(&mut self, value: &C) {
        let mut query_state = self.world.query::<(Entity, &C)>();
        for (entity, component) in query_state.iter(&self.world) {
            if component != value {
                panic!(
                    "Found component {component:?} for {entity:?}, but was expecting {value:?}."
                );
            }
        }
    }

    fn set_component<C: Component + PartialEq + Debug + Clone>(&mut self, value: C) {
        let mut query_state = self.world.query::<&mut C>();
        for mut component in query_state.iter_mut(&mut self.world) {
            if *component != value {
                *component = value.clone();
            }
        }
    }

    fn assert_orientation_approx_eq<C: Component + Orientation>(&mut self, value: C) {
        let mut query_state = self.world.query::<&C>();
        for &component in query_state.iter(&self.world) {
            component.assert_approx_eq(value);
        }
    }

    fn assert_positionlike_approx_eq<C: Component + Positionlike>(&mut self, value: C) {
        let mut query_state = self.world.query::<&C>();
        for &component in query_state.iter(&self.world) {
            component.assert_approx_eq(value);
        }
    }
}

fn test_app() -> App {
    let mut app = App::new();
    app.add_plugin(TwoDPlugin::<f32>::default());
    app.add_startup_system(test_entity);
    app.add_system_to_stage(CoreStage::Last, assert_orientation_matches);
    app.add_system_to_stage(CoreStage::Last, assert_position_matches);

    app
}

fn test_entity(mut commands: Commands) {
    commands.spawn_bundle(TwoDBundle::<f32>::default());
}

fn assert_orientation_matches(query: Query<(Option<&Rotation>, Option<&Direction>, &Transform)>) {
    for (maybe_rotation, maybe_direction, transform) in query.iter() {
        if let Some(&rotation) = maybe_rotation {
            transform.rotation.assert_approx_eq(rotation);
        }

        if let Some(&direction) = maybe_direction {
            transform.rotation.assert_approx_eq(direction);
        }
    }
}

fn assert_position_matches(query: Query<(&Position<f32>, &Transform)>) {
    for (&position, &transform) in query.iter() {
        transform.translation.assert_approx_eq(position);
    }
}

#[test]
fn sync_orientation() {
    let mut app = test_app();

    // Run startup systems
    app.update();

    // Changing direction
    app.set_component(Direction::NORTH);
    app.update();
    app.assert_orientation_approx_eq(Rotation::NORTH);
    app.assert_orientation_approx_eq(Transform::from_rotation(Direction::NORTH.into()));

    // Changing rotation
    app.set_component(Rotation::EAST);
    app.update();
    app.assert_orientation_approx_eq(Direction::EAST);
    app.assert_orientation_approx_eq(Transform::from_rotation(Direction::EAST.into()));

    // Changing rotation and direction (rotation wins)
    app.set_component(Rotation::WEST);
    app.set_component(Direction::SOUTH);
    app.update();
    app.assert_orientation_approx_eq(Direction::WEST);
    app.assert_orientation_approx_eq(Rotation::WEST);
    app.assert_orientation_approx_eq(Transform::from_rotation(Direction::WEST.into()));

    // Changing transform quat
    app.set_component(Transform::from_rotation(Rotation::NORTHEAST.into()));
    app.update();
    app.assert_orientation_approx_eq(Direction::NORTHEAST);
    app.assert_orientation_approx_eq(Rotation::NORTHEAST);
    app.assert_orientation_approx_eq(Transform::from_rotation(Direction::NORTHEAST.into()));

    // Changing transform and direction (rotation wins)
    app.set_component(Transform::from_rotation(Rotation::SOUTHEAST.into()));
    app.set_component(Rotation::NORTH);
    app.update();
    app.assert_orientation_approx_eq(Direction::NORTH);
    app.assert_orientation_approx_eq(Rotation::NORTH);
    app.assert_orientation_approx_eq(Transform::from_rotation(Direction::NORTH.into()));

    // Changing transform and direction (direction wins)
    app.set_component(Transform::from_rotation(Rotation::SOUTHEAST.into()));
    app.set_component(Direction::SOUTH);
    app.update();
    app.assert_orientation_approx_eq(Direction::SOUTH);
    app.assert_orientation_approx_eq(Rotation::SOUTH);
    app.assert_orientation_approx_eq(Transform::from_rotation(Direction::SOUTH.into()));

    // Changing all three (rotation wins)
    app.set_component(Transform::from_rotation(Rotation::SOUTHEAST.into()));
    app.set_component(Direction::WEST);
    app.set_component(Rotation::NORTH);
    app.update();
    app.assert_orientation_approx_eq(Direction::NORTH);
    app.assert_orientation_approx_eq(Rotation::NORTH);
    app.assert_orientation_approx_eq(Transform::from_rotation(Direction::NORTH.into()));
}

#[test]
fn sync_position() {
    let mut app = test_app();

    // Run startup systems
    app.update();

    // Changing position
    app.set_component(Position::new(1.0, 1.0));
    app.update();
    app.assert_positionlike_approx_eq(Transform::from_xyz(1.0, 1.0, 0.0));

    // Changing transform translation
    app.set_component(Transform::from_xyz(2.0, 2.0, 0.0));
    app.update();
    app.assert_positionlike_approx_eq(Position::new(2.0, 2.0));

    // Changing transform and position (position wins)
    app.set_component(Position::new(3.0, 3.0));
    app.set_component(Transform::from_xyz(0.0, 42.0, 0.0));
    app.update();
    app.assert_positionlike_approx_eq(Transform::from_xyz(3.0, 3.0, 0.0));
    app.assert_positionlike_approx_eq(Position::new(3.0, 3.0));

    // Z is unmodified
    app.set_component(Transform::from_xyz(0.0, 42.0, 5.0));
    app.set_component(Position::new(4.0, 4.0));

    app.update();
    app.assert_positionlike_approx_eq(Transform::from_xyz(4.0, 4.0, 5.0));
}
