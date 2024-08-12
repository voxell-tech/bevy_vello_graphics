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

    fn border_translation(&self, time: f64) -> DVec2 {
        let theta = time * std::f64::consts::TAU;
        DVec2::new(f64::sin(theta), f64::cos(theta)) * self.radius
    }

    fn border_rotation(&self, time: f64) -> f64 {
        time * -std::f64::consts::TAU
    }
}
