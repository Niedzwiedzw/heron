#![cfg(feature = "2d")]

use std::time::Duration;

use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::{CollisionShape, PhysicsSteps, RigidBody, RotationConstraints};
use heron_rapier::rapier::dynamics::RigidBodySet;
use heron_rapier::RapierPlugin;

fn test_app() -> App {
    let mut builder = App::build();
    builder
        .init_resource::<TypeRegistryArc>()
        .insert_resource(PhysicsSteps::every_frame(Duration::from_secs(1)))
        .add_plugin(CorePlugin)
        .add_plugin(RapierPlugin);
    builder.app
}

#[test]
fn rotation_is_not_constrained_without_the_component() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            GlobalTransform::default(),
            RigidBody::Dynamic,
            CollisionShape::Sphere { radius: 10.0 },
        ))
        .id();

    app.update();

    let bodies = app.world.get_resource::<RigidBodySet>().unwrap();

    assert!(
        bodies
            .get(*app.world.get(entity).unwrap())
            .unwrap()
            .effective_world_inv_inertia_sqrt
            > 0.0
    );
}

#[test]
fn rotation_can_be_locked_at_creation() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            GlobalTransform::default(),
            RigidBody::Dynamic,
            CollisionShape::Sphere { radius: 10.0 },
            RotationConstraints::lock(),
        ))
        .id();

    app.update();

    let bodies = app.world.get_resource::<RigidBodySet>().unwrap();

    assert_eq!(
        bodies
            .get(*app.world.get(entity).unwrap())
            .unwrap()
            .effective_world_inv_inertia_sqrt,
        0.0
    );
}

#[test]
fn rotation_can_be_locked_after_creation() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            GlobalTransform::default(),
            RigidBody::Dynamic,
            CollisionShape::Sphere { radius: 10.0 },
        ))
        .id();

    app.update();

    app.world
        .entity_mut(entity)
        .insert(RotationConstraints::lock());

    app.update();

    let bodies = app.world.get_resource::<RigidBodySet>().unwrap();

    assert_eq!(
        bodies
            .get(*app.world.get(entity).unwrap())
            .unwrap()
            .effective_world_inv_inertia_sqrt,
        0.0
    );
}

#[test]
fn rotation_is_unlocked_if_component_is_removed() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            GlobalTransform::default(),
            RigidBody::Dynamic,
            CollisionShape::Sphere { radius: 10.0 },
            RotationConstraints::lock(),
        ))
        .id();

    app.update();

    app.world.entity_mut(entity).remove::<RotationConstraints>();

    app.update();

    let bodies = app.world.get_resource::<RigidBodySet>().unwrap();

    assert!(
        bodies
            .get(*app.world.get(entity).unwrap())
            .unwrap()
            .effective_world_inv_inertia_sqrt
            > 0.0
    );
}
