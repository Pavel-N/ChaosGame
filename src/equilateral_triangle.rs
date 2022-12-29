use crate::normal_point::NormalPoint;
use sdl2::{render::Canvas, video::Window};

pub struct EquilateralTriangle {
    a: NormalPoint,
    b: NormalPoint,
    c: NormalPoint,
}
impl EquilateralTriangle {
    pub fn new(center: impl Into<NormalPoint>, side_length: f32) -> Self {
        let c = center.into();
        let h = (side_length.powi(2) - (side_length / 2.0).powi(2)).sqrt();

        EquilateralTriangle {
            b: (c.x + side_length / 2.0, c.y - h / 2.0).into(),
            a: (c.x - side_length / 2.0, c.y - h / 2.0).into(),
            c: (c.x, c.y + h / 2.0).into(),
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.draw_lines(&vec![self.a.into(), self.b.into(), self.c.into(), self.a.into()][..])?;

        Ok(())
    }

    pub fn random_vertex(&self) -> NormalPoint {
        vec![self.a, self.b, self.c][rand::Rng::gen_range(&mut rand::thread_rng(), 0..=2)]
    }
}
