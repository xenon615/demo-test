use bevy::prelude::*;
use avian3d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
mod camera;
mod env;
mod balance;
mod pendulum;
mod e1;
mod e2;
fn main() {
    App::new()
    .insert_resource(ClearColor(Color::BLACK))
    .insert_resource(SubstepCount(80))
    .add_plugins((
        DefaultPlugins,
        PhysicsPlugins::default(), 
        PhysicsDebugPlugin::default(),
        camera::CameraPlugin,
        // env::EnvPlugin,
        // balance::BPlugin,
        // pendulum::PenPlugin,
        // e1::E1Plugin,
        e2::E2Plugin,
        WorldInspectorPlugin::new()
    ))
    .run();
}
