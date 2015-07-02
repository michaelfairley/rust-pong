extern crate sdl2;

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
    Ball{x: (super::WIDTH as f64) / 2.0, y: (super::HEIGHT as f64) / 2.0, speed: 4.0, angle: PI / 4.0}
  }

  pub fn mov(&mut self) {
    let (dx, dy) = self.angle.sin_cos();
    let (dx, dy) = (dx * self.speed, dy * self.speed);

    self.x += dx;
    self.y += dy;
    if self.y < 0.0 {
      self.y = -self.y;
      self.angle = (dy).atan2(-dx);
    } else if self.y > super::HEIGHT as f64{
      self.y -= self.y - (super::HEIGHT as f64);
      self.angle = (dy).atan2(-dx);
    }
  }

  pub fn to_sdl(&self) -> sdl2::rect::Rect {
    sdl2::rect::Rect{
      x: (self.x - SIZE/2.0) as i32,
      y: (self.y - SIZE/2.0) as i32,
      w: SIZE as i32,
      h: SIZE as i32,
    }
  }
}
