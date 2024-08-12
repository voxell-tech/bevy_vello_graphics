//! A Bevy friendly wrapper around [`kurbo::Stroke`].

use bevy_color::Color;
use bevy_ecs::prelude::*;
use bevy_utils::prelude::*;
use bevy_vello::prelude::*;

use crate::brush::Brush;

/// Stroke of a [`Vector`][Vector].
///
/// [Vector]: crate::Vector
#[derive(Component, Default, Clone)]
pub struct Stroke {
    pub style: kurbo::Stroke,
    pub brush: Brush,
}

impl Stroke {
    pub fn new(width: f64) -> Self {
        Self {
            style: kurbo::Stroke::new(width),
            ..default()
        }
    }

    pub fn from_style(style: kurbo::Stroke) -> Self {
        Self { style, ..default() }
    }

    pub fn with_brush(mut self, brush: Brush) -> Self {
        self.brush = brush;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.brush = Brush::from_color(color);
        self
    }

    pub fn with_style(mut self, style: kurbo::Stroke) -> Self {
        self.style = style;
        self
    }
}
