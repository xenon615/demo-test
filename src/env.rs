
use bevy::prelude::*;
use avian3d::prelude::*;

pub struct EnvPlugin;
impl Plugin for EnvPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, startup)
        ;
    }
}

// ---

#[derive(Component)]
pub struct  Bar;

// ---

fn startup(
    mut cmd: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut al: ResMut<AmbientLight>
) {
    al.brightness = 1000.;
    let mh = materials.add(Color::WHITE);
    let bar_dim = Vec3::new(8., 1., 40.);
    // cmd.spawn((
    //     MaterialMeshBundle {
    //         transform: Transform::from_xyz(0., -12., 0.),
    //         material: mh.clone(),
    //         mesh: meshes.add(Cuboid::from_size(bar_dim)),
    //         ..default()
    //     },
    //     RigidBody::Static,
    //     Collider::cuboid(bar_dim.x, bar_dim.y, bar_dim.z),
    //     Bar
    // ));

}

