use bevy::prelude::*;
use avian3d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
// mod balance;
mod camera;
mod pendulum;
fn main() {
    App::new()
    .insert_resource(ClearColor(Color::BLACK))
    .insert_resource(SubstepCount(80))
    .add_plugins((
        DefaultPlugins,
        PhysicsPlugins::default(), 
        PhysicsDebugPlugin::default(),
        camera::CameraPlugin,
        // balance::BPlugin,
        pendulum::PenPlugin,
        WorldInspectorPlugin::new()
    ))
    .run();
}
