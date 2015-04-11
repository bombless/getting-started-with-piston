extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

use std::cell::RefCell;
use std::rc::Rc;
use opengl_graphics::{
    GlGraphics,
    OpenGL,
};
use sdl2_window::Sdl2Window;
use piston::window::{Size, WindowSettings};
use piston::event::Events;

fn main() {
    let opengl = OpenGL::_3_2;
    let window = Sdl2Window::new(
        opengl,
        WindowSettings::new("Hello Piston".to_string(), Size{ width: 300, height: 300 })
    );

    let ref mut gl = GlGraphics::new(opengl);
    let window = RefCell::new(window);
    for e in Events::events(Rc::new(window)) {
        use piston::event::*;

        if let Some(args) = e.render_args() {
            use graphics::*;
            let viewport = Viewport {
                rect: [0, 0, 300, 300],
                draw_size: [300, 300],
                window_size: [300, 300]
            };
            gl.draw(viewport, |c, gl| {
                clear([1.0; 4], gl);
                rectangle([1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 100.0, 100.0], c.transform, gl);
            });
        };
    }
}
