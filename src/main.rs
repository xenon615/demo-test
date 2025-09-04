use bevy::{
    prelude::*, 
    render::view::NoIndirectDrawing,
};

use bevy_panorbit_camera::{PanOrbitCameraPlugin, PanOrbitCamera};

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_egui::EguiPlugin;


// ---

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins(DefaultPlugins)
    .add_plugins(PanOrbitCameraPlugin)
    .add_plugins(EguiPlugin::default() )
    .add_plugins(WorldInspectorPlugin::new())
    .add_systems(Startup, startup)
    .run()
    ;
}    

//  ---
    #[derive(Component, Clone)]
    struct MyCube;

    #[derive(Component)]
    struct Cloned;

// ---

fn startup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut al: ResMut<AmbientLight>
) {

    al.brightness = 5000.;
    cmd.spawn((
        Mesh3d(meshes.add(Cuboid::from_size(Vec3::new(4., 0.5, 0.1)))),
        MeshMaterial3d(materials.add(Color::srgb(1., 1., 0.))),
        Transform::from_xyz(0., 0., -2.)
    ));

    let material_handle = materials.add(Color::srgba(0., 0., 1., 0.5));

    let mut id = cmd.spawn((
        Mesh3d(meshes.add(Cuboid::from_length(1.))),
        MeshMaterial3d(material_handle.clone()),
        Transform::from_xyz(-0.5, 0., 0.),
        Name::new("Original"),
        MyCube
    ));

    id.clone_and_spawn().insert((Transform::from_xyz(0.5, 0., 0.), Cloned, Name::new("Cloned")));

    cmd.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 0., 10.).looking_at(Vec3::ZERO, Vec3::Y),
        NoIndirectDrawing , 
        PanOrbitCamera::default()
    ));
}

