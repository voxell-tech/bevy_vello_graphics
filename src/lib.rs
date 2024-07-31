pub use bevy_vello;

use bevy::{ecs::schedule::SystemConfigs, math::DVec2, prelude::*};
use bevy_vello::VelloPlugin;
use prelude::*;

pub mod bezpath;
pub mod brush;
pub mod circle;
pub mod fill;
pub mod head;
pub mod line;
pub mod rect;
pub mod stroke;

pub mod prelude {
    pub use crate::bezpath::VelloBezPath;
    pub use crate::brush::Brush;
    pub use crate::circle::VelloCircle;
    pub use crate::fill::Fill;
    pub use crate::head::{Head, HeadBundle, HeadTransform, HeadVector};
    pub use crate::line::VelloLine;
    pub use crate::rect::VelloRect;
    pub use crate::stroke::Stroke;
    pub use crate::VelloGraphicsPlugin;

    pub use bevy_vello::prelude::*;
}

pub struct VelloGraphicsPlugin;

impl Plugin for VelloGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                ClearVectorScene,
                FillVector,
                StrokeVector,
                FillHead,
                StrokeHead,
            )
                .chain(),
        );
        app.configure_sets(Update, PrepareHead.before(FillHead));

        app.add_plugins(VelloPlugin).add_systems(
            Update,
            (
                build_vector_with_head::<VelloRect>(),
                build_vector_with_head::<VelloCircle>(),
                build_vector_with_head::<VelloLine>(),
                build_vector_with_head::<VelloBezPath>(),
            ),
        );
    }
}

#[derive(SystemSet, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ClearVectorScene;

#[derive(SystemSet, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct FillVector;

#[derive(SystemSet, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct StrokeVector;

#[derive(SystemSet, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct PrepareHead;

#[derive(SystemSet, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct FillHead;

#[derive(SystemSet, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct StrokeHead;

pub fn build_vector_with_head<V: Vector + Component + VectorBorder>() -> SystemConfigs {
    (
        // clear
        clear_vector_scene::<V>.in_set(ClearVectorScene),
        // vector
        fill_vectors::<V>.in_set(FillVector),
        stroke_vectors::<V>.in_set(StrokeVector),
        // prepare head
        prepare_heads::<V>.in_set(PrepareHead),
        // head
        fill_heads::<V>.in_set(FillHead),
        stroke_heads::<V>.in_set(StrokeHead),
    )
        .into_configs()
}

pub fn build_vector_without_head<V: Vector + Component>() -> SystemConfigs {
    (
        // clear
        clear_vector_scene::<V>.in_set(ClearVectorScene),
        // vector
        fill_vectors::<V>.in_set(FillVector),
        stroke_vectors::<V>.in_set(StrokeVector),
    )
        .into_configs()
}

#[allow(clippy::type_complexity)]
fn clear_vector_scene<V: Vector + Component>(
    mut q_scenes: Query<
        &mut VelloScene,
        (
            With<V>,
            Or<(Changed<V>, Changed<Fill>, Changed<Stroke>, Changed<Head>)>,
        ),
    >,
) {
    for mut scene in q_scenes.iter_mut() {
        scene.reset()
    }
}

#[allow(clippy::type_complexity)]
fn fill_vectors<V: Vector + Component>(
    mut q_vectors: Query<(&V, &Fill, &mut VelloScene), Or<(Changed<V>, Changed<Fill>)>>,
) {
    for (vector, fill, mut scene) in q_vectors.iter_mut() {
        scene.fill(
            fill.style,
            kurbo::Affine::IDENTITY,
            &fill.brush.value,
            Some(fill.brush.transform),
            &vector.shape(),
        );
    }
}

#[allow(clippy::type_complexity)]
fn stroke_vectors<V: Vector + Component>(
    mut q_vectors: Query<(&V, &Stroke, &mut VelloScene), Or<(Changed<V>, Changed<Stroke>)>>,
) {
    for (vector, stroke, mut scene) in q_vectors.iter_mut() {
        scene.stroke(
            &stroke.style,
            kurbo::Affine::IDENTITY,
            &stroke.brush.value,
            Some(stroke.brush.transform),
            &vector.shape(),
        );
    }
}

#[allow(clippy::type_complexity)]
fn prepare_heads<V: VectorBorder + Component>(
    mut q_vectors: Query<(&V, &Head, &mut HeadTransform), Or<(Changed<V>, Changed<Head>)>>,
) {
    for (vector, head, mut head_transform) in q_vectors.iter_mut() {
        let translation = vector.border_translation(head.time) + head.translation_offset;
        let rotation = vector.border_tangent(head.time) + head.rotation_offset;
        let scale = head.scale;

        head_transform.0 = kurbo::Affine::rotate(rotation)
            .then_scale(scale)
            .then_translate(kurbo::Vec2::new(translation.x, translation.y));
    }
}

#[allow(clippy::type_complexity)]
fn fill_heads<V: Vector + VectorBorder + Component>(
    mut q_vectors: Query<
        (&HeadVector<V>, &HeadTransform, &Fill, &mut VelloScene),
        Or<(
            Changed<HeadVector<V>>,
            Changed<HeadTransform>,
            Changed<Fill>,
        )>,
    >,
) {
    for (head_vector, head_transform, fill, mut scene) in q_vectors.iter_mut() {
        scene.fill(
            fill.style,
            head_transform.0,
            &fill.brush.value,
            Some(fill.brush.transform),
            &head_vector.0.shape(),
        );
    }
}

#[allow(clippy::type_complexity)]
fn stroke_heads<V: Vector + VectorBorder + Component>(
    mut q_vectors: Query<
        (&HeadVector<V>, &HeadTransform, &Stroke, &mut VelloScene),
        Or<(
            Changed<HeadVector<V>>,
            Changed<HeadTransform>,
            Changed<Stroke>,
        )>,
    >,
) {
    for (head_vector, head_transform, stroke, mut scene) in q_vectors.iter_mut() {
        scene.stroke(
            &stroke.style,
            head_transform.0,
            &stroke.brush.value,
            Some(stroke.brush.transform),
            &head_vector.0.shape(),
        );
    }
}

pub trait Vector {
    fn shape(&self) -> impl kurbo::Shape;
}

pub trait VectorBorder {
    /// Translation of the of the border at a specific `time` value.
    fn border_translation(&self, time: f64) -> DVec2;
    /// The gradient of the tangent to the border at a specific `time` value.
    fn border_tangent(&self, time: f64) -> f64;
}
