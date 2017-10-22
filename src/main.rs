extern crate ggez;
use ggez::conf;
use ggez::event;
use ggez::event::{MouseButton};
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::graphics::{DrawMode, Point};
use std::time::Duration;

enum DrawingState {
    Idle,
    Drawing(Vec<Point>),
}

struct MainState {
    state: DrawingState,
    canvas: Canvas,
}

struct Canvas {
    meshes: Vec<Vec<Point>>,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let canvas = Canvas { meshes: [].to_vec() };
        Ok(MainState { state: DrawingState::Idle, canvas: canvas })
    }

    fn left_click(&mut self, x: i32, y: i32) {
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

    fn right_click(&mut self, _x: i32, _y: i32) {
        self.state = match self.state {
            DrawingState::Drawing(ref mesh) => {
                self.canvas.meshes.push(mesh.clone());
                DrawingState::Idle
            },
            _ => DrawingState::Idle
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        match self.state {
            DrawingState::Idle => {
                ()
            },
            DrawingState::Drawing(ref vertices) => {
                if vertices.len() > 2 {
                    graphics::polygon(ctx, DrawMode::Fill, vertices).unwrap();
                }
                ()
            },
        }
        for vertices in &self.canvas.meshes {
            graphics::polygon(ctx, DrawMode::Fill, &vertices).unwrap();
        }
        graphics::present(ctx);
        Ok(())
    }

    fn mouse_button_down_event(&mut self, button: MouseButton, x: i32, y: i32) {
        match button {
            MouseButton::Left => self.left_click(x, y),
            MouseButton::Right => self.right_click(x, y),
            _ => (),
        }
    }

}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "Zeneixe", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
