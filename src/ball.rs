extern crate sdl2;
extern crate rand;

use self::rand::{thread_rng,Rng};

use std::f64::consts::PI;

const SIZE: f64 = 20.0;

pub struct Ball {
  x: f64,
  y: f64,
  speed: f64,
  angle: f64,
}

impl Ball {
  pub fn new() -> Ball {
    let angle: f64 = thread_rng().gen_range(PI / -4.0, PI / 4.0) + (if thread_rng().gen() { 0.0 } else { PI });
    Ball{x: (super::WIDTH as f64) / 2.0, y: (super::HEIGHT as f64) / 2.0, speed: 4.0, angle: angle}
  }

  pub fn mov(&mut self) {
    let (dy, dx) = self.angle.sin_cos();
    let (dy, dx) = (dy * self.speed, dx * self.speed);

    self.x += dx;
    self.y += dy;
    if self.top() < 0.0 {
      self.y += (0.0 - self.top());
      self.angle = (-dy).atan2(dx);
    } else if self.bottom() > super::HEIGHT as f64{
      self.y -= (self.bottom() - super::HEIGHT as f64);
      self.angle = (-dy).atan2(dx);
    }
  }

  fn top(&self) -> f64 {
    self.y - SIZE/2.0
  }

  fn bottom(&self) -> f64 {
    self.y + SIZE/2.0
  }

  fn left(&self) -> f64 {
    self.x - SIZE/2.0
  }

  fn right(&self) -> f64 {
    self.x + SIZE/2.0
  }

  pub fn to_sdl(&self) -> sdl2::rect::Rect {
    sdl2::rect::Rect::new_unwrap(
      self.left() as i32,
      self.top() as i32,
      SIZE as u32,
      SIZE as u32,
    )
  }

  pub fn off_left_edge(&self) -> bool {
    self.right() < 0.0
  }

  pub fn off_right_edge(&self) -> bool {
    self.left() > super::WIDTH as f64
  }

  pub fn moving_left(&self) -> bool {
    self.angle.cos() < 0.0
  }

  pub fn maybe_bounce_off(&mut self, paddle: &super::paddle::Paddle) {
    if self.intersecting(paddle) {
      let (_dy, dx) = self.angle.sin_cos();
      let paddle_ratio = (self.y - paddle.y) / super::paddle::HEIGHT;
      let angle = paddle_ratio * PI/1.5;
      self.angle = angle;
      if dx > 0.0 { self.angle = -self.angle + PI; }

      self.speed *= 1.1;
    }
  }

  fn intersecting(&self, paddle: &super::paddle::Paddle) -> bool {
    self.to_sdl().has_intersection(&paddle.to_sdl())
  }
}
