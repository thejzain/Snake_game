extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

use std::collections::LinkedList;

#[derive(Clone, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

pub struct Game {
    gl: GlGraphics,
    snake: Snake,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        let GREEN = [0.0, 1.0, 0.0, 1.0];

        self.gl
            .draw(arg.viewport(), |_c, gl| graphics::clear(GREEN, gl));

        self.snake.render(&mut self.gl, arg);
    }

    fn update(&mut self) {
        self.snake.update()
    }

    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.dir.clone();

        self.snake.dir = match btn {
            &Button::Keyboard(Key::Up) if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down) if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left) if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right) if last_direction != Direction::Left => Direction::Right,

            _ => last_direction,
        }
    }
}

struct Snake {
    body: LinkedList<(i32, i32)>,
    dir: Direction,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let RED = [1.0, 0.0, 0.0, 1.0];

        let square: Vec<graphics::types::Rectangle> = self
            .body
            .iter()
            .map(|&(x, y)| graphics::rectangle::square((x * 20) as f64, (y * 20) as f64, 20_f64))
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            square
                .into_iter()
                .for_each(|square| graphics::rectangle(RED, square, transform, gl));
        })
    }

    fn update(&mut self) {
        let mut newhead = (*self.body.front().expect("Snake has no body")).clone();

        match self.dir {
            Direction::Down => newhead.1 += 1,
            Direction::Left => newhead.0 -= 1,
            Direction::Right => newhead.0 += 1,
            Direction::Up => newhead.1 -= 1,
        }

        self.body.push_front(newhead);
        self.body.pop_back().unwrap();
    }
}

fn main() {
    let opengl = OpenGL::V4_5;

    let mut window: Window = WindowSettings::new("Snake", (500, 500))
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            body: LinkedList::from_iter((vec![(0, 0), (0, 1)]).into_iter()),
            dir: Direction::Right,
        },
    };

    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(_u) = e.update_args() {
            app.update();
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                app.pressed(&k.button);
            }
        }
    }
}
