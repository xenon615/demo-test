use bevy::{
    prelude::*,
    pbr::Material,
    render::render_resource::{ShaderRef, AsBindGroup}
};

// ---

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins((DefaultPlugins, MaterialPlugin::<CustomMaterial>::default()))
    .add_systems(Startup, startup)
    .run();
}    

// ---

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {}
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/cm.wgsl".into()
    }
}

//  ---

fn startup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut c_materials: ResMut<Assets<CustomMaterial>>,
) {
    cmd.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(Color::srgb(1., 0., 0.))),
        Transform::from_xyz(-2., 0., 0.)
    ));

    cmd.spawn((
        Mesh3d(meshes.add(Cuboid::from_length(1.))),
        MeshMaterial3d(c_materials.add(CustomMaterial{})),
        Transform::from_xyz(2., 0., 0.)
    ));

    cmd.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 0., 10.).looking_at(Vec3::ZERO, Vec3::Y)
    ));

    cmd.spawn(DirectionalLight {
        illuminance: 5_000.,
        ..default()
    });


}


