use std::marker::PhantomData;

pub use bevy_vello;

use bevy::prelude::*;
use bevy_vello::VelloPlugin;
use head::{HeadPlugin, HeadScene, PrepareHeadPlugin};
use prelude::*;
use vector::{VectorPlugin, VectorScene};

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
        vector::{Vector, VectorBorder},
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
pub(crate) struct CompositePlugin<V: Vector + VectorBorder + Component>(PhantomData<V>);

impl<V: Vector + VectorBorder + Component> Plugin for CompositePlugin<V>
where
    V: Default,
{
    fn build(&self, app: &mut App) {
        app.add_plugins((
            VectorPlugin::<V>::default(),
            PrepareHeadPlugin::<V>::default(),
            HeadPlugin::<V>::default(),
        ))
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
