//! Drawing [`Vector`] shapes.

use bevy_ecs::prelude::*;
use bevy_math::DVec2;
use bevy_vello::vello::{
    self,
    kurbo::{self, BezPath, Shape},
};

use crate::{Fill, SceneHolder, Stroke, VelloBezPath};

/// Draw [`Vector`] shapes.
#[allow(clippy::type_complexity)]
pub(super) fn draw_vectors<V: Vector + Component>(
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

        commands
            .entity(entity)
            .insert(SceneHolder::<VectorScene>::new(scene));
    }
}

/// Marker struct of a vector scene for [`SceneHolder`].
pub struct VectorScene;

pub trait Vector {
    /// Returns vector graphics that implements [`kurbo::Shape`].
    fn shape(&self) -> impl kurbo::Shape;
    /// Translation of the border at a specific `time` value.
    fn border_translation(&self, time: f64) -> DVec2 {
        let path = BezPath::from_iter(self.shape().path_elements(0.0));

        VelloBezPath::default()
            .with_path(path)
            .border_translation(time)
    }
    /// The rotation at the tangent of the border at a specific `time` value.
    fn border_rotation(&self, time: f64) -> f64 {
        // TODO: really should rotate based on a pair of segments like in `border_translation` but its already kinda gross to compute that once so we need like a cache or somethjing (impl in bez path tho)
        self.border_translation(time).to_angle()
    }
}
