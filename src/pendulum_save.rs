use std::f32::consts::PI;
use bevy::gizmos::gizmos;
use bevy::prelude::*;
use bevy::math::*;
use avian3d::prelude::*;
use crate::env::Bar;

pub struct PenPlugin;
impl Plugin for PenPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, startup)
        .add_systems(Update, key_control)
        .add_systems(Update, do_tension.run_if(in_state(PState::Tension)))
        .add_systems(OnEnter(PState::Tension), enter_tension)
        .add_systems(OnExit(PState::Tension), exit_tension)
        .add_systems(OnEnter(PState::Arming), enter_arming)
        .add_systems(Update, do_arming.run_if(in_state(PState::Arming)))
        .add_systems(OnEnter(PState::Loose), enter_loose)
        .add_systems(Update, do_loose.run_if(in_state(PState::Loose)))

        .init_state::<PState>()

        ;
    }
}

// ---

#[derive(Component)]
pub struct  Arm;

#[derive(Component)]
pub struct  ArmLongEnd;

#[derive(Component)]
pub struct  Ball;

#[derive(Component)]
pub struct  Pivot;

#[derive(Component)]
pub struct  SlingEnd;

#[derive(Component)]
pub struct  BallLink;


#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum PState {
    #[default]
    Idle,
    Tension,
    Arming,
    Loose,
}

// ---

fn startup(
    mut cmd: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: ResMut<AssetServer>,
) {
    let mh = materials.add(Color::WHITE);

    // --- ARM -----

    let arm_dim = Vec3::new(1., 1., 15.);
    
    let mut arm_force = ExternalForce::default();
    arm_force.apply_force_at_point(Vec3::ZERO, Vec3::NEG_Z * arm_dim.z / 2., Vec3::ZERO);

    let arm_id = cmd.spawn((
        MaterialMeshBundle {
            material: mh.clone(),
            mesh: meshes.add(Cuboid::from_size(arm_dim)),
            ..default()
        },
        RigidBody::Dynamic,
        // RigidBody::Static,
        MassPropertiesBundle::new_computed(&Collider::cuboid(arm_dim.x, arm_dim.y, arm_dim.z), 1.),
        GravityScale(0.1),
        arm_force,
        Arm
    ))

    .with_children(|arm| {
        arm.spawn((
            TransformBundle {
                local: Transform::from_translation(Vec3::NEG_Z * arm_dim.z / 2.),
                ..default()
            },
            ArmLongEnd,
        ));
    })

    .id();

    // --- PIVOT -----------

    let pivot_offset = arm_dim.z  * 0.3;
    let pivot_id = cmd.spawn((
        TransformBundle {
            local: Transform::from_translation(Vec3::Z * pivot_offset),
            ..default()
        },
        RigidBody::Static,
    )).id();

    cmd.spawn((
        RevoluteJoint::new(pivot_id, arm_id)
        .with_local_anchor_2(Vec3::Z * pivot_offset)
        .with_aligned_axis(Vec3::X)
        .with_angular_velocity_damping(10000.1),
        Pivot,
        Name::new("Pivot")
    ));
    
    // --- COUNTERWEIGHT -------------------------

    let cw_radius = 2.;
    let anchor_arm = Vec3::Z * arm_dim.z / 2.;
    let anchor_cw =  Vec3::Z * (2. * cw_radius);

    let cw_id = cmd.spawn((
        MaterialMeshBundle {
            transform: Transform::from_translation(anchor_cw + anchor_arm),
            material: mh.clone(),
            mesh: meshes.add(Sphere::new(cw_radius)),
            ..default()
        },
        RigidBody::Dynamic,
        MassPropertiesBundle::new_computed(&Collider::sphere(cw_radius), 1.),
        GravityScale(10.),
    )).id();

    cmd.spawn(
        RevoluteJoint::new(arm_id, cw_id)
        .with_aligned_axis(Vec3::X)
        .with_local_anchor_1(anchor_arm)
        .with_local_anchor_2(-anchor_cw)
        .with_angle_limits(-PI, PI)
    );
    
    // --- SLING ---------------------------

    let sling_len = arm_dim.z * 0.8;
    let element_count = 16;
    let element_dim = Vec3::new(0.1, 0.1, sling_len / element_count as f32);
    let anchor_element = Vec3::Z * element_dim.z / 2.;
    let element_mesh = meshes.add(Cuboid::from_size(element_dim));
    let mut pos = -anchor_arm - anchor_element; 
    let mut prev_element_id = arm_id;

    for i in 0 .. element_count {
        let element_id = cmd.spawn((
            MaterialMeshBundle {
                transform: Transform::from_translation(pos),
                material: mh.clone(),
                mesh: element_mesh.clone(),
                ..default()
            },
            // RigidBody::Static,
            RigidBody::Dynamic,
            MassPropertiesBundle::new_computed(&Collider::cuboid(element_dim.x, element_dim.y, element_dim.z), 10.),
        )).id();

        cmd.spawn(
            RevoluteJoint::new(prev_element_id, element_id)
            .with_local_anchor_1(if i == 0 {-anchor_arm} else {-anchor_element})
            .with_local_anchor_2(anchor_element)
            .with_aligned_axis(Vec3::X)
            .with_compliance(0.000000)
        );
        prev_element_id  = element_id;
        pos += -2. * anchor_element;
    }
    
    

    // --- ENDING --------------------------------------------------------------------
    
    let ending_radius = 0.1;
    let ending_id = cmd.spawn((
        MaterialMeshBundle {
            transform: Transform::from_translation(pos - anchor_element - Vec3::Z * ending_radius),
            mesh: meshes.add(Sphere::new(ending_radius)),
            material: mh.clone(),
            ..default()
        }, 
        RigidBody::Dynamic,
        Restitution::new(0.).with_combine_rule(CoefficientCombine::Min),
        Friction::new(0.).with_combine_rule(CoefficientCombine::Min),
        Collider::sphere(ending_radius),
        SlingEnd
    ))
    .id()
    ;
    cmd.spawn((
        RevoluteJoint::new(prev_element_id, ending_id)
        .with_aligned_axis(Vec3::X)
        .with_local_anchor_1(-anchor_element)
        .with_compliance(0.),
    ));

}

// ---

fn key_control(
    keys: Res<ButtonInput<KeyCode>>,
    mut next: ResMut<NextState<PState>>
) {

    if keys.just_pressed(KeyCode::ArrowDown) {
        next.set(PState::Tension);
    }
    if keys.just_pressed(KeyCode::Space) {
        next.set(PState::Loose);
    }

}

// ---

fn enter_tension(
    mut se_q: Query<&mut ExternalForce, With<SlingEnd>>,
    mut pivot_q: Query<&mut RevoluteJoint, With<Pivot>>
) {
    se_q.get_single_mut().unwrap().set_force(Vec3::Z * 5.);
    pivot_q.get_single_mut().unwrap().damping_angular = 1000.0;

}

// ---

fn do_tension(
    q_end: Query<&GlobalTransform, With<ArmLongEnd>>,
    q_bar: Query<&Transform, With<Bar>>,
    mut next: ResMut<NextState<PState>>,
    mut arm_q: Query<(&mut ExternalForce, &Transform), With<Arm>>,
) {
    let et = q_end.single();
    let bt = q_bar.single();

    if et.translation().y - 1. < bt.translation.y {
        next.set(PState::Arming);
    } else {
        let (mut f, t) = arm_q.single_mut();
        f.set_force(-t.up() * 10000.);    
    } 
}

// ---

fn exit_tension(
    mut arm_q: Query<&mut ExternalForce, With<Arm>>,
    mut se_q: Query<(&mut ExternalForce, &mut RigidBody), (With<SlingEnd>, Without<Arm>)>,
    mut pivot_q: Query<&mut RevoluteJoint, With<Pivot>>,
) {
    if let Ok(mut arm_ef) = arm_q.get_single_mut() {
        arm_ef.set_force(Vec3::ZERO);
    }
    if let Ok((mut se_ef, mut se_rb)) = se_q.get_single_mut() {
        se_ef.set_force(Vec3::ZERO);
        *se_rb = RigidBody::Static;
    }
    if let Ok(mut pivot_rj) =  pivot_q.get_single_mut() {
        pivot_rj.damping_angular = 0.1;
    }
}   

// ---

fn enter_arming(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ball_radius = 0.5;
    cmd.spawn((
        MaterialMeshBundle {
            transform: Transform::from_translation(Vec3::Z * 15.),
            mesh: meshes.add(Sphere::new(ball_radius)),
            material: materials.add(Color::WHITE),
            ..default()
        }, 
        RigidBody::Dynamic,
        Restitution::new(0.).with_combine_rule(CoefficientCombine::Min),
        Friction::new(0.).with_combine_rule(CoefficientCombine::Min),
        Collider::sphere(ball_radius),
        ExternalForce::new(-Vec3::Z * 0.5),
        Ball
    ));
}

// ---

fn do_arming(
    ball_q: Query<(Entity, &Transform), With<Ball>>,
    se_q: Query<(Entity, &Transform), (Without<Ball>, With<SlingEnd>)>,
    rj_q: Query<Entity, With<BallLink>>,
    mut cmd: Commands
) {

    if !rj_q.is_empty() {
        return;
    }
    
    let Ok((ball_e, ball_t)) =  ball_q.get_single() else {
        return;
    };
    let Ok((se_e, se_t)) =  se_q.get_single() else {
        return;
    };

    if ball_t.translation.distance(se_t.translation) < 1. {
        println!("lock");
        cmd.spawn((
            RevoluteJoint::new(se_e, ball_e)
            .with_aligned_axis(Vec3::X)
            .with_local_anchor_1(-Vec3::Z * 1.)
            .with_compliance(0.),
            BallLink
        ));
    }
}


// ---

fn enter_loose(
    mut ball_q: Query<&mut RigidBody, With<SlingEnd>>,
) {
    *ball_q.single_mut() = RigidBody::Dynamic;
}

// ---

fn do_loose(
    mut cmd: Commands,
    ball_q: Query<&Transform, With<Ball>>,
    link_q: Query<Entity, With<BallLink>>,
    mut gizmos: Gizmos,
    mut next: ResMut<NextState<PState>>,
) {

    let Ok(link_e) = link_q.get_single() else {
        next.set(PState::Idle);
        return;
    };
    let ball_t = ball_q.single();
    let to_ball = (ball_t.translation - Vec3::ZERO).normalize();

    gizmos.ray(Vec3::ZERO, to_ball * 10., Color::srgb(1., 0., 0.));

    let dot = to_ball.dot(Vec3::Y);
    
    if dot > 0.95 {
        cmd.entity(link_e).despawn();
    } 
}

// ---
