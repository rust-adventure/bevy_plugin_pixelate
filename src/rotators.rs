use std::f32::consts::FRAC_PI_4;

use bevy::prelude::*;

#[derive(Component)]
pub struct Rotate;

/// Rotates the inner cube (first pass)
pub fn rotator_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Rotate>>,
) {
    for mut transform in &mut query {
        transform.rotate_x(1.5 * time.delta_seconds());
        transform.rotate_z(1.3 * time.delta_seconds());
    }
}

pub fn light_rotator_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<PointLight>>,
) {
    for mut transform in &mut query {
        transform.translation = Transform::from_xyz(
            (time.elapsed_seconds()
                + transform.translation.z / 1.)
                .sin()
                * 10.0,
            4.0,
            transform.translation.z, // (time.elapsed_seconds() / 1.).cos() * 10.0,
        )
        .translation;
    }
}

#[derive(Component)]
pub struct CircleRotate;

pub fn circle_rotator_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<CircleRotate>>,
    mut gizmos: Gizmos,
) {
    for mut transform in &mut query {
        let trans = Transform::from_xyz(
            (time.elapsed_seconds() + 1. / 1.).sin() * 10.0,
            0.0,
            (time.elapsed_seconds() + 1. / 1.).cos() * 10.0,
        );

        let z2 = trans.translation.x / trans.translation.z;
        let tangent =
            Vec3::new(1., 0., z2).normalize().abs();
        // trans.tan
        transform.translation = Transform::from_xyz(
            (time.elapsed_seconds() / 1.).sin() * 10.0,
            0.0,
            (time.elapsed_seconds() / 1.).cos() * 10.0,
        )
        .translation;
        // gizmos.sphere(
        //     transform.translation + tangent,
        //     Quat::from_rotation_y(0.),
        //     1.,
        //     Color::RED,
        // );
        // dbg!(tangent);
        *transform = transform
            .looking_at(trans.translation, Vec3::Y);
        // .with_rotation(Quat::from_rotation_y(
        //     (time.elapsed_seconds() / 1.).sin() * 10.0,
        // ));
    }
}
