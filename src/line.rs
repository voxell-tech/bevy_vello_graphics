use bevy::{math::DVec2, prelude::*};
use bevy_vello::prelude::*;

use crate::VectorBorder;

use super::Vector;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct VelloLine {
    pub p0: DVec2,
    pub p1: DVec2,
}

impl VelloLine {
    pub fn new(p0: DVec2, p1: DVec2) -> Self {
        Self::default().with_p0(p0).with_p1(p1)
    }

    pub fn with_p0(mut self, p0: DVec2) -> Self {
        self.p0 = p0;
        self
    }

    pub fn with_p1(mut self, p1: DVec2) -> Self {
        self.p1 = p1;
        self
    }

    pub fn extend(mut self, extension: f64) -> Self {
        let dir = DVec2::normalize_or_zero(self.p1 - self.p0);
        self.p0 -= dir * extension;
        self.p1 += dir * extension;
        self
    }
}

impl Vector for VelloLine {
    fn shape(&self) -> impl kurbo::Shape {
        kurbo::Line::new(
            kurbo::Point::new(self.p0.x, self.p0.y),
            kurbo::Point::new(self.p1.x, self.p1.y),
        )
    }
}

impl VectorBorder for VelloLine {
    fn border_translation(&self, time: f64) -> DVec2 {
        println!("{}, {}", self.p0, self.p1);
        self.p0.lerp(self.p1, time)
    }

    fn border_tangent(&self, _time: f64) -> f64 {
        DVec2::normalize_or_zero(self.p1 - self.p0).to_angle()
    }
}
