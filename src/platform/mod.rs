mod monero;

use crate::platform::monero::Monero;

pub trait Platform {
  fn get_address(&self);
}

#[derive(Debug)]
pub enum PlatformType {
  Monero
}

pub struct PlatformFactory;
impl PlatformFactory {
  pub fn new_platform(p: &PlatformType, seed: &[u8]) -> Box<dyn Platform> {
    match p {
      PlatformType::Monero => Box::new(Monero::new(seed)),
    }
  }
}