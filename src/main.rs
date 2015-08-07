extern crate sdl2;
extern crate sdl2_ttf;

pub mod ring;
pub mod fps;
pub mod paddle;
pub mod ball;
pub mod text;
use ball::*;

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

  let text = text::Text::new("OpenSans-Regular.ttf", 20);

  let mut fps = fps::FPS::new();

  while running {
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

    text.render(&mut renderer, &format!("{} fps", fps.average()), 10, 10);

    text.render(&mut renderer, &format!("{}", left_score), (WIDTH / 2 - 20) as i32, 25);
    text.render(&mut renderer, &format!("{}", right_score), (WIDTH / 2 + 10) as i32, 25);

    renderer.present();
  }

  sdl2_ttf::quit();
}
