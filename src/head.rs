use bevy::{math::DVec2, prelude::*, utils::Uuid};

use bevy_vello::prelude::*;

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ShapeId(Uuid);

#[derive(Component, Default, Copy, Clone)]
pub struct Head {
    pub shape_id: ShapeId,
    pub time: f64,

    pub scale: f64,
    pub offset: f32,
    pub rotation_offset: f32,
}

// impl Head {
//     pub fn new()
// }

#[derive(Resource, Default)]
pub struct Shapes {
    pub scenes: std::collections::HashMap<ShapeId, &'static mut vello::Scene>,
}

pub trait VectorBorder {
    /// Translation of the of the border at a specific `time` value.
    fn border_translation(&self, time: f64) -> DVec2;
    /// The gradient of the tangent to the border at a specific `time` value.
    fn border_tangent(&self, time: f64) -> f64;
}
