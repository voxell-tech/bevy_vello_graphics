use bevy::{math::DVec2, prelude::*, utils::Uuid};

use bevy_vello::prelude::*;

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ShapeId(Uuid);

#[derive(Component, Default, Copy, Clone)]
pub struct Head {
    pub shape_id: ShapeId,

    pub scale: f32,
    pub offset: f32,
    pub rotation_offset: f32,
}

#[derive(Resource)]
pub struct Shapes {
    pub scenes: std::collections::HashMap<ShapeId, &'static mut vello::Scene>,
}

pub trait VectorBorder {
    /// translation of the chosen "apex"
    fn border_translation(&self, time: f32) -> DVec2;
    /// gradient of the tangent to the border
    fn border_tangent(&self, time: f32) -> f64;
}
