extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

pub struct Game {
    gl: GlGraphics,
    snake : Snake,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        use graphics;

        let GREEN = [0.0, 1.0, 0.0, 1.0];

        self.gl
            .draw(arg.viewport(), |_c, gl| graphics::clear(GREEN, gl));

        self.snake.render(&mut self.gl, arg);
    }
}

struct Snake{
    pos_x:i32, 
    pos_y:i32,
}

impl Snake {
    fn render(&self, gl:&mut GlGraphics, args:&RenderArgs){
        use graphics;
        let RED = [1.0,0.0,0.0,1.0];

        let square = graphics::rectangle::square(self.pos_x as f64, self.pos_y as f64, 20_f64);

        gl.draw(args.viewport(), |c, gl|{
            let transform = c.transform;

            graphics::rectangle(RED, square, transform, gl);
        })
    }
}



fn main() {
    let opengl = OpenGL::V4_5;

    let mut window: Window = WindowSettings::new("spinning-square", (200, 200))
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = Game {
        gl: GlGraphics::new(opengl), 
        snake : Snake { pos_x: 50, pos_y: 100 }
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        // if let Some(args) = e.update_args() {
        //     app.update(&args);
        // }
    }
}
