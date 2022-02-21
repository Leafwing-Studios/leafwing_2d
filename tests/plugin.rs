use bevy::ecs::query::{FilterFetch, WorldQuery};
use bevy::prelude::*;
use core::fmt::Debug;
use leafwing_2d::orientation::Direction;
use leafwing_2d::prelude::*;

trait AppExtension {
    fn assert_component_eq<C, F>(&mut self, value: &C)
    where
        C: Component + PartialEq + Debug,
        F: WorldQuery,
        <F as WorldQuery>::Fetch: FilterFetch;

    fn set_component<C, F>(&mut self, value: &C)
    where
        C: Component + PartialEq + Debug + Clone,
        F: WorldQuery,
        <F as WorldQuery>::Fetch: FilterFetch;
}

impl AppExtension for App {
    fn assert_component_eq<C, F>(&mut self, value: &C)
    where
        C: Component + PartialEq + Debug,
        F: WorldQuery,
        <F as WorldQuery>::Fetch: FilterFetch,
    {
        let mut query_state = self.world.query_filtered::<(Entity, &C), F>();
        for (entity, component) in query_state.iter(&self.world) {
            if component != value {
                panic!(
                    "Found component {component:?} for {entity:?}, but was expecting {value:?}."
                );
            }
        }
    }

    fn set_component<C, F>(&mut self, value: &C)
    where
        C: Component + PartialEq + Debug + Clone,
        F: WorldQuery,
        <F as WorldQuery>::Fetch: FilterFetch,
    {
        let mut query_state = self.world.query_filtered::<&mut C, F>();
        for mut component in query_state.iter_mut(&mut self.world) {
            if *component != *value {
                *component = value.clone();
            }
        }
    }
}

fn test_app() -> App {
    let mut app = App::new();
    app.add_plugin(TwoDimPlugin::<f32>::default());
    app.add_startup_system(test_entity);
    app.add_system_to_stage(CoreStage::Last, assert_orientation_matches);
    app.add_system_to_stage(CoreStage::Last, assert_position_matches);

    app
}

fn test_entity(mut commands: Commands) {
    commands.spawn_bundle(TwoDimBundle::<f32>::default());
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
        transform.assert_approx_eq(position);
    }
}

#[test]
fn sync_orientation() {
    let mut app = test_app();

    // Run startup systems
    app.update();

    // Changing direction
    app.set_component::<Direction, ()>(&Direction::NORTH);
    app.update();
    app.assert_component_eq::<Rotation, ()>(&Rotation::NORTH);
    app.assert_component_eq::<Transform, ()>(&Transform::from_rotation(Direction::NORTH.into()));

    // Changing rotation
    app.set_component::<Rotation, ()>(&Rotation::EAST);
    app.update();
    app.assert_component_eq::<Direction, ()>(&Direction::EAST);
    app.assert_component_eq::<Transform, ()>(&Transform::from_rotation(Direction::EAST.into()));

    // Changing rotation and direction (rotation wins)
    app.set_component::<Rotation, ()>(&Rotation::WEST);
    app.set_component::<Direction, ()>(&Direction::SOUTH);
    app.update();
    app.assert_component_eq::<Direction, ()>(&Direction::WEST);
    app.assert_component_eq::<Rotation, ()>(&Rotation::WEST);
    app.assert_component_eq::<Transform, ()>(&Transform::from_rotation(Direction::WEST.into()));

    // Changing transform quat
    app.set_component::<Transform, ()>(&Transform::from_rotation(Rotation::NORTHEAST.into()));
    app.update();
    app.assert_component_eq::<Direction, ()>(&Direction::NORTHEAST);
    app.assert_component_eq::<Rotation, ()>(&Rotation::NORTHEAST);
    app.assert_component_eq::<Transform, ()>(&Transform::from_rotation(Direction::WEST.into()));

    // Changing transform and direction (direction wins)
    app.set_component::<Transform, ()>(&Transform::from_rotation(Rotation::SOUTHEAST.into()));
    app.set_component::<Direction, ()>(&Direction::NORTH);
    app.update();
    app.assert_component_eq::<Direction, ()>(&Direction::NORTHEAST);
    app.assert_component_eq::<Rotation, ()>(&Rotation::NORTHEAST);
    app.assert_component_eq::<Transform, ()>(&Transform::from_rotation(Direction::WEST.into()));
}

#[test]
fn sync_position() {
    let mut app = test_app();

    // Run startup systems
    app.update();

    // Changing position
    app.set_component::<Position<f32>, ()>(&Position::new(1.0, 1.0));
    app.update();
    app.assert_component_eq::<Transform, ()>(&Transform::from_xyz(1.0, 1.0, 0.0));

    // Changing transform translation
    app.set_component::<Transform, ()>(&Transform::from_xyz(2.0, 2.0, 0.0));
    app.update();
    app.assert_component_eq::<Position<f32>, ()>(&Position::new(2.0, 2.0));

    // Changing transform and position (position wins)
    app.set_component::<Position<f32>, ()>(&Position::new(3.0, 3.0));
    app.set_component::<Transform, ()>(&Transform::from_xyz(0.0, 42.0, 0.0));
    app.update();
    app.assert_component_eq::<Transform, ()>(&Transform::from_xyz(3.0, 3.0, 0.0));
    app.assert_component_eq::<Position<f32>, ()>(&Position::new(3.0, 3.0));

    // Z is unmodified
    app.set_component::<Transform, ()>(&Transform::from_xyz(0.0, 42.0, 5.0));
    app.set_component::<Position<f32>, ()>(&Position::new(4.0, 4.0));

    app.update();
    app.assert_component_eq::<Transform, ()>(&Transform::from_xyz(3.0, 3.0, 5.0));
}
