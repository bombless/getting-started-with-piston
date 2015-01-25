extern crate piston;

fn main() {
  piston::start(
    piston::shader_version::OpenGL::_3_2,
    piston::WindowSettings {
      title: "Hello Piston".to_string(),
      samples: 0,
      size: [300, 300],
      fullscreen: false,
      exit_on_esc: true
    },
    || start()
  );
}

fn start() {
  for e in piston::events() {
    use piston::event::RenderEvent;
    use piston::graphics::Rectangle;

    e.render(|_| {
      piston::render_2d_opengl(
        Some(piston::graphics::color::WHITE), |c, g| {
          Rectangle::new([1.0, 0.0, 0.0, 1.0])
            .draw([0.0, 0.0, 100.0, 100.0], &c, g);
      });
    });
  }
}