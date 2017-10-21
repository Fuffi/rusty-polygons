extern crate ggez;
use ggez::conf;
use ggez::event;
use ggez::event::{MouseButton};
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, Mesh, Point};
use std::time::Duration;

enum DrawingState {
    Idle,
    Drawing(Vec<Point>),
}

struct MainState {
    state: DrawingState,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let result = Mesh::new_polygon(ctx, DrawMode::Fill, &[Point { x: -50.0, y: -50.0 }, Point { x: 50.0, y: 0.0 }, Point { x: -50.0, y: 50.0 }], 100.0);
        match result {
            Ok(mesh) => Ok(MainState { state: DrawingState::Idle }),
            Err(err) => Err(err),
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        use ggez::graphics::Drawable;
        graphics::clear(ctx);
        match self.state {
            DrawingState::Idle => {
                ()
            },
            DrawingState::Drawing(ref vertices) => {
                if vertices.len() > 2 {
                    graphics::polygon(ctx, DrawMode::Fill, vertices);
                }
                ()
            },
        }
        graphics::present(ctx);
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _button: MouseButton, x: i32, y: i32) {
        self.state = match self.state {
            DrawingState::Idle => {
                let point = Point { x: x as f32, y: y as f32 };
                DrawingState::Drawing([point].to_vec())
            },
            DrawingState::Drawing(ref mut vertices) => {
                vertices.push(Point { x: x as f32, y: y as f32 });
                DrawingState::Drawing(vertices.to_vec())
            },
        }
    }

}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
