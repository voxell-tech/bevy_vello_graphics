//! A Bevy friendly wrapper around [`kurbo::Circle`].

use bevy_ecs::prelude::*;
use bevy_math::DVec2;
use bevy_vello::prelude::*;

use super::Vector;

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

impl Vector for VelloCircle {
    fn shape(&self) -> impl kurbo::Shape {
        kurbo::Circle::new(kurbo::Point::default(), self.radius)
    }

    fn border_translation(&self, _time: f64) -> DVec2 {
        DVec2::ZERO
    }

    fn border_rotation(&self, _time: f64) -> f64 {
        0.0
    }
}
