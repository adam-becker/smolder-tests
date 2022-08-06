use crate::crc::CrcSource;

pub mod cpu_instrs;

/// Address used for tests that require deferencing `HL`.
const RP_TEMP: u16 = 0xdef4;

impl<T: CpuTestHarness> CrcSource for T {
  fn add(&self, crc: &mut crate::crc::Crc) {
    crc.add(self.get_reg(CpuRegister::A)); // AF
    crc.add(self.get_reg(CpuRegister::F));

    crc.add(self.get_reg(CpuRegister::B)); // BC
    crc.add(self.get_reg(CpuRegister::C));

    crc.add(self.get_reg(CpuRegister::D)); // DE
    crc.add(self.get_reg(CpuRegister::E));

    crc.add(self.get_reg(CpuRegister::H)); // HL
    crc.add(self.get_reg(CpuRegister::L))
  }
}

/// Abstraction to allow the tests to be run against any implementation.
pub trait CpuTestHarness {
  /// Gets a value, or panics if that value wasn't initialized.
  fn get_mem(&mut self, address: u16) -> u8;

  /// Sets a value for later use.
  fn set_mem(&mut self, address: u16, val: u8);

  /// Helper method to set multiple values in linear order, starting from `address`.
  fn set_mem_linear(&mut self, address: u16, val: &[u8]) {
    for (n, &val) in val.iter().enumerate() {
      self.set_mem(address.wrapping_add(n as u16), val)
    }
  }

  /// Gets a register value.
  fn get_reg(&self, n: CpuRegister) -> u8;

  /// Sets a register value.
  fn set_reg(&mut self, n: CpuRegister, val: u8);

  /// Runs an instruction that is available at `$0000`.
  fn run(&mut self);
}

/// Values used to reference the various CPU registers of the `SM83` core.
pub enum CpuRegister {
  /// The `A` register.
  A,
  /// The `F` register.
  F,
  /// The `B` register.
  B,
  /// The `C` register.
  C,
  /// The `D` register.
  D,
  /// The `E` register.
  E,
  /// The `H` register.
  H,
  /// The `L` register.
  L,
}
