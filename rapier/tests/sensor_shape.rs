#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

use std::time::Duration;

use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::{CollisionShape, PhysicsSteps, RigidBody, SensorShape};
use heron_rapier::rapier::dynamics::IntegrationParameters;
use heron_rapier::rapier::geometry::ColliderSet;
use heron_rapier::RapierPlugin;

fn test_app() -> App {
    let mut builder = App::build();
    let mut parameters = IntegrationParameters::default();
    parameters.dt = 1.0;

    builder
        .init_resource::<TypeRegistryArc>()
        .insert_resource(PhysicsSteps::every_frame(Duration::from_secs(1)))
        .add_plugin(CorePlugin)
        .add_plugin(RapierPlugin);
    builder.app
}

#[test]
fn a_non_sensor_body_can_have_a_sensor_shape() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            GlobalTransform::default(),
            RigidBody::Dynamic,
            CollisionShape::Sphere { radius: 1.0 },
            SensorShape,
        ))
        .id();

    app.update();

    let collider = app
        .world
        .get_resource::<ColliderSet>()
        .unwrap()
        .get(*app.world.get(entity).unwrap())
        .unwrap();

    assert!(collider.is_sensor());
}

#[test]
fn sensor_flag_can_be_added_after_creation() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            GlobalTransform::default(),
            RigidBody::Dynamic,
            CollisionShape::Sphere { radius: 1.0 },
        ))
        .id();

    app.update();

    app.world.entity_mut(entity).insert(SensorShape);

    app.update();

    let collider = app
        .world
        .get_resource::<ColliderSet>()
        .unwrap()
        .get(*app.world.get(entity).unwrap())
        .unwrap();

    assert!(collider.is_sensor());
}

#[test]
fn sensor_flag_can_removed() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            GlobalTransform::default(),
            RigidBody::Dynamic,
            CollisionShape::Sphere { radius: 1.0 },
            SensorShape,
        ))
        .id();

    app.update();

    app.world.entity_mut(entity).remove::<SensorShape>();

    app.update();

    let collider = app
        .world
        .get_resource::<ColliderSet>()
        .unwrap()
        .get(*app.world.get(entity).unwrap())
        .unwrap();

    assert!(!collider.is_sensor());
}

#[test]
fn removing_sensor_flag_has_no_effect_if_body_is_sensor() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            GlobalTransform::default(),
            RigidBody::Sensor,
            CollisionShape::Sphere { radius: 1.0 },
            SensorShape,
        ))
        .id();

    app.update();

    app.world.entity_mut(entity).remove::<SensorShape>();

    app.update();

    let collider = app
        .world
        .get_resource::<ColliderSet>()
        .unwrap()
        .get(*app.world.get(entity).unwrap())
        .unwrap();

    assert!(collider.is_sensor());
}
