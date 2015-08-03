extern crate sdl2;
extern crate sdl2_ttf;

use std::path::Path;

pub mod ring;
pub mod fps;
pub mod paddle;
pub mod ball;
use ball::*;

static MS_PER_FRAME : u32 = 16;
const WIDTH : u32 = 640;
const HEIGHT : u32 = 480;

fn main() {
  let mut sdl_context = sdl2::init().video().unwrap();
  sdl2_ttf::init();

  let window = sdl_context.window("Pong", 640, 480)
    .position_centered()
    .build()
    .unwrap();

  let mut renderer = window.renderer().present_vsync().build().unwrap();

  let mut running = true;

  let mut left_paddle = paddle::Paddle::new(paddle::Side::Left);
  let mut right_paddle = paddle::Paddle::new(paddle::Side::Right);

  let mut ball = Ball::new();

  let mut left_score = 0;
  let mut right_score = 0;

  let mut previous_frame_length = 0;

  let font = sdl2_ttf::Font::from_file(Path::new("OpenSans-Regular.ttf"), 20).unwrap();

  let mut fps = fps::FPS::new();

  while running {
    let frame_start = sdl2::timer::get_ticks();
    fps.tick();
    for event in sdl_context.event_pump().poll_iter() {
      use sdl2::event::Event;
      use sdl2::keyboard::Keycode;

      match event {
        Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => running = false,
        Event::KeyDown { keycode: Some(Keycode::R), ..} => {
          left_paddle = paddle::Paddle::new(paddle::Side::Left);
          right_paddle = paddle::Paddle::new(paddle::Side::Right);
          ball = Ball::new();
        }
        _ => {}
      }
    }

    {
      use sdl2::keyboard::Scancode;

      let keystates = sdl_context.keyboard_state();
      if keystates.is_scancode_pressed(Scancode::S) || keystates.is_scancode_pressed(Scancode::D) {
        left_paddle.move_down();
      }
      if keystates.is_scancode_pressed(Scancode::W) || keystates.is_scancode_pressed(Scancode::E)  {
        left_paddle.move_up();
      }
      if keystates.is_scancode_pressed(Scancode::K) || keystates.is_scancode_pressed(Scancode::Down) {
        right_paddle.move_down();
      }
      if keystates.is_scancode_pressed(Scancode::I) || keystates.is_scancode_pressed(Scancode::Up)  {
        right_paddle.move_up();
      }
    }

    ball.mov();
    if ball.off_left_edge() {
      right_score += 1;
      ball = Ball::new();
    } else if ball.off_right_edge() {
      left_score += 1;
      ball = Ball::new();
    }

    if ball.moving_left() {
      ball.maybe_bounce_off(&left_paddle);
    } else {
      ball.maybe_bounce_off(&right_paddle);
    }

    renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    renderer.clear();

    renderer.set_draw_color(sdl2::pixels::Color::RGB(0xff, 0xff, 0xff));
    renderer.fill_rect(left_paddle.to_sdl());
    renderer.fill_rect(right_paddle.to_sdl());
    renderer.fill_rect(ball.to_sdl());

    {
      let fps_surface = font.render_str_solid(&format!("{} fps", fps.average()), sdl2::pixels::Color::RGB(0xff, 0xff, 0xff)).unwrap();
      let fps_texture = renderer.create_texture_from_surface(&fps_surface).unwrap();
      renderer.copy(&fps_texture, None, Some(sdl2::rect::Rect::new_unwrap(10, 10, fps_surface.get_width(), fps_surface.get_height())));
    }

    {
      let time_surface = font.render_str_solid(&format!("{} ms", previous_frame_length), sdl2::pixels::Color::RGB(0xff, 0xff, 0xff)).unwrap();
      let time_texture = renderer.create_texture_from_surface(&time_surface).unwrap();
      renderer.copy(&time_texture, None, Some(sdl2::rect::Rect::new_unwrap(10, 25, time_surface.get_width(), time_surface.get_height())));
    }

    {
      let left_score_surface = font.render_str_solid(&format!("{}", left_score), sdl2::pixels::Color::RGB(0xff, 0xff, 0xff)).unwrap();
      let left_score_texture = renderer.create_texture_from_surface(&left_score_surface).unwrap();
      renderer.copy(&left_score_texture, None, Some(sdl2::rect::Rect::new_unwrap((WIDTH/2 - left_score_surface.get_width() - 10) as i32, 25, left_score_surface.get_width(), left_score_surface.get_height())));
    }

    {
      let right_score_surface = font.render_str_solid(&format!("{}", right_score), sdl2::pixels::Color::RGB(0xff, 0xff, 0xff)).unwrap();
      let right_score_texture = renderer.create_texture_from_surface(&right_score_surface).unwrap();
      renderer.copy(&right_score_texture, None, Some(sdl2::rect::Rect::new_unwrap((WIDTH / 2 + 10) as i32, 25, right_score_surface.get_width(), right_score_surface.get_height())));
    }

    renderer.present();

    let frame_end = sdl2::timer::get_ticks();
    let frame_length = frame_end - frame_start;
    previous_frame_length = frame_length;
    if frame_length < MS_PER_FRAME {
      sdl2::timer::delay(MS_PER_FRAME - frame_length);
    }
  }

  sdl2_ttf::quit();
}
