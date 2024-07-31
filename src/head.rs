use bevy::{math::DVec2, prelude::*};
use bevy_vello::vello::kurbo;

use crate::Vector;

#[derive(Bundle, Copy, Clone, Debug)]
pub struct HeadBundle<V: Vector>
where
    V: Send + Sync + 'static,
{
    pub vector: HeadVector<V>,
    pub head: Head,
    pub transform: HeadTransform,
}

impl<V: Vector> HeadBundle<V>
where
    V: Send + Sync + 'static,
{
    pub fn new(vector: V) -> Self {
        Self {
            vector: HeadVector(vector),
            head: Head::default(),
            transform: HeadTransform::default(),
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct HeadVector<V: Vector>(pub V);

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct HeadTransform(pub kurbo::Affine);

#[derive(Component, Debug, Clone, Copy)]
pub struct Head {
    /// Percentage position of the shape's border.
    pub time: f64,
    /// Scale of the head.
    pub scale: f64,
    /// Translation offset from the tangent of the shape.
    pub translation_offset: DVec2,
    /// Rotational offset from the tangent of the shape.
    pub rotation_offset: f64,
}

impl Default for Head {
    fn default() -> Self {
        Self {
            time: 1.0,
            scale: 1.0,
            translation_offset: DVec2::default(),
            rotation_offset: 0.0,
        }
    }
}

impl Head {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_time(mut self, time: f64) -> Self {
        self.time = time;
        self
    }

    pub fn with_scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_offset(mut self, offset: DVec2) -> Self {
        self.translation_offset = offset;
        self
    }

    pub fn with_offset_splat(mut self, offset: f64) -> Self {
        self.translation_offset = DVec2::splat(offset);
        self
    }

    pub fn with_rotation_offset(mut self, rotation_offset: f64) -> Self {
        self.rotation_offset = rotation_offset;
        self
    }
}
