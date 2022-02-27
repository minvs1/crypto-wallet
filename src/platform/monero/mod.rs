use crate::platform::Platform;

#[derive(Debug)]
pub struct Monero {
  seed: Vec<u8>,
}

impl Monero {
  pub fn new(seed: &[u8]) -> Self {
    Self { seed: seed.to_vec() }
  }
}

impl Platform for Monero {
  fn get_address(&self) {

  }
}