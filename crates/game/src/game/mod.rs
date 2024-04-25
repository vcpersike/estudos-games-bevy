pub mod entities;
pub mod camera;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::game::entities::player::Player;

use self::camera::CameraPlugin;

pub struct GamePlugin;

impl  Plugin for GamePlugin {
    fn build(&self, app: &mut App) {

        app.add_plugins(CameraPlugin).add_systems(Startup, setup).add_systems(Update, update);
    }
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::new(1.0, 1.0)).into(),
        material: materials.add(Color::GRAY),
        transform: Transform::default().with_scale(Vec3::splat(100.0)),
        ..Default::default()
    }, Player));
}

fn update(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>, mut query: Query<(&Player, &mut Transform)>) {

    for (_, mut transform) in query.iter_mut() {
        let time_step = time.delta_seconds() * 100.0;
        if keyboard_input.pressed(KeyCode::KeyD) {
            transform.translation.x += time_step;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            transform.translation.x -=  time_step;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            transform.translation.y -=  time_step;
        }
        if keyboard_input.pressed(KeyCode::KeyW) {
            transform.translation.y +=  time_step;
        }
        if keyboard_input.pressed(KeyCode::KeyQ) {
            transform.rotation *= Quat::from_rotation_z(0.1);
        }
        if keyboard_input.pressed(KeyCode::KeyE) {
            transform.rotation *= Quat::from_rotation_z(-0.1);
        }
        if keyboard_input.pressed(KeyCode::KeyR) {
            transform.rotation *= Quat::from_rotation_y(-0.1);
        }
    }
}