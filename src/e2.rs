
use std::f32::consts::PI;

use bevy::{math::VectorSpace, prelude::*};
use avian3d::prelude::*;

pub struct E2Plugin;
impl Plugin for E2Plugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, startup)
        .add_systems(Update, show_gizmos)
        // .add_systems(Update, key)
        ;
    }
}

// ---

#[derive(Component)]
pub struct Main;

#[derive(Component)]
pub struct Element;

// ---

fn startup(
    mut cmd: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mh = materials.add(Color::WHITE);

    cmd.spawn((
        MaterialMeshBundle {
            material: materials.add(Color::srgba(0., 0., 1., 0.1)),
            mesh: meshes.add(Cuboid::from_length(20.)),
            transform: Transform::from_rotation(Quat::from_rotation_y(PI / 3.)),
            ..default()
        },
        Name::new("Parent"),
        Main
    ))
    .with_children(|base| {
        let base_id = base.spawn((
            MaterialMeshBundle {
                material: materials.add(Color::srgb(1., 0., 0.)),
                mesh: meshes.add(Sphere::new(0.5)),
                transform: Transform::from_rotation(Quat::from_rotation_y(PI / 4.)),
                ..default()
            },
            RigidBody::Static
        )).id();

        // ---CHAIN

        let element_count = 10;
        let element_dim = Vec3::new(0.1, 0.1, 1.);
        let element_anchor = Vec3::Z * element_dim.z * 0.5;
        let element_mesh = meshes.add(Cuboid::from_size(element_dim));
        let mut pos = -element_anchor;
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
                // RigidBody::Static,
                Element,
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
            ColliderDensity(0.1),
        ))
        .id()
        ;
    
        base.spawn(
            SphericalJoint::new(prev_element_id, ball_id)
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

// fn key(
//     keys: Res<ButtonInput<KeyCode>>,
//     mut q: Query<&mut RigidBody, With<Element>>
// ) {
//     for mut r in &mut q  {
//         if keys.just_pressed(KeyCode::Space) {
//             *r = RigidBody::Dynamic;
//         }
//     }
// }