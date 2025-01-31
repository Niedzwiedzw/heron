#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::{CollisionShape, PhysicMaterial, PhysicsSteps, RigidBody};
use heron_rapier::rapier::geometry::ColliderSet;
use heron_rapier::RapierPlugin;
use std::time::Duration;

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
fn restitution_can_be_defined_when_creating_body() {
    let mut app = test_app();

    let restitution = 0.42;
    let entity = app
        .world
        .spawn()
        .insert_bundle((
            GlobalTransform::default(),
            RigidBody::Dynamic,
            CollisionShape::Sphere { radius: 10.0 },
            PhysicMaterial {
                restitution,
                ..Default::default()
            },
        ))
        .id();

    app.update();

    let colliders = app.world.get_resource::<ColliderSet>().unwrap();
    let collider = colliders.get(*app.world.get(entity).unwrap()).unwrap();

    assert_eq!(restitution, collider.restitution)
}

#[test]
fn restitution_can_be_updated() {
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

    let restitution = 2.0;
    app.world.entity_mut(entity).insert(PhysicMaterial {
        restitution,
        ..Default::default()
    });

    app.update();

    let colliders = app.world.get_resource::<ColliderSet>().unwrap();
    let collider = colliders.get(*app.world.get(entity).unwrap()).unwrap();

    assert_eq!(restitution, collider.restitution)
}
