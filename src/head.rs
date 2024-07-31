use std::marker::PhantomData;

use bevy::{math::DVec2, prelude::*};
use bevy_vello::vello::{self, kurbo};

use crate::{DrawHead, Fill, PrepareHead, Stroke, Vector, VectorBorder};

/// Prepare head transform for drawing head on top of vector shape.
#[derive(Default)]
pub(super) struct PrepareHeadPlugin<V: VectorBorder + Component>(PhantomData<V>);

impl<V: VectorBorder + Component> Plugin for PrepareHeadPlugin<V> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, prepare_heads::<V>.in_set(PrepareHead));
    }
}

/// Draw head vector shape.
#[derive(Default)]
pub(super) struct HeadPlugin<V: Vector + Component>(PhantomData<V>);

impl<V: Vector + Component> Plugin for HeadPlugin<V> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_heads::<V>.in_set(DrawHead));
    }
}

#[allow(clippy::type_complexity)]
fn prepare_heads<V: VectorBorder + Component>(
    mut q_vectors: Query<(&V, &Head, &mut HeadTransform), Or<(Changed<V>, Changed<Head>)>>,
) {
    for (vector, head, mut head_transform) in q_vectors.iter_mut() {
        let translation = vector.border_translation(head.time) + head.translation_offset;
        let rotation = vector.border_rotation(head.time) + head.rotation_offset;
        let scale = head.scale;

        head_transform.0 = kurbo::Affine::rotate(rotation)
            .then_scale(scale)
            .then_translate(kurbo::Vec2::new(translation.x, translation.y));
    }
}

#[allow(clippy::type_complexity)]
fn draw_heads<V: Vector + Component>(
    mut commands: Commands,
    q_vectors: Query<
        (
            Entity,
            &HeadVector<V>,
            &HeadTransform,
            Option<&Fill>,
            Option<&Stroke>,
        ),
        Or<(
            Changed<HeadVector<V>>,
            Changed<HeadTransform>,
            Changed<Fill>,
            Changed<Stroke>,
        )>,
    >,
) {
    for (entity, head_vector, head_transform, fill, stroke) in q_vectors.iter() {
        let mut scene = vello::Scene::new();

        if let Some(fill) = fill {
            scene.fill(
                fill.style,
                head_transform.0,
                &fill.brush.value,
                Some(fill.brush.transform),
                &head_vector.0.shape(),
            );
        }

        if let Some(stroke) = stroke {
            scene.stroke(
                &stroke.style,
                head_transform.0,
                &stroke.brush.value,
                Some(stroke.brush.transform),
                &head_vector.0.shape(),
            );
        }

        commands.entity(entity).insert(HeadScene(scene));
    }
}

#[derive(Component, Default, Clone)]
pub struct HeadScene(pub vello::Scene);

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
