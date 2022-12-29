use sdl2::rect::Point;

use crate::{HEIGHT, WIDTH};

#[derive(Copy, Clone)]
pub struct NormalPoint {
    pub x: f32,
    pub y: f32,
}

impl From<(f32, f32)> for NormalPoint {
    fn from(value: (f32, f32)) -> Self {
        NormalPoint {
            x: value.0,
            y: value.1,
        }
    }
}

impl Into<Point> for NormalPoint {
    fn into(self) -> Point {
        Point::new(
            ((WIDTH as f32) / 2.0 * self.x) as i32 + (WIDTH as f32 / 2.0) as i32 - 1,
            ((HEIGHT as f32) / -2.0 * self.y) as i32 + (HEIGHT as f32 / 2.0) as i32 - 1,
        )
    }
}
