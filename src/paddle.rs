extern crate sdl2;

const SPEED: f64 = 5.0;
pub const WIDTH: f64 = 15.0;
pub const HEIGHT: f64 = 80.0;

pub enum Side {
  Left,
  Right,
}

pub struct Paddle {
  pub side: Side,
  pub y: f64,
}

impl Paddle {
  pub fn new(side: Side) -> Paddle {
    Paddle{side: side, y: ((super::HEIGHT/2) as f64 - HEIGHT/2.0) as f64}
  }

  pub fn move_down(&mut self) {
    self.y += SPEED;
    if self.y + HEIGHT/2.0 > super::HEIGHT as f64 {
      self.y = super::HEIGHT as f64 - HEIGHT/2.0;
    }
  }

  pub fn move_up(&mut self) {
    self.y -= SPEED;
    if self.y - HEIGHT/2.0 < 0.0 {
      self.y = HEIGHT/2.0;
    }
  }

  pub fn to_sdl(&self) -> sdl2::rect::Rect {
    let x = match self.side {
      Side::Left => WIDTH,
      Side::Right => super::WIDTH as f64 - WIDTH*2.0,
    };

    sdl2::rect::Rect::new_unwrap(
      x as i32,
      (self.y - HEIGHT/2.0) as i32,
      WIDTH as u32,
      HEIGHT as u32,
    )
  }
}
