//! A Bevy friendly wrapper around [`kurbo::BezPath`] with tracing capability.

use bevy_ecs::prelude::*;
use bevy_math::DVec2;
use bevy_utils::prelude::*;
use bevy_vello::{
    prelude::*,
    vello::kurbo::{PathEl, Point},
};

use super::Vector;

/// Vello Bézier path component.
#[derive(Component, Debug, Clone)]
pub struct VelloBezPath {
    /// Bézier path.
    pub path: kurbo::BezPath,
    /// Tracing percentage from the start to the end of the entire Bézier path.
    pub trace: f64,
}

impl VelloBezPath {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_path(mut self, path: kurbo::BezPath) -> Self {
        self.path = path;
        self
    }

    pub fn with_trace(mut self, trace: f64) -> Self {
        self.trace = trace;
        self
    }

    // TODO: need a constant interpolation time `t` thats even accross all `PathEl`s
    /// Gets the progress of/and [`kurbo::PathEl`] which the [`VelloBezPath`] is inbetween at `t`
    /// NOTE: each [`kurbo::PathEl`] will be the same at `t == 1.0`
    fn inbetween(&self, t: f64) -> ((PathEl, PathEl), f64) {
        let elements = self.path.elements();
        let index_f = t.clamp(0.0, 1.0) * (elements.len() - 1) as f64;
        let index = index_f as usize;

        (
            (elements[index], elements[index + 1 - (t == 1.0) as usize]),
            index_f % 1.0,
        )
    }
}

impl Default for VelloBezPath {
    fn default() -> Self {
        Self {
            path: default(),
            trace: 1.0,
        }
    }
}

impl Vector for VelloBezPath {
    fn shape(&self) -> impl kurbo::Shape {
        let pathels = self.path.elements();
        // TODO(perf): Prevent from creating a new BezPath for every animation update.
        let mut path = kurbo::BezPath::new();

        let pathel_count = pathels.len();
        let trace_raw = self.trace * pathel_count as f64;

        let mut most_recent_initial = kurbo::Point::new(0.0, 0.0);
        let mut most_recent_point = kurbo::Point::new(0.0, 0.0);

        for (path_index, pathel) in pathels.iter().enumerate() {
            let mut interp_value = trace_raw - path_index as f64;

            if interp_value > 0.0 {
                // Clamp value within 1.0
                interp_value = f64::min(interp_value, 1.0);

                match pathel {
                    kurbo::PathEl::MoveTo(p) => {
                        path.push(PathEl::MoveTo(*p));

                        most_recent_initial = *p;
                        most_recent_point = *p;
                    }
                    kurbo::PathEl::LineTo(p) => {
                        path.push(interp_pathel(most_recent_point, *pathel, interp_value));

                        most_recent_point = *p;
                    }
                    kurbo::PathEl::QuadTo(_, p) => {
                        path.push(interp_pathel(most_recent_point, *pathel, interp_value));

                        most_recent_point = *p;
                    }
                    kurbo::PathEl::CurveTo(.., p) => {
                        path.push(interp_pathel(most_recent_point, *pathel, interp_value));

                        most_recent_point = *p;
                    }
                    kurbo::PathEl::ClosePath => {
                        if interp_value == 1.0 {
                            path.push(kurbo::PathEl::ClosePath);
                        } else {
                            path.push(interp_pathel(
                                most_recent_point,
                                kurbo::PathEl::MoveTo(most_recent_initial),
                                interp_value,
                            ));
                        }
                    }
                }
            }
        }

        path
    }

    fn border_translation(&self, time: f64) -> DVec2 {
        let pathels = self.path.elements();
        let pathel_count = pathels.len();

        let fallback = pathels
            .first()
            .and_then(|path| path.end_point().map(|point| DVec2::new(point.x, point.y)))
            .unwrap_or_default();

        // Guarantee to have at least 2 path elements
        if pathel_count < 2 {
            return fallback;
        }

        let seg_count = pathel_count - 1;
        let trace_raw = time * seg_count as f64;
        let trace_index = f64::clamp(trace_raw, 0.0, (seg_count - 1) as f64) as usize;
        let seg_index = trace_index + 1;

        if let Some(segment) = self.path.get_seg(seg_index) {
            let t = trace_raw - trace_index as f64;
            return match segment {
                kurbo::PathSeg::Line(line) => point_to_vec(Point::lerp(line.p0, line.p1, t)),
                kurbo::PathSeg::Quad(quad) => {
                    point_to_vec(lerp_quad_point(quad.p0, quad.p1, quad.p2, t))
                }
                kurbo::PathSeg::Cubic(cubic) => {
                    point_to_vec(lerp_cubic_point(cubic.p0, cubic.p1, cubic.p2, cubic.p3, t))
                }
            };
        }

        // All else fails..
        fallback
    }

    fn border_rotation(&self, time: f64) -> f64 {
        return 0.0;
        let (path, t) = self.inbetween(time);

        let current = path.0.end_point().unwrap_or_default();
        match path.1 {
            PathEl::MoveTo(_) => unreachable!(),
            PathEl::ClosePath => (self
                .path
                .elements()
                .first()
                .unwrap()
                .end_point()
                .unwrap()
                .to_vec2()
                - current.to_vec2())
            .angle(),
            PathEl::LineTo(p) => (p.to_vec2() - current.to_vec2()).angle(),
            PathEl::QuadTo(p1, p2) => {
                let a = current.lerp(p1, t);
                let b = p1.lerp(p2, t);
                (b.y - a.y).atan2(b.x - a.x)
            }
            PathEl::CurveTo(p1, p2, p3) => {
                let a = current.lerp(p1, t);
                let b = p1.lerp(p2, t);
                let c = p2.lerp(p3, t);

                let d = a.lerp(b, t);
                let e = b.lerp(c, t);
                (e.y - d.y).atan2(e.x - d.x)
            }
        }
    }
}

/// Interpolate [`kurbo::PathEl`].
fn interp_pathel(p0: Point, pathel: PathEl, t: f64) -> PathEl {
    if t == 1.0 {
        return pathel;
    }

    match pathel {
        PathEl::MoveTo(p1) => PathEl::MoveTo(Point::lerp(p0, p1, t)),
        PathEl::LineTo(p1) => PathEl::LineTo(Point::lerp(p0, p1, t)),
        PathEl::QuadTo(p1, p2) => lerp_quad_pathel(p0, p1, p2, t),
        PathEl::CurveTo(p1, p2, p3) => lerp_cubic_pathel(p0, p1, p2, p3, t),
        PathEl::ClosePath => PathEl::ClosePath,
    }
}

fn lerp_quad_pathel(p0: Point, p1: Point, p2: Point, t: f64) -> PathEl {
    // Point between p0 and p1
    let x0 = Point::lerp(p0, p1, t);
    // Point between p1 and p2
    let x1 = Point::lerp(p1, p2, t);
    // Point on curve
    let end_p = Point::lerp(x0, x1, t);

    PathEl::QuadTo(x0, end_p)
}

fn lerp_quad_point(p0: Point, p1: Point, p2: Point, t: f64) -> Point {
    // Point between p0 and p1
    let x0 = Point::lerp(p0, p1, t);
    // Point between p1 and p2
    let x1 = Point::lerp(p1, p2, t);
    // Point on curve
    Point::lerp(x0, x1, t)
}

fn lerp_cubic_pathel(p0: Point, p1: Point, p2: Point, p3: Point, t: f64) -> PathEl {
    // Point between p0 and p1
    let x0 = Point::lerp(p0, p1, t);
    // Point between p1 and p2
    let x1 = Point::lerp(p1, p2, t);
    // Point between p2 and p3
    let x2 = Point::lerp(p2, p3, t);
    // Point between x0 and x1
    let y0 = Point::lerp(x0, x1, t);
    // Point between x1 and x2
    let y1 = Point::lerp(x1, x2, t);
    // Point on curve
    let end_p = Point::lerp(y0, y1, t);

    PathEl::CurveTo(x0, y0, end_p)
}

fn lerp_cubic_point(p0: Point, p1: Point, p2: Point, p3: Point, t: f64) -> Point {
    // Point between p0 and p1
    let x0 = Point::lerp(p0, p1, t);
    // Point between p1 and p2
    let x1 = Point::lerp(p1, p2, t);
    // Point between p2 and p3
    let x2 = Point::lerp(p2, p3, t);
    // Point between x0 and x1
    let y0 = Point::lerp(x0, x1, t);
    // Point between x1 and x2
    let y1 = Point::lerp(x1, x2, t);
    // Point on curve
    Point::lerp(y0, y1, t)
}

fn point_to_vec(point: Point) -> DVec2 {
    DVec2::new(point.x, point.y)
}
