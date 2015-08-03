extern crate sdl2;

use ring::Ring;

pub struct FPS {
  ring: Ring,
}

const SIZE: usize = 11;

impl FPS {
  pub fn new() -> FPS {
    FPS { ring: Ring::new(SIZE) }
  }

  pub fn tick(&mut self) {
    let now = sdl2::timer::get_ticks();
    self.ring.push(now);
  }

  pub fn average(&self) -> u32 {
    let ticks_over_period = self.ring.head() - self.ring.tail();
    ((SIZE - 1) as u32 * 1000) / ticks_over_period
  }
}
