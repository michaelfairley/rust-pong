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
    if self.y < 0.0 {
      self.y = -self.y;
      self.angle = (-dy).atan2(dx);
    } else if self.y > super::HEIGHT as f64{
      self.y -= self.y - (super::HEIGHT as f64);
      self.angle = (-dy).atan2(dx);
    }
  }

  pub fn to_sdl(&self) -> sdl2::rect::Rect {
    sdl2::rect::Rect::new_unwrap(
      (self.x - SIZE/2.0) as i32,
      (self.y - SIZE/2.0) as i32,
      SIZE as u32,
      SIZE as u32,
    )
  }
}
