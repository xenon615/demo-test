
use std::f32::consts::PI;

use bevy::{math::VectorSpace, prelude::*};
use avian3d::prelude::*;

pub struct E2Plugin;
impl Plugin for E2Plugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, startup)
        .add_systems(Update, show_gizmos)
        ;
    }
}

// ---

#[derive(Component)]
pub struct Main;

// ---

fn startup(
    mut cmd: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mh = materials.add(Color::WHITE);

    cmd.spawn((
        TransformBundle::from_transform(
            Transform::from_xyz(0., 0., 0.)
            .with_rotation(Quat::from_rotation_y(-PI / 4.))
        ),
        VisibilityBundle::default(),
        Name::new("Parent"),
        Main
    ))
    .with_children(|base| {
        let base_id = base.spawn(RigidBody::Static).id();

        // ---CHAIN

        let element_count = 15;
        let element_dim = Vec3::new(0.1, 0.1, 1.);
        let element_anchor = Vec3::Z * element_dim.z * 0.5;
        let element_mesh = meshes.add(Cuboid::from_size(element_dim));

        let mut pos = -Vec3::Z * element_dim.z * 0.5;     
        let mut prev_element_id = base_id;

        for i in 0 .. element_count {
            let element_id = base.spawn((
                MaterialMeshBundle {
                    transform: Transform::from_translation(pos)
                    ,
                    material: mh.clone(),
                    mesh: element_mesh.clone(),
                    ..default()
                },
                RigidBody::Dynamic,
                MassPropertiesBundle::new_computed(&Collider::cuboid(element_dim.x, element_dim.y, element_dim.z), 10.),
            )).id();
    
            base.spawn(
                SphericalJoint::new(prev_element_id, element_id)
                .with_local_anchor_1(if i == 0 {Vec3::ZERO } else {-element_anchor})
                .with_local_anchor_2(element_anchor)
            );
            prev_element_id  = element_id;
            pos -= element_anchor;
        }

        // BALL -----------------------------------

        let ball_radius = 1.0;
        let mesh = meshes.add(Sphere::new(ball_radius));
        let collider = Collider::sphere(ball_radius);

        let ball_id = base.spawn((
            MaterialMeshBundle {
                transform: Transform::from_translation(pos - element_anchor - Vec3::Z * ball_radius),
                mesh: mesh.clone(),
                material: mh.clone(),
                ..default()
            }, 
            RigidBody::Dynamic,
            collider,
            ColliderDensity(200.),
        ))
        .id()
        ;
    
        base.spawn(
            RevoluteJoint::new(prev_element_id, ball_id)
            .with_aligned_axis(Vec3::X)
            .with_local_anchor_1(-element_anchor)
        );

    });



}

fn show_gizmos(
    mut gizmos: Gizmos,
    q: Query<&Transform, With<Main>>
) {
    for t  in  &q {
        gizmos.axes(*t, 5.);    
    }
}
