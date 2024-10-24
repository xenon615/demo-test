use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup)
        .add_plugins(PanOrbitCameraPlugin)
        ; 
    }
} 

// ---

fn setup (
    mut commands : Commands,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-50., 0., 0.).looking_at(Vec3::new(0., 0.,0.), Vec3::Y),
            ..default()
        },
        PanOrbitCamera {
            enabled: true,
            ..default()
        }
    ));
}