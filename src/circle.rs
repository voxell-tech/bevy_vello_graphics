use bevy::{math::DVec2, prelude::*};
use bevy_vello::prelude::*;

use crate::VectorBorder;

use super::VelloVector;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct VelloCircle {
    pub radius: f64,
}

impl VelloCircle {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }

    pub fn with_radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }
}

impl VelloVector for VelloCircle {
    fn shape(&self) -> impl kurbo::Shape {
        kurbo::Circle::new(kurbo::Point::default(), self.radius)
    }
}

impl VectorBorder for VelloCircle {
    fn border_translation(&self, _time: f32) -> DVec2 {
        DVec2::new(0.0, self.radius)
    }

    fn border_tangent(&self, _time: f32) -> f64 {
        0.0
    }
}
