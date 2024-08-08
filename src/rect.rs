//! A Bevy friendly wrapper around [`kurbo::RoundedRect`].

use bevy_ecs::prelude::*;
use bevy_math::{DVec2, FloatExt};
use bevy_vello::vello::kurbo;

use super::Vector;

/// Vello rect component.
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct VelloRect {
    /// Width and height.
    pub size: DVec2,
    /// Origin of the rect.
    pub anchor: DVec2,
    /// Border radius.
    pub radius: f64,
}

impl VelloRect {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            size: DVec2::new(width, height),
            anchor: DVec2::splat(0.5),
            radius: 0.0,
        }
    }

    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.size = DVec2::new(width, height);
        self
    }

    pub fn with_anchor(mut self, x: f64, y: f64) -> Self {
        self.anchor = DVec2::new(x, y);
        self
    }

    pub fn with_radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }

    #[inline]
    fn x0(&self) -> f64 {
        -self.size.x * self.anchor.x
    }

    #[inline]
    fn y0(&self) -> f64 {
        -self.size.y * self.anchor.y
    }

    #[inline]
    fn x1(&self) -> f64 {
        self.size.x * (1.0 - self.anchor.x)
    }

    #[inline]
    fn y1(&self) -> f64 {
        self.size.y * (1.0 - self.anchor.y)
    }
}

impl Vector for VelloRect {
    fn shape(&self) -> impl kurbo::Shape {
        kurbo::RoundedRect::new(self.x0(), self.y0(), self.x1(), self.y1(), self.radius)
    }

    fn border_translation(&self, time: f64) -> DVec2 {
        let t = time * 4.0;
        let time = t.ceil() as u64;
        let t = t % 1.0;

        if time > 3 {
            DVec2::new(self.x0(), self.y0().lerp(self.y1(), t))
        } else if time > 2 {
            DVec2::new(self.x1().lerp(self.x0(), t), self.y0())
        } else if time > 1 {
            DVec2::new(self.x1(), self.y1().lerp(self.y0(), t))
        } else {
            DVec2::new(self.x0().lerp(self.x1(), t), self.y1())
        }
    }

    fn border_rotation(&self, time: f64) -> f64 {
        let diff =
            self.border_translation(time).abs() - self.border_translation((4.0 * time).floor() / 4.0);

        if diff.y > 0.0 {
            std::f64::consts::FRAC_PI_2
        } else if diff.y < 0.0 {
            3.0 * std::f64::consts::FRAC_PI_2
        } else if diff.x < 0.0 {
            std::f64::consts::PI
        } else {
            0.0
        }
    }
}
