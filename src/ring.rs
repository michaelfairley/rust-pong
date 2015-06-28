pub struct Ring {
  head: usize,
  elements: Vec<u32>,
}

impl Ring {
  pub fn new(size: usize) -> Ring {
    let mut elements = Vec::with_capacity(size);
    for _ in 0..size {
      elements.push(0);
    }

    Ring{head: 0, elements: elements}
  }

  pub fn push(&mut self, new_element: u32) {
    self.head = (self.head + 1) % self.elements.len();
    self.elements[self.head] = new_element;
  }

  pub fn head(&self) -> u32 {
    self.elements[self.head]
  }

  pub fn tail(&self) -> u32 {
    let tail = (self.head + 1) % self.elements.len();
    self.elements[tail]
  }
}
