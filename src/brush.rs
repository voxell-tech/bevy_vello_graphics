use bevy_color::Color;
use bevy_utils::prelude::*;
use bevy_vello::prelude::*;

#[derive(Default, Clone)]
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
        let color = color.to_linear();
        Self {
            value: peniko::Brush::Solid(peniko::Color::rgba(
                color.red as f64,
                color.green as f64,
                color.blue as f64,
                color.alpha as f64,
            )),
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
