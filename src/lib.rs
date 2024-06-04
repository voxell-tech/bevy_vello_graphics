pub use bevy_vello;

use arrow::ArrowHead;
use bevy::{ecs::schedule::SystemConfigs, prelude::*};
use bevy_vello::{prelude::*, VelloPlugin};
use bezpath::VelloBezPath;
use circle::VelloCircle;
use fill::Fill;
use line::VelloLine;
use rect::VelloRect;
use stroke::Stroke;

pub mod arrow;
pub mod bezpath;
pub mod brush;
pub mod circle;
pub mod fill;
pub mod line;
pub mod rect;
pub mod stroke;

pub mod prelude {
    pub use crate::VelloGraphicsPlugin;
    pub use crate::{
        arrow::ArrowHead, bezpath::VelloBezPath, circle::VelloCircle, line::VelloLine,
        rect::VelloRect,
    };
    pub use crate::{brush::Brush, fill::Fill, stroke::Stroke};
    pub use bevy_vello::prelude::*;
}

pub struct VelloGraphicsPlugin;

impl Plugin for VelloGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VelloPlugin).add_systems(
            Update,
            (
                build_vector::<VelloRect>(),
                build_vector::<VelloCircle>(),
                build_vector::<VelloLine>(),
                build_vector::<VelloBezPath>(),
            ),
        );
    }
}

pub trait VelloVector {
    fn shape(&self) -> impl kurbo::Shape;

    #[inline]
    fn build_fill(&self, fill: &Fill, scene: &mut vello::Scene) {
        scene.fill(
            fill.style,
            default(),
            &fill.brush.value,
            Some(fill.brush.transform),
            &self.shape(),
        );
    }

    #[inline]
    fn build_stroke(&self, stroke: &Stroke, scene: &mut vello::Scene) {
        scene.stroke(
            &stroke.style,
            default(),
            &stroke.brush.value,
            Some(stroke.brush.transform),
            &self.shape(),
        );
    }
}

pub(crate) fn build_vector<Vector: VelloVector + Component>() -> SystemConfigs {
    (
        build_fill_only_vector::<Vector>,
        build_stroke_only_vector::<Vector>,
        build_fill_and_stroke_vector::<Vector>,
    )
        .into_configs()
}

#[allow(clippy::type_complexity)]
fn build_fill_only_vector<Vector: VelloVector + Component>(
    mut q_vectors: Query<
        (&Vector, &Fill, Option<&ArrowHead>, &mut VelloScene),
        (
            Without<Stroke>,
            Or<(Changed<Vector>, Changed<Fill>, Changed<ArrowHead>)>,
        ),
    >,
) {
    for (vector, fill, arrow, mut scene) in q_vectors.iter_mut() {
        *scene = VelloScene::default();

        // Build the vector to the VelloScene
        vector.build_fill(fill, &mut scene);

        // Build the possible arrow
        if let Some(arrow) = arrow {
            arrow.build_fill(fill, &mut scene);
        }
    }
}

#[allow(clippy::type_complexity)]
fn build_stroke_only_vector<Vector: VelloVector + Component>(
    mut q_vectors: Query<
        (&Vector, &Stroke, Option<&ArrowHead>, &mut VelloScene),
        (
            Without<Fill>,
            Or<(Changed<Vector>, Changed<Stroke>)>,
            Changed<ArrowHead>,
        ),
    >,
) {
    for (vector, stroke, arrow, mut scene) in q_vectors.iter_mut() {
        *scene = VelloScene::default();

        // Build the vector to the VelloScene
        vector.build_stroke(stroke, &mut scene);

        // Build the possible arrow
        if let Some(arrow) = arrow {
            arrow.build_stroke(stroke, &mut scene);
        }
    }
}

#[allow(clippy::type_complexity)]
fn build_fill_and_stroke_vector<Vector: VelloVector + Component>(
    mut q_vectors: Query<
        (&Vector, &Fill, &Stroke, Option<&ArrowHead>, &mut VelloScene),
        Or<(
            Changed<Vector>,
            Changed<Fill>,
            Changed<Stroke>,
            Changed<ArrowHead>,
        )>,
    >,
) {
    for (vector, fill, stroke, arrow, mut scene) in q_vectors.iter_mut() {
        *scene = VelloScene::default();

        // Build the vector to the VelloScene
        vector.build_fill(fill, &mut scene);
        vector.build_stroke(stroke, &mut scene);

        // Build the possible arrow
        if let Some(arrow) = arrow {
            arrow.build_fill(fill, &mut scene);
            arrow.build_stroke(stroke, &mut scene);
        }
    }
}
