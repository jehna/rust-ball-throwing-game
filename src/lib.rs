use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::prelude::*;
use web_sys::Blob;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Collider::cuboid(5., 0.1, 5.))
                .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -0.1, 0.0)));
        });

    // player
    let cube_transform = Transform::from_xyz(0.0, 0.5, 0.0);
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: cube_transform,
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Restitution::coefficient(0.7))
        .insert(Velocity::zero())
        .insert(User { speed: 5. })
        .with_children(|parent| {
            parent.spawn_bundle(Camera3dBundle {
                transform: Transform::from_xyz(0., 2.5, -5.0)
                    .looking_at(cube_transform.translation, Vec3::Y),
                ..default()
            });
        });

    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

#[derive(Component)]
pub struct User {
    speed: f32,
}

pub fn user_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut mouse_input: EventReader<MouseMotion>,
    mut component: Query<(&User, &mut Transform, &mut Velocity)>,
) {
    for (user, mut transform, mut velociy) in &mut component {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::W) {
            direction -= transform.forward();
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction += transform.forward();
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction -= transform.left();
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction += transform.left();
        }
        transform.translation += direction * 0.01 * user.speed;

        if keyboard_input.pressed(KeyCode::Space) {
            velociy.linvel.y = 3.0;
        }

        for ev in mouse_input.iter() {
            transform.rotation *= Quat::from_rotation_y(ev.delta.x * -0.01);
        }
    }
}

pub fn cursor_grab_system(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }

    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }
}
