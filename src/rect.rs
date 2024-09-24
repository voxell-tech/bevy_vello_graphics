//! A Bevy friendly wrapper around [`kurbo::RoundedRect`].

use bevy::math::DVec2;
use bevy::prelude::*;
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

    fn border_translation(&self, mut time: f64) -> DVec2 {
        // Loop around the rect
        if time > 0.0 {
            time %= 1.0;
        } else {
            time = 1.0 - (time.abs() % 1.0);
        }

        let scaled_time = time * 4.0;
        let time_int = scaled_time as u64;
        let t = scaled_time % 1.0;

        match time_int {
            0 => DVec2::new(f64::lerp(self.x0(), self.x1(), t), self.y1()),
            1 => DVec2::new(self.x1(), f64::lerp(self.y1(), self.y0(), t)),
            2 => DVec2::new(f64::lerp(self.x1(), self.x0(), t), self.y0()),
            3.. => DVec2::new(self.x0(), self.y0().lerp(self.y1(), t)),
        }
    }

    fn border_rotation(&self, mut time: f64) -> f64 {
        // Loop around the rect
        if time > 0.0 {
            time %= 1.0;
        } else {
            time = 1.0 - (time.abs() % 1.0);
        }

        let scaled_time = time * 4.0;
        let time_int = scaled_time as u64;

        match time_int {
            0 => 0.0,
            1 => 3.0 * std::f64::consts::FRAC_PI_2,
            2 => std::f64::consts::PI,
            3.. => std::f64::consts::FRAC_PI_2,
        }
    }
}
