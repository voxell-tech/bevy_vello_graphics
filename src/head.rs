//! Drawing [`HeadVector`] on the border of [`Vector`] shapes.

use bevy::math::DVec2;
use bevy::prelude::*;
use bevy_vello::vello::{self, kurbo};

use crate::{Fill, SceneHolder, Stroke, Vector};

/// Prepare [`HeadTransform`]s for drawing [`HeadVector`]s on the border of [`Vector`] shapes.
#[allow(clippy::type_complexity)]
pub(super) fn prepare_heads<V: Vector + Component>(
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

/// Draw [`HeadVector`] shapes.
#[allow(clippy::type_complexity)]
pub(super) fn draw_heads<V: Vector + Component>(
    mut commands: Commands,
    q_vectors: Query<
        (
            Entity,
            &HeadVector<V>,
            &HeadTransform,
            Option<&HeadFill>,
            Option<&HeadStroke>,
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
                fill.0.style,
                head_transform.0,
                &fill.0.brush.value,
                Some(fill.0.brush.transform),
                &head_vector.0.shape(),
            );
        }

        if let Some(stroke) = stroke {
            scene.stroke(
                &stroke.0.style,
                head_transform.0,
                &stroke.0.brush.value,
                Some(stroke.0.brush.transform),
                &head_vector.0.shape(),
            );
        }

        commands
            .entity(entity)
            .insert(SceneHolder::<HeadScene>::new(scene));
    }
}

/// Marker struct of a vector scene for [`SceneHolder`].
pub struct HeadScene;

/// Bundle of components needed for drawing a [`HeadVector`] on the border of a [`Vector`] shape.
#[derive(Bundle, Debug, Copy, Clone)]
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

/// Vector defining the shape of the head.
#[derive(Component, Debug, Clone, Copy)]
pub struct HeadVector<V: Vector>(pub V);

/// Fill of a [`HeadVector`].
#[derive(Component, Debug, Clone)]
pub struct HeadFill(pub Fill);

/// Stroke of a [`HeadVector`].
#[derive(Component, Default, Debug, Clone)]
pub struct HeadStroke(pub Stroke);

/// Positioning configurations of a head.
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

/// A read-only computed [`Head`] transform for drawing [`HeadVector`] on top of [`Vector`].
///
/// The transform is computed in the [`PrepareHead`][PrepareHead] system set.
///
/// [PrepareHead]: crate::PrepareHead
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct HeadTransform(kurbo::Affine);

impl HeadTransform {
    pub fn affine(&self) -> kurbo::Affine {
        self.0
    }
}
