use bevy::prelude::*;


pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    println!("Hello from the camera plugin!");
    commands.spawn(Camera2dBundle::default());
}