use sdl2::{pixels::Color, ttf::Font};

use crate::{BACKGROUND, HEIGHT, WIDTH};

type Canvas = sdl2::render::Canvas<sdl2::video::Window>;

pub struct App<'a> {
    pub canvas: Canvas,
    pub event_pump: sdl2::EventPump,
    font: Font<'a, 'a>,
    should_close: bool,
}

impl<'a> App<'a> {
    pub fn new(
        name: &'a str,
        ttf_context: &'a sdl2::ttf::Sdl2TtfContext,
    ) -> Result<App<'a>, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window(name, WIDTH, HEIGHT)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let event_pump = sdl_context.event_pump()?;

        let font = ttf_context.load_font("/usr/share/fonts/TTF/arial.ttf", 40)?;

        Ok(App {
            canvas,
            event_pump,
            font,
            should_close: false,
        })
    }

    fn handle_events(&mut self) {
        use sdl2::{event::Event, keyboard::Keycode};
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.should_close = true,
                _ => (),
            }
        }
    }

    pub fn draw_text(
        canvas: &mut Canvas,
        font: &Font,
        text: &str,
        x: i32,
        y: i32,
    ) -> Result<(), String> {
        let pr = font
            .render(text)
            .solid(Color::WHITE)
            .map_err(|e| e.to_string())?;

        let bounds = sdl2::rect::Rect::new(x, y, pr.width(), pr.height());
        let draw_color = canvas.draw_color();
        canvas.set_draw_color(BACKGROUND);
        canvas.fill_rect(bounds)?;
        canvas.set_draw_color(draw_color);
        canvas.copy(
            &pr.as_texture(&canvas.texture_creator())
                .map_err(|e| e.to_string())?,
            pr.rect(),
            bounds,
        )?;

        Ok(())
    }

    pub fn run(
        &mut self,
        once: &mut dyn FnMut(&mut Canvas, &Font) -> Result<(), String>,
        repeat: &mut dyn FnMut(&mut Canvas, &Font) -> Result<(), String>,
    ) -> Result<(), String> {
        self.canvas.set_draw_color(BACKGROUND);
        self.canvas.clear();

        self.canvas.set_draw_color(Color::WHITE);
        once(&mut self.canvas, &self.font)?;

        while !self.should_close {
            // Handle events
            self.handle_events();

            // Run user commands
            repeat(&mut self.canvas, &self.font)?;

            // Update
            self.canvas.present();
            std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 200));
        }

        Ok(())
    }
}
