
use std::f32::consts::PI;

use bevy::prelude::*;
use avian3d::prelude::*;

pub struct BPlugin;
impl Plugin for BPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, startup)
        ;
    }
}

// ---

#[derive(Component)]
pub struct  Beam;

#[derive(Component)]
pub struct  Pivot;

// ---

fn startup(
    mut cmd: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut al: ResMut<AmbientLight>
) {
    al.brightness = 500.;
    let mh = materials.add(Color::WHITE);



    let beam_dim = Vec3::new(1., 1., 10.);
    let beam_id = cmd.spawn((
        MaterialMeshBundle {
            material: mh.clone(),
            mesh: meshes.add(Cuboid::from_size(beam_dim)),
            ..default()
        },
        RigidBody::Dynamic,
        MassPropertiesBundle::new_computed(&Collider::cuboid(beam_dim.x, beam_dim.y, beam_dim.z), 1.),
        Beam
    )).id();

    // --- PIVOT -----------

    let pivot_id = cmd.spawn((
        TransformBundle::default(),
        RigidBody::Static,
    )).id();

    cmd.spawn((
        RevoluteJoint::new(pivot_id, beam_id)
        .with_aligned_axis(Vec3::X)
        .with_angular_velocity_damping(100.)
        ,
        
        Pivot,
    ));

    // ---CHAIN

    let chain_len = beam_dim.z * 0.9;
    let element_count = 12;
    let element_dim = Vec3::new(0.1, 0.1, chain_len / element_count as f32);
    let anchor_element = Vec3::Z * element_dim.z / 2.;
    let element_mesh = meshes.add(Cuboid::from_size(element_dim));

    for (idx, init_pos) in [
        -Vec3::Z * beam_dim.z / 2., 
        Vec3::Z * beam_dim.z / 2. 
    ].iter().enumerate() {
        let mut pos = *init_pos;     
        let mut prev_element_id = beam_id;
        for i in 0 .. element_count {
            let element_id = cmd.spawn((
                MaterialMeshBundle {
                    transform: Transform::from_translation(pos).with_rotation(Quat::from_rotation_x(PI * 0.5)),
                    material: mh.clone(),
                    mesh: element_mesh.clone(),
                    ..default()
                },
                RigidBody::Dynamic,
                // RigidBody::Static,
                MassPropertiesBundle::new_computed(&Collider::cuboid(element_dim.x, element_dim.y, element_dim.z), 10.),
            )).id();
    
            cmd.spawn(
                RevoluteJoint::new(prev_element_id, element_id)
                .with_local_anchor_1(if i == 0 {*init_pos} else {-anchor_element})
                .with_local_anchor_2(anchor_element)
                .with_aligned_axis(Vec3::X)
            );
            prev_element_id  = element_id;
            pos -= element_dim.z * Vec3::Y;
        }

        // --- BALL --------------------------------------------------------------------

        let ball_radius = 0.9;
        let msh = meshes.add(Cuboid::from_length(2. * ball_radius));
        // let msh = meshes.add(Sphere::new(ball_radius));

        let ball_id = cmd.spawn((
            MaterialMeshBundle {
                transform: Transform::from_translation(pos - anchor_element - Vec3::Z * ball_radius),
                mesh: msh.clone(),
                material: mh.clone(),
                ..default()
            }, 
            RigidBody::Dynamic,
            // Collider::cuboid(ball_radius * 2. , ball_radius * 2., ball_radius * 2.),
            Collider::sphere(ball_radius),
            ColliderDensity((idx * 10)  as f32 + 20.)
        ))
        .id()
        ;

        cmd.spawn(
            RevoluteJoint::new(prev_element_id, ball_id)
            .with_aligned_axis(Vec3::X)
            .with_local_anchor_1(-anchor_element)
            // .with_local_anchor_2(-Vec3::Z * 2.)
            .with_compliance(0.)
        );

    }
    // --- BAR ----------------------------------------------------------------------

    let bar_dim = Vec3::new(8., 1., 40.);
    cmd.spawn((
        MaterialMeshBundle {
            transform: Transform::from_xyz(0., -12., 0.),
            material: mh.clone(),
            mesh: meshes.add(Cuboid::from_size(bar_dim)),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(bar_dim.x, bar_dim.y, bar_dim.z),
        Dominance(10)
    ));

}

