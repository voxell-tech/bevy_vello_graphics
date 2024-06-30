use bevy::{math::DVec2, prelude::*, utils::Uuid};

use bevy_vello::prelude::*;

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ShapeId(Uuid);
impl ShapeId {
    pub fn get(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for ShapeId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

#[derive(Component, Copy, Clone, Debug)]
pub struct Head {
    pub shape_id: ShapeId,
    pub time: f64,

    pub scale: f64,
    pub offset: DVec2,
    pub rotation_offset: f64,
}

impl Default for Head {
    fn default() -> Self {
        Self {
            shape_id: ShapeId(Uuid::new_v4()),
            time: 1.0,
            scale: 1.0,
            offset: DVec2::default(),
            rotation_offset: 0.0,
        }
    }
}

impl Head {
    pub fn new(shape_id: ShapeId, scale: f64, offset: DVec2, rotation_offset: f64) -> Self {
        Self {
            shape_id,
            scale,
            offset,
            rotation_offset,
            ..default()
        }
    }

    pub fn with_shape_id(mut self, shape_id: ShapeId) -> Self {
        self.shape_id = shape_id;
        self
    }

    pub fn with_scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_offset(mut self, offset: DVec2) -> Self {
        self.offset = offset;
        self
    }

    pub fn with_offset_splat(mut self, offset: f64) -> Self {
        self.offset = DVec2::splat(offset);
        self
    }

    pub fn with_rotation_offset(mut self, rotation_offset: f64) -> Self {
        self.rotation_offset = rotation_offset;
        self
    }
}

#[derive(Resource, Default)]
pub struct Shapes {
    pub scenes: std::collections::HashMap<ShapeId, vello::Scene>,
}

impl Shapes {
    pub fn insert(&mut self, scene: vello::Scene) -> ShapeId {
        let shape_id = ShapeId(Uuid::new_v4());
        self.scenes.insert(shape_id, scene);
        shape_id
    }
}

pub trait VectorBorder {
    /// Translation of the of the border at a specific `time` value.
    fn border_translation(&self, time: f64) -> DVec2;
    /// The gradient of the tangent to the border at a specific `time` value.
    fn border_tangent(&self, time: f64) -> f64;
}
