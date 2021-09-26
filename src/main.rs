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


fn main() {
    
}