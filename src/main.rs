extern crate sdl2;
extern crate sdl2_ttf;

use std::path::Path;

pub mod ring;
pub mod paddle;
use paddle::*;
pub mod ball;
use ball::*;

static MS_PER_FRAME : u32 = 16;
const WIDTH : i32 = 640;
const HEIGHT : i32 = 480;

fn main() {
  let mut sdl_context = sdl2::init().video().unwrap();
  sdl2_ttf::init();

  let window = sdl_context.window("Pong", 640, 480)
    .position_centered()
    .build()
    .unwrap();

  let mut renderer = window.renderer().build().unwrap();

  let mut running = true;

  let mut left_paddle = Paddle::new(Side::Left);
  let mut right_paddle = Paddle::new(Side::Right);
  let mut ball = Ball::new();

  let mut previous_frame_length = 0;

  let font = sdl2_ttf::Font::from_file(Path::new("OpenSans-Regular.ttf"), 20).unwrap();

  let mut last_eleven_ticks = ring::Ring::new(11);

  while running {
    let frame_start = sdl2::timer::get_ticks();
    last_eleven_ticks.push(frame_start);
    for event in sdl_context.event_pump().poll_iter() {
      use sdl2::event::Event;
      use sdl2::keycode::KeyCode;

      match event {
        Event::Quit {..} | Event::KeyDown { keycode: KeyCode::Escape, .. } => running = false,
        _ => {}
      }
    }

    {
      use sdl2::scancode::ScanCode;

      let keystates = sdl2::keyboard::get_keyboard_state();
      if keystates[&ScanCode::S] || keystates[&ScanCode::D] {
        left_paddle.move_down();
      }
      if keystates[&ScanCode::W] || keystates[&ScanCode::E]  {
        left_paddle.move_up();
      }
      if keystates[&ScanCode::K] || keystates[&ScanCode::Down] {
        right_paddle.move_down();
      }
      if keystates[&ScanCode::I] || keystates[&ScanCode::Up]  {
        right_paddle.move_up();
      }
    }

    let ticks_for_last_ten_frames = last_eleven_ticks.head() - last_eleven_ticks.tail();
    let fps_over_last_ten_frames = 10000 / ticks_for_last_ten_frames;

    let fps_surface = font.render_str_solid(&format!("{} fps", fps_over_last_ten_frames), sdl2::pixels::Color::RGB(0xff, 0xff, 0xff)).unwrap();
    let fps_texture = renderer.create_texture_from_surface(&fps_surface).unwrap();

    let time_surface = font.render_str_solid(&format!("{} ms", previous_frame_length), sdl2::pixels::Color::RGB(0xff, 0xff, 0xff)).unwrap();
    let time_texture = renderer.create_texture_from_surface(&time_surface).unwrap();

    let mut drawer = renderer.drawer();

    drawer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    drawer.clear();

    drawer.set_draw_color(sdl2::pixels::Color::RGB(0xff, 0xff, 0xff));
    drawer.fill_rect(left_paddle.to_sdl());
    drawer.fill_rect(right_paddle.to_sdl());

    drawer.copy(&fps_texture, None, Some(sdl2::rect::Rect{x: 10, y: 10, w: fps_surface.get_width() as i32, h: time_surface.get_height() as i32}));
    drawer.copy(&time_texture, None, Some(sdl2::rect::Rect{x: 10, y: 25, w: time_surface.get_width() as i32, h: time_surface.get_height() as i32}));

    drawer.present();

    let frame_end = sdl2::timer::get_ticks();
    let frame_length = frame_end - frame_start;
    previous_frame_length = frame_length;
    if frame_length < MS_PER_FRAME {
      sdl2::timer::delay(MS_PER_FRAME - frame_length);
    }
  }

  sdl2_ttf::quit();
}
