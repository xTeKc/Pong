use std::process;
use piston::window::WindowSettings;
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, Key, PressEvent, ReleaseEvent,
RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use glutin_window::GlutinWindow;
use opengl_graphics::{Glgraphics, OpenGL};


pub struct App {
    gl: GlGraphics,
    left_score: i32,
    left_position: i32,
    left_velocity: i32,
    right_score: i32,
    right_position: i32,
    right_velocity: i32,
    ball_x: i32,
    ball_y: i32,
    velocity_x: i32,
    velocity_y: i32,
}


impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [0.0, 0.5, 0.5, 1.0];
        const FOREGROUND: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let left = rectangle::square(0.0, 0.0, 50.0);
        let left_position = self.left_position as f64;
        let right = rectangle::square(0.0, 0.0, 50.0);
        let right_position = self.right_position as f64;

        let ball = rectangle::square(0.0, 0.0, 10.0);
        let ball_x = self.ball_x as f64;
        let ball_y = self.ball_y as f64;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);
            rectangle(FOREGROUND, left, c.transform.trans(-40.0, left_position), gl);
            rectangle(FOREGROUND, right, c.transform.trans(
                args.width as f64 - 10.0, right_position), gl);
            rectangle(FOREGROUND, ball, c.transform.trans(ball_x, ball_y), gl);
        })
    }
}


fn main() {
    
}