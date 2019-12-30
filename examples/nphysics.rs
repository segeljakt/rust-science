use nalgebra::{Isometry2, Point2, Vector2};
use ncollide2d::{
    pipeline::CollisionGroups,
    shape::{Ball, ShapeHandle},
};
use nphysics2d::{
    force_generator::DefaultForceGeneratorSet,
    joint::DefaultJointConstraintSet,
    material::{BasicMaterial, MaterialHandle},
    math::{Force, ForceType, Inertia, Velocity},
    object::{
        Body, BodyPartHandle, BodyStatus, ColliderDesc, DefaultBodySet, DefaultColliderSet,
        RigidBodyDesc,
    },
    world::{DefaultGeometricalWorld, DefaultMechanicalWorld},
};
use rawkey::{KeyCode, RawKey};

const PI: f32 = 3.14;

fn read_input(rawkey: &mut RawKey, pressed: &mut [bool], released: &mut [bool]) {
    for (i, &key) in [
        KeyCode::UpArrow,
        KeyCode::DownArrow,
        KeyCode::LeftArrow,
        KeyCode::RightArrow,
    ]
    .into_iter()
    .enumerate()
    {
        pressed[i] = !pressed[0] && rawkey.is_pressed(key);
        released[i] = !released[0] && !rawkey.is_pressed(key);
    }
}

fn main() {
    let mut mechanical_world = DefaultMechanicalWorld::new(Vector2::new(0.0, -9.81));
    let mut geometrical_world = DefaultGeometricalWorld::new();

    let mut bodies = DefaultBodySet::new();
    let mut colliders = DefaultColliderSet::new();
    let mut joint_constraints = DefaultJointConstraintSet::new();
    let mut force_generators = DefaultForceGeneratorSet::new();

    let body_handle = bodies.insert(
        RigidBodyDesc::new()
            .translation(Vector2::y() * 5.0)
            .rotation(5.0)
            .position(Isometry2::new(Vector2::new(1.0, 2.0), PI))
            .gravity_enabled(false)
            .status(BodyStatus::Kinematic)
            .velocity(Velocity::linear(1.0, 2.0))
            .linear_damping(10.0)
            .angular_damping(5.0)
            .max_linear_velocity(10.0)
            .max_angular_velocity(1.7)
            .angular_inertia(3.0)
            .mass(1.2)
            .local_inertia(Inertia::new(1.0, 3.0))
            .local_center_of_mass(Point2::new(1.0, 2.))
            .sleep_threshold(None)
            .kinematic_translations(Vector2::new(true, false))
            .kinematic_rotations(true)
            .linear_motion_interpolation_enabled(true)
            .user_data(10)
            .build(),
    );

    let collider_handle = colliders.insert(
        ColliderDesc::new(ShapeHandle::new(Ball::new(1.5)))
            .translation(Vector2::y() * 5.0)
            .rotation(5.0)
            .position(Isometry2::new(Vector2::new(1.0, 2.0), PI))
            .density(1.3)
            .material(MaterialHandle::new(BasicMaterial::new(0.3, 0.8)))
            .margin(0.02)
            .collision_groups(
                CollisionGroups::new()
                    .with_membership(&[1, 6])
                    .with_whitelist(&[1, 3, 5]),
            )
            .linear_prediction(0.01)
            .angular_prediction(0.1)
            .sensor(true)
            .user_data(10)
            .build(BodyPartHandle(body_handle, 0)),
    );

    let pressed = &mut [false; 0];
    let released = &mut [false; 0];
    let rawkey = &mut RawKey::new();

    loop {
        read_input(rawkey, pressed, released);
//         if let Some(mut body) = bodies.get_mut(body_handle) {
//             let delta_pos = body.part(0).unwrap().center_of_mass();
//             body.apply_force(
//                 0,
//                 &Force::linear(delta_pos * -10.0),
//                 ForceType::VelocityChange,
//                 false,
//             );
//         }
        // Run the simulation.
        mechanical_world.step(
            &mut geometrical_world,
            &mut bodies,
            &mut colliders,
            &mut joint_constraints,
            &mut force_generators,
        )
    }
}
