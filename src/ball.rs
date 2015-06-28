extern crate sdl2;

use std::f64::consts::PI;

pub struct Ball {
  x: f64,
  y: f64,
  speed: f64,
  angle: f64,
}

impl Ball {
  pub fn new() -> Ball {
    Ball {x: 40.0, y: 40.0, speed: 1.0, angle: PI / 4.0}
  }
}
