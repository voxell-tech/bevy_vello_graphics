use std::marker::PhantomData;

use bevy::{math::DVec2, prelude::*};
use bevy_vello::vello::{self, kurbo};

use crate::{DrawVector, Fill, Stroke};

#[derive(Default)]
pub(super) struct VectorPlugin<V: Vector + Component>(PhantomData<V>);

impl<V: Vector + Component> Plugin for VectorPlugin<V> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_vectors::<V>.in_set(DrawVector));
    }
}

#[allow(clippy::type_complexity)]
fn draw_vectors<V: Vector + Component>(
    mut commands: Commands,
    q_vectors: Query<
        (Entity, &V, Option<&Fill>, Option<&Stroke>),
        Or<(Changed<V>, Changed<Fill>, Changed<Stroke>)>,
    >,
) {
    for (entity, vector, fill, stroke) in q_vectors.iter() {
        let mut scene = vello::Scene::new();

        if let Some(fill) = fill {
            scene.fill(
                fill.style,
                kurbo::Affine::IDENTITY,
                &fill.brush.value,
                Some(fill.brush.transform),
                &vector.shape(),
            );
        }

        if let Some(stroke) = stroke {
            scene.stroke(
                &stroke.style,
                kurbo::Affine::IDENTITY,
                &stroke.brush.value,
                Some(stroke.brush.transform),
                &vector.shape(),
            );
        }

        commands.entity(entity).insert(VectorScene(scene));
    }
}

#[derive(Component, Default, Clone)]
pub struct VectorScene(pub vello::Scene);

pub trait Vector {
    fn shape(&self) -> impl kurbo::Shape;
}

pub trait VectorBorder {
    /// Translation of the border at a specific `time` value.
    fn border_translation(&self, time: f64) -> DVec2;
    /// The rotation at the tangent of the border at a specific `time` value.
    fn border_rotation(&self, time: f64) -> f64;
}
