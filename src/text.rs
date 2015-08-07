extern crate sdl2;
extern crate sdl2_ttf;

use std::path::Path;

pub struct Text {
  font: sdl2_ttf::Font,
}

impl Text {
  pub fn new(font_name: &str, size: i32) -> Text {
    let font = sdl2_ttf::Font::from_file(Path::new(font_name), size).unwrap();
    Text{font: font}
  }

  pub fn render(&self, renderer: &mut sdl2::render::Renderer, text: &str, x: i32, y: i32) {
    let surface = self.font.render_str_solid(&text, sdl2::pixels::Color::RGB(0xff, 0xff, 0xff)).unwrap();
    let texture = renderer.create_texture_from_surface(&surface).unwrap();
    renderer.copy(&texture, None, Some(sdl2::rect::Rect::new_unwrap(x, y, surface.get_width(), surface.get_height())));
  }
}
