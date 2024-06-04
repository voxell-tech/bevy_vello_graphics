use bevy::prelude::*;
use bevy_vello::{prelude::*, vello::kurbo::Shape};

use super::VelloVector;

#[derive(Component, Default, Debug, Clone, Copy)]
pub enum ArrowHead {
    #[default]
    Triangle,
    Square,
    Circle,
}

impl VelloVector for ArrowHead {
    fn shape(&self) -> impl kurbo::Shape {
        // exmaple
        match self {
            // TODO: wait for https://github.com/linebender/kurbo/pull/350
            Self::Triangle => kurbo::Rect::new(0.0, 0.0, 1.0, 1.0).to_path(0.1),
            Self::Square => kurbo::Rect::new(0.0, 0.0, 1.0, 1.0).to_path(0.1),
            Self::Circle => kurbo::Circle::new((0.0, 0.0), 1.0).to_path(0.1),
        }
    }
}
