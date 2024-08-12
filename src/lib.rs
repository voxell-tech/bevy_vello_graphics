//! # Bevy Vello Graphics
//!
//! Bevy Vello Graphics is a Bevy friendly wrapper around [Vello][vello] graphics.
//!
//! ## Quick Start
//!
//! To use Bevy Vello Graphics, you will first need to add the [`VelloGraphicsPlugin`]:
//!
//! ```no_run
//! use bevy::prelude::*;
//! use bevy_vello_graphics::prelude::*;
//!
//! App::new()
//!     .add_plugins((DefaultPlugins, VelloGraphicsPlugin))
//!     // ...your other plugins, systems, etc.
//!     .run();
//! ```
//!
//! Then, spawn in a 2d camera to render the vello scene:
//!
//! ```
//! use bevy::prelude::*;
//!
//! fn setup(mut commands: Commands) {
//!     commands.spawn(Camera2dBundle::default());
//! }
//! ```
//!
//! Finally, spawn in your vector graphics of choice:
//!
//! ```
//! use bevy::prelude::*;
//! use bevy_vello_graphics::{bevy_vello::prelude::*, prelude::*};
//!
//! fn spawn_rect(mut commands: Commands) {
//!    let rect = (
//!        VelloRect::new(100.0, 200.0),
//!        Fill::new().with_color(Color::WHITE),
//!        Stroke::new(5.0).with_color(Color::BLACK),
//!    );
//!
//!    commands.spawn(VelloSceneBundle::default()).insert(rect);
//! }
//! ```

pub use bevy_vello;

use std::marker::PhantomData;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_vello::prelude::*;

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
        head::{Head, HeadBundle, HeadFill, HeadStroke, HeadTransform, HeadVector},
        line::VelloLine,
        rect::VelloRect,
        stroke::Stroke,
        vector::Vector,
        VelloGraphicsPlugin,
    };
}

/// A plugin that automates the pipeline of drawing and compositing vello shapes.
pub struct VelloGraphicsPlugin;

impl Plugin for VelloGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, ((DrawVector, DrawHead), Composite).chain());
        app.configure_sets(Update, (PrepareHead, DrawHead).chain());

        app.add_plugins(VelloPlugin)
            .add_plugins((
                VectorPlugin::<VelloRect>::default(),
                VectorPlugin::<VelloCircle>::default(),
                VectorPlugin::<VelloLine>::default(),
                VectorPlugin::<VelloBezPath>::default(),
            ))
            .add_systems(Update, composite.in_set(Composite));
    }
}

/// A plugin for drawing [`Vector`].
#[derive(Default)]
pub struct VectorPlugin<V: Vector + Component>(PhantomData<V>);

impl<V: Vector + Component> Plugin for VectorPlugin<V>
where
    V: Default,
{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_vectors::<V>.in_set(DrawVector))
            .add_systems(Update, draw_heads::<V>.in_set(DrawHead))
            .add_systems(Update, prepare_heads::<V>.in_set(PrepareHead));
    }
}

#[allow(clippy::type_complexity)]
fn composite(
    mut commands: Commands,
    q_scenes: Query<
        (
            Entity,
            &SceneHolder<VectorScene>,
            Option<&SceneHolder<HeadScene>>,
        ),
        Or<(
            Changed<SceneHolder<VectorScene>>,
            Changed<SceneHolder<HeadScene>>,
        )>,
    >,
) {
    for (entity, vector_scene, head_scene) in q_scenes.iter() {
        let mut scene = vello::Scene::new();
        scene.append(vector_scene.scene(), None);

        if let Some(head_scene) = head_scene {
            scene.append(head_scene.scene(), None);
        }

        commands.entity(entity).insert(VelloScene::from(scene));
    }
}

/// A read-only holder of [`vello::Scene`].
#[derive(Component, Default, Clone)]
pub struct SceneHolder<T>(vello::Scene, PhantomData<T>);

impl<T> SceneHolder<T> {
    pub fn new(scene: vello::Scene) -> Self {
        Self(scene, PhantomData)
    }

    pub fn scene(&self) -> &vello::Scene {
        &self.0
    }
}

/// System set for drawing vector shape.
#[derive(SystemSet, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct DrawVector;

/// System set for preparing vector shape's head data.
#[derive(SystemSet, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct PrepareHead;

/// System set for drawing vector shape's head.
#[derive(SystemSet, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct DrawHead;

/// System set for compositing all [`SceneHolder`]s into [`VelloScene`].
#[derive(SystemSet, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Composite;
