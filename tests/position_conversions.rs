use bevy::math::Vec3;
use leafwing_2d::continuous::F32;
use leafwing_2d::position::Position;

#[test]
fn position_to_vec3() {
    assert_eq!(
        Vec3::from(Position::<F32>::new(0., 0.)),
        Vec3::new(0., 0., 0.)
    );

    assert_eq!(
        Vec3::from(Position::<F32>::new(1., 0.)),
        Vec3::new(1., 0., 0.)
    );

    assert_eq!(
        Vec3::from(Position::<F32>::new(0., 1.)),
        Vec3::new(0., 1., 0.)
    );

    assert_eq!(
        Vec3::from(Position::<F32>::new(1., 1.)),
        Vec3::new(1., 1., 0.)
    );

    assert_eq!(
        Vec3::from(Position::<F32>::new(-1., -1.)),
        Vec3::new(-1., -1., 0.)
    );

    assert_eq!(
        Vec3::from(Position::<F32>::new(-42., 3.)),
        Vec3::new(-42., 3., 0.)
    );
}

#[test]
fn vec3_to_position() {
    assert_eq!(
        Ok(Position::<F32>::new(0., 0.)),
        Vec3::new(0., 0., 0.).try_into()
    );

    assert_eq!(
        Ok(Position::<F32>::new(1., 0.)),
        Vec3::new(1., 0., 0.).try_into()
    );

    assert_eq!(
        Ok(Position::<F32>::new(0., 1.)),
        Vec3::new(0., 1., 0.).try_into()
    );

    assert_eq!(
        Ok(Position::<F32>::new(1., 1.)),
        Vec3::new(1., 1., 0.).try_into()
    );

    assert_eq!(
        Ok(Position::<F32>::new(-1., -1.)),
        Vec3::new(-1., -1., 0.).try_into()
    );

    assert_eq!(
        Ok(Position::<F32>::new(-42., 3.)),
        Vec3::new(-42., 3., 0.).try_into()
    );

    assert_eq!(
        Ok(Position::<F32>::new(-42., 3.)),
        Vec3::new(-42., 3., 17.).try_into()
    );
}
