use bevy::{math::DVec2, prelude::*};
use bevy_vello::{prelude::*, vello::kurbo::Shape};

use super::VelloVector;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct VelloArrow {
    pub head: ArrowHead,

    // parent
    pub position: kurbo::Vec2,
    pub angle: f64, // radians

    // settings
    pub offset: kurbo::Vec2,
    pub rotation: f64, // radians
    pub size: f32,
}

impl VelloArrow {
    pub fn absolute(
        head: ArrowHead,
        position: kurbo::Vec2,
        angle: f64,
        offset: kurbo::Vec2,
        rotation: f64,
        size: f32,
    ) -> Self {
        Self {
            head,
            position,
            angle,
            offset,
            rotation,
            size,
        }
    }

    pub fn new(head: ArrowHead, offset: kurbo::Vec2, rotation: f64, size: f32) -> Self {
        Self {
            head,
            offset,
            rotation,
            size,
            ..default()
        }
    }

    pub fn new_simple(head: ArrowHead, size: f32) -> Self {
        Self {
            head,
            size,
            ..default()
        }
    }

    pub fn with_head(mut self, head: ArrowHead) -> Self {
        self.head = head;
        self
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn with_offset(mut self, offset: kurbo::Vec2) -> Self {
        self.offset = offset;
        self
    }

    pub fn with_rotation(mut self, angle: f64) -> Self {
        self.rotation = angle;
        self
    }

    /// Attach [`VelloArrow`] to `shape` with its position and rotation
    pub fn attach<Shape: kurbo::Shape>(&mut self, shape: Shape) {
        let r#box = shape.bounding_box();

        let endpoint = DVec2::new(r#box.x1, r#box.y1);
        self.position = kurbo::Vec2::new(endpoint.x, endpoint.y);
        self.rotation = endpoint.angle_between(DVec2::new(r#box.x0, r#box.y0));
    }
}

#[derive(Component, Default, Debug, Clone, Copy)]
pub enum ArrowHead {
    #[default]
    Triangle,
    Square,
    Circle,
}

impl VelloVector for VelloArrow {
    fn shape(&self) -> impl kurbo::Shape {
        let center = (self.position + self.offset).to_point();
        // example
        match self.head {
            // TODO: wait for https://github.com/linebender/kurbo/pull/350
            ArrowHead::Triangle => unreachable!(),

            ArrowHead::Square => {
                kurbo::Rect::from_center_size(center, (self.size as f64, self.size as f64))
                    .to_path(0.1)
            }

            ArrowHead::Circle => kurbo::Circle::new(center, self.size as f64).to_path(0.1),
        }
    }
}
