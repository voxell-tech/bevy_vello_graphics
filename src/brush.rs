//! A Bevy friendly wrapper around [`peniko::Brush`].

use bevy_color::Color;
use bevy_utils::prelude::*;
use bevy_vello::prelude::*;

#[derive(Default, Debug, Clone)]
pub struct Brush {
    pub value: peniko::Brush,
    pub transform: kurbo::Affine,
}

impl Brush {
    pub fn from_brush(brush: peniko::Brush) -> Self {
        Self {
            value: brush,
            ..default()
        }
    }

    pub fn from_color(color: Color) -> Self {
        let color = color.to_srgba();
        Self {
            value: peniko::Brush::Solid(peniko::Color::new([
                color.red,
                color.green,
                color.blue,
                color.alpha,
            ])),
            ..default()
        }
    }

    pub fn from_gradient(gradient: peniko::Gradient) -> Self {
        Self {
            value: peniko::Brush::Gradient(gradient),
            ..default()
        }
    }

    pub fn with_transform(mut self, transform: kurbo::Affine) -> Self {
        self.transform = transform;
        self
    }
}
