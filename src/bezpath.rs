//! A Bevy friendly wrapper around [`kurbo::BezPath`] with tracing capability.

use bevy_ecs::prelude::*;
use bevy_math::DVec2;
use bevy_utils::prelude::*;
use bevy_vello::vello::kurbo;

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
                        path.push(kurbo::PathEl::MoveTo(*p));

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
        let trace_index = i64::clamp(trace_raw as i64, 0, seg_count as i64 - 1) as usize;
        let seg_index = trace_index + 1;

        if let Some(segment) = self.path.get_seg(seg_index) {
            let t = trace_raw - trace_index as f64;
            return match segment {
                kurbo::PathSeg::Line(line) => point_to_vec(kurbo::Point::lerp(line.p0, line.p1, t)),
                kurbo::PathSeg::Quad(kurbo::QuadBez { p0, p1, p2 }) => {
                    point_to_vec(lerp_quad_point(p0, p1, p2, t))
                }
                kurbo::PathSeg::Cubic(kurbo::CubicBez { p0, p1, p2, p3 }) => {
                    point_to_vec(lerp_cubic_point(p0, p1, p2, p3, t))
                }
            };
        }

        // All else fails..
        fallback
    }

    fn border_rotation(&self, time: f64) -> f64 {
        let pathels = self.path.elements();
        let pathel_count = pathels.len();

        let fallback = 0.0;

        // Guarantee to have at least 2 path elements
        if pathel_count < 2 {
            return fallback;
        }

        let seg_count = pathel_count - 1;
        let trace_raw = time * seg_count as f64;
        let trace_index = i64::clamp(trace_raw as i64, 0, seg_count as i64 - 1) as usize;
        let seg_index = trace_index + 1;

        if let Some(segment) = self.path.get_seg(seg_index) {
            let t = trace_raw - trace_index as f64;
            return match segment {
                kurbo::PathSeg::Line(line) => (line.p1 - line.p0).angle(),
                kurbo::PathSeg::Quad(kurbo::QuadBez { p0, p1, p2 }) => {
                    // kurbo::Point between p0 and p1
                    let x0 = kurbo::Point::lerp(p0, p1, t);
                    // kurbo::Point between p1 and p2
                    let x1 = kurbo::Point::lerp(p1, p2, t);
                    (x1.y - x0.y).atan2(x1.x - x0.x)
                }
                kurbo::PathSeg::Cubic(kurbo::CubicBez { p0, p1, p2, p3 }) => {
                    // point_to_vec(lerp_cubic_point(cubic.p0, cubic.p1, cubic.p2, cubic.p3, t))
                    // kurbo::Point between p0 and p1
                    let x0 = kurbo::Point::lerp(p0, p1, t);
                    // kurbo::Point between p1 and p2
                    let x1 = kurbo::Point::lerp(p1, p2, t);
                    // kurbo::Point between p2 and p3
                    let x2 = kurbo::Point::lerp(p2, p3, t);
                    // kurbo::Point between x0 and x1
                    let y0 = kurbo::Point::lerp(x0, x1, t);
                    // kurbo::Point between x1 and x2
                    let y1 = kurbo::Point::lerp(x1, x2, t);

                    (y1.y - y0.y).atan2(y1.x - y0.x)
                }
            };
        }

        // All else fails..
        fallback
    }
}

/// Interpolate [`kurbo::PathEl`].
fn interp_pathel(p0: kurbo::Point, pathel: kurbo::PathEl, t: f64) -> kurbo::PathEl {
    if t == 1.0 {
        return pathel;
    }

    match pathel {
        kurbo::PathEl::MoveTo(p1) => kurbo::PathEl::MoveTo(kurbo::Point::lerp(p0, p1, t)),
        kurbo::PathEl::LineTo(p1) => kurbo::PathEl::LineTo(kurbo::Point::lerp(p0, p1, t)),
        kurbo::PathEl::QuadTo(p1, p2) => lerp_quad_pathel(p0, p1, p2, t),
        kurbo::PathEl::CurveTo(p1, p2, p3) => lerp_cubic_pathel(p0, p1, p2, p3, t),
        kurbo::PathEl::ClosePath => kurbo::PathEl::ClosePath,
    }
}

fn lerp_quad_pathel(p0: kurbo::Point, p1: kurbo::Point, p2: kurbo::Point, t: f64) -> kurbo::PathEl {
    // kurbo::Point between p0 and p1
    let x0 = kurbo::Point::lerp(p0, p1, t);
    // kurbo::Point between p1 and p2
    let x1 = kurbo::Point::lerp(p1, p2, t);
    // kurbo::Point on curve
    let end_p = kurbo::Point::lerp(x0, x1, t);

    kurbo::PathEl::QuadTo(x0, end_p)
}

fn lerp_quad_point(p0: kurbo::Point, p1: kurbo::Point, p2: kurbo::Point, t: f64) -> kurbo::Point {
    // kurbo::Point between p0 and p1
    let x0 = kurbo::Point::lerp(p0, p1, t);
    // kurbo::Point between p1 and p2
    let x1 = kurbo::Point::lerp(p1, p2, t);
    // kurbo::Point on curve
    kurbo::Point::lerp(x0, x1, t)
}

fn lerp_cubic_pathel(
    p0: kurbo::Point,
    p1: kurbo::Point,
    p2: kurbo::Point,
    p3: kurbo::Point,
    t: f64,
) -> kurbo::PathEl {
    // kurbo::Point between p0 and p1
    let x0 = kurbo::Point::lerp(p0, p1, t);
    // kurbo::Point between p1 and p2
    let x1 = kurbo::Point::lerp(p1, p2, t);
    // kurbo::Point between p2 and p3
    let x2 = kurbo::Point::lerp(p2, p3, t);
    // kurbo::Point between x0 and x1
    let y0 = kurbo::Point::lerp(x0, x1, t);
    // kurbo::Point between x1 and x2
    let y1 = kurbo::Point::lerp(x1, x2, t);
    // kurbo::Point on curve
    let end_p = kurbo::Point::lerp(y0, y1, t);

    kurbo::PathEl::CurveTo(x0, y0, end_p)
}

fn lerp_cubic_point(
    p0: kurbo::Point,
    p1: kurbo::Point,
    p2: kurbo::Point,
    p3: kurbo::Point,
    t: f64,
) -> kurbo::Point {
    // kurbo::Point between p0 and p1
    let x0 = kurbo::Point::lerp(p0, p1, t);
    // kurbo::Point between p1 and p2
    let x1 = kurbo::Point::lerp(p1, p2, t);
    // kurbo::Point between p2 and p3
    let x2 = kurbo::Point::lerp(p2, p3, t);
    // kurbo::Point between x0 and x1
    let y0 = kurbo::Point::lerp(x0, x1, t);
    // kurbo::Point between x1 and x2
    let y1 = kurbo::Point::lerp(x1, x2, t);
    // kurbo::Point on curve
    kurbo::Point::lerp(y0, y1, t)
}

fn point_to_vec(point: kurbo::Point) -> DVec2 {
    DVec2::new(point.x, point.y)
}
