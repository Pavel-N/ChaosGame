use app::App;
use equilateral_triangle::EquilateralTriangle;
use normal_point::NormalPoint;
use sdl2::pixels::Color;
use sdl2::rect::Point;
mod app;
mod equilateral_triangle;
mod normal_point;

const BACKGROUND: Color = Color::RGB(33, 33, 33);
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const FONT_PATH: &str = "/usr/share/fonts/TTF/arial.ttf";

pub fn main() -> Result<(), String> {
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut app = App::new("Chaos Game", &ttf_context)?;

    let triangle = EquilateralTriangle::new((0.0, 0.0), 1.6);
    let mut half_point: NormalPoint = (0.0, 0.0).into(); // Starting point inside the triangle
    let mut points_count = 0;

    app.run(
        &mut |canvas, _| triangle.draw(canvas),
        &mut |canvas, font| {
            // Point Counter
            App::draw_text(
                canvas,
                font,
                format!("Points: {}", points_count).as_str(),
                10,
                10,
            )?;

            // Limit to 5000 points
            if points_count < 5000 {
                // Choose random vertex
                let vertex = triangle.random_vertex();
                // Half point of line between vertex and previous half point
                half_point = (
                    (vertex.x + half_point.x) / 2.0,
                    (vertex.y + half_point.y) / 2.0,
                )
                    .into();
                // Draw calculated point
                canvas.draw_point::<Point>(half_point.into())?;

                points_count += 1;
            }

            Ok(())
        },
    )?;

    Ok(())
}
