use std::collections::HashMap;

#[derive(Default)]
pub struct Mem(HashMap<u16, u8>);

impl Mem {
  /// Gets a value, or panics if that value wasn't initialized.
  pub fn get(&mut self, address: u16) -> u8 {
    if let Some(&val) = self.0.get(&address) {
      return val;
    }

    panic!("read from uninitialized memory: ${:04x}", address);
  }

  /// Sets a value for later use.
  pub fn set(&mut self, address: u16, val: u8) {
    self.0.insert(address, val);
  }
}
