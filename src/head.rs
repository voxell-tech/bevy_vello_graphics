use bevy::{prelude::*, utils::Uuid};

use bevy_vello::prelude::*;

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ShapeId(Uuid);

#[derive(Component, Default, Copy, Clone)]
pub struct Head {
    pub shape_id: ShapeId,

    pub scale: f64,
    pub offset: f32,
    pub rotation_offset: f32,
}

#[derive(Resource)]
pub struct Shapes {
    pub scenes: std::collections::HashMap<ShapeId, &'static mut vello::Scene>,
}

pub trait VectorBorder {
    fn border_translation(&self, time: f32) -> kurbo::Vec2;
    /// returns the gradient of the tangent to the border
    fn border_tangent(&self, time: f32) -> f64;
}
