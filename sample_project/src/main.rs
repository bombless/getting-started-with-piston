extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

use std::cell::RefCell;
use opengl_graphics::{
    GlGraphics,
    OpenGL,
};
use sdl2_window::Sdl2Window;

fn main() {
    let opengl = OpenGL::_3_2;
    let window = Sdl2Window::new(
        opengl,
        piston::window::WindowSettings {
            title: "Hello Piston".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    let ref mut gl = GlGraphics::new(opengl);
    let window = RefCell::new(window);
    for e in piston::events(&window) {
        use piston::event::*;

        if let Some(args) = e.render_args() {
            use graphics::*;

            gl.draw([0, 0, args.width as i32, args.height as i32], |c, gl| {
                clear([1.0; 4], gl);
                rectangle([1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 100.0, 100.0], c.transform, gl);
            });
        };
    }
}