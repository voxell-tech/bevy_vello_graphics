use std::marker::PhantomData;

pub use bevy_vello;

use bevy::prelude::*;
use bevy_vello::VelloPlugin;
use head::{draw_heads, prepare_heads, HeadScene};
use prelude::*;
use vector::{draw_vectors, VectorScene};

pub mod bezpath;
pub mod brush;
pub mod circle;
pub mod fill;
pub mod head;
pub mod line;
pub mod rect;
pub mod stroke;
pub mod vector;

pub mod prelude {
    pub use crate::{
        bezpath::VelloBezPath,
        brush::Brush,
        circle::VelloCircle,
        fill::Fill,
        head::{Head, HeadBundle, HeadTransform, HeadVector},
        line::VelloLine,
        rect::VelloRect,
        stroke::Stroke,
        vector::Vector,
        VelloGraphicsPlugin,
    };

    pub use bevy_vello::prelude::*;
}

pub struct VelloGraphicsPlugin;

impl Plugin for VelloGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, (DrawVector, DrawHead, Composite).chain());
        app.configure_sets(Update, (PrepareHead, DrawHead).chain());

        app.add_plugins(VelloPlugin).add_plugins((
            CompositePlugin::<VelloRect>::default(),
            CompositePlugin::<VelloCircle>::default(),
            CompositePlugin::<VelloLine>::default(),
            CompositePlugin::<VelloBezPath>::default(),
        ));
    }
}

/// Plugin for compositing vector that implements [`Vector`] and [`VectorBorder`].
#[derive(Default)]
pub(crate) struct CompositePlugin<V: Vector + Component>(PhantomData<V>);

impl<V: Vector + Component> Plugin for CompositePlugin<V>
where
    V: Default,
{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_vectors::<V>.in_set(DrawVector))
            .add_systems(Update, draw_heads::<V>.in_set(DrawHead))
            .add_systems(Update, prepare_heads::<V>.in_set(PrepareHead))
            .add_systems(Update, composite);
    }
}

#[allow(clippy::type_complexity)]
fn composite(
    mut commands: Commands,
    q_scenes: Query<
        (Entity, &VectorScene, Option<&HeadScene>),
        Or<(Changed<VectorScene>, Changed<HeadScene>)>,
    >,
) {
    for (entity, vector_scene, head_scene) in q_scenes.iter() {
        let mut scene = vector_scene.0.clone();

        if let Some(head_scene) = head_scene {
            scene.append(&head_scene.0, None);
        }

        commands.entity(entity).insert(VelloScene::from(scene));
    }
}

#[derive(SystemSet, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct DrawVector;

#[derive(SystemSet, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct PrepareHead;

#[derive(SystemSet, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct DrawHead;

/// Composite
#[derive(SystemSet, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Composite;
