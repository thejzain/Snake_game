extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

use rand::Rng;
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
    apple: Apple,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        let green = [0.0, 1.0, 0.0, 1.0];

        self.gl
            .draw(arg.viewport(), |_c, gl| graphics::clear(green, gl));

        self.snake.render(&mut self.gl, arg);
        self.apple.render(&mut self.gl, arg);
    }

    fn update(&mut self) {
        self.snake.update(&mut self.apple);
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
        let red = [1.0, 0.0, 0.0, 1.0];

        let square: Vec<graphics::types::Rectangle> = self
            .body
            .iter()
            .map(|&(x, y)| graphics::rectangle::square((x * 20) as f64, (y * 20) as f64, 20_f64))
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            square
                .into_iter()
                .for_each(|square| graphics::rectangle(red, square, transform, gl));
        })
    }

    fn update(&mut self, apple: &mut Apple) {
        let mut newhead = (*self.body.front().expect("Snake has no body")).clone();

        match self.dir {
            Direction::Down => newhead.1 += 1,
            Direction::Left => newhead.0 -= 1,
            Direction::Right => newhead.0 += 1,
            Direction::Up => newhead.1 -= 1,
        }

        self.collision(&newhead);
        self.body.push_front(newhead);
        self.body.pop_back().unwrap();

        if apple.x == newhead.0 && apple.y == newhead.1 {
            self.body.push_front(newhead);
            apple.update(&self);
        }
    }

    fn collision(&self, newhead: &(i32, i32)) {
        self.body.iter().for_each(
            |&(x, y)| match newhead {
                n if n.0 == x && n.1 == y => {
                    println!("Collision Game over");
                }
                _ => {}
            },
            //This also works

            // if newhead.0==x && newhead.1==y{
            //     println!("Collision Game over");
            // }

            //why to use pattern matching here instead of if else ? cause pattern matching is working with use of jump command while compiling to mc
            //its faster than if else which is typically implemented with conditional branches
        )
    }
}

struct Apple {
    x: i32,
    y: i32,
}

impl Apple {
    fn new() -> Apple {
        let mut rng = rand::thread_rng();
        Apple {
            x: rng.gen_range(0..20),
            y: rng.gen_range(0..20),
        }
    }
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        const BLACK: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let x = self.x * 20;
        let y = self.y * 20;

        let square = graphics::rectangle::square(x as f64, y as f64, 20 as f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(BLACK, square, transform, gl)
        });
    }
    fn update(&mut self, snake: &Snake) {
        let mut rng = rand::thread_rng();
        self.x = rng.gen_range(0..20);
        self.y = rng.gen_range(0..20);
        snake.body.iter().for_each(|&(x, y)| {
            if self.x == x && self.y == y {
                self.update(&snake);
                // println!("update");
            }
        });
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
        apple: Apple::new(),
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
