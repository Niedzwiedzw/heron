#![deny(future_incompatible, nonstandard_style)]
#![warn(missing_docs, rust_2018_idioms, clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]

//! Rendering of Heron's collision shapes for debugging purposes

use bevy::prelude::*;
use fnv::FnvHashMap;

#[cfg(feature = "2d")]
mod dim2;

/// Plugin that enables rendering of collision shapes
#[derive(Debug, Copy, Clone)]
pub struct DebugPlugin(Color);

#[derive(Debug, Copy, Clone)]
struct DebugColor(Color);

type DebugEntityMap = FnvHashMap<Entity, Entity>;

#[allow(unused)]
struct HasDebug;

#[allow(unused)]
struct IsDebug(Entity);

#[allow(unused)]
struct Indexed;

impl From<Color> for DebugPlugin {
    fn from(color: Color) -> Self {
        Self(color)
    }
}

impl Default for DebugPlugin {
    fn default() -> Self {
        let mut color = bevy::render::color::Color::BLUE;
        color.set_a(0.4);
        Self(color)
    }
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        #[cfg(feature = "2d")]
        app.add_plugin(bevy_prototype_lyon::plugin::ShapePlugin)
            .add_system_set_to_stage(CoreStage::PostUpdate, dim2::systems());

        app.insert_resource(DebugColor(self.0))
            .init_resource::<DebugEntityMap>()
            .add_system_to_stage(CoreStage::Last, track_debug_entities.system())
            .add_system_to_stage(CoreStage::Last, scale_debug_entities.system());
    }
}

impl From<Color> for DebugColor {
    fn from(color: Color) -> Self {
        Self(color)
    }
}

impl From<DebugColor> for Color {
    fn from(DebugColor(color): DebugColor) -> Self {
        color
    }
}

fn track_debug_entities(
    mut commands: Commands<'_>,
    mut map: ResMut<'_, DebugEntityMap>,
    query: Query<'_, (Entity, &IsDebug), Without<Indexed>>,
) {
    for (debug_entity, IsDebug(parent_entity)) in query.iter() {
        map.insert(*parent_entity, debug_entity);
        commands.entity(debug_entity).insert(Indexed);
    }
}

fn scale_debug_entities(mut query: Query<'_, (Option<&mut Transform>, &mut GlobalTransform)>) {
    query
        .iter_mut()
        .filter(|(_, global)| {
            let scale = global.scale;
            !is_near(scale.x, 1.0) || !is_near(scale.y, 1.0)
        })
        .for_each(|(local, mut global)| {
            if let Some(mut local) = local {
                local.scale *= global.scale.recip()
            }
            global.scale.x = 1.0;
            global.scale.y = 1.0;
            global.scale.z = 1.0;
        });
}

#[inline]
fn is_near(v1: f32, v2: f32) -> bool {
    (v2 - v1).abs() <= f32::EPSILON
}
