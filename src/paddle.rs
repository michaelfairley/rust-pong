extern crate sdl2;

const SPEED: f64 = 5.0;
const WIDTH: i32 = 15;
const HEIGHT: i32 = 80;

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
    Paddle{side: side, y: (super::HEIGHT/2 - HEIGHT/2) as f64}
  }

  pub fn move_down(&mut self) {
    self.y += SPEED;
  }

  pub fn move_up(&mut self) {
    self.y -= SPEED;
  }

  pub fn to_sdl(&self) -> sdl2::rect::Rect {
    let x = match self.side {
      Side::Left => WIDTH,
      Side::Right => super::WIDTH - WIDTH*2,
    };

    sdl2::rect::Rect{
      x: x,
      y: self.y as i32 - HEIGHT/2,
      w: WIDTH,
      h: HEIGHT,
    }
  }
}
