use crate::crc::CrcSource;

pub mod cpu_instrs;

/// Address used for tests that require deferencing `HL`.
const RP_TEMP: u16 = 0xdef4;

impl<T: CpuTestHarness> CrcSource for T {
  fn add(&self, crc: &mut crate::crc::Crc) {
    crc.add(self.get_reg_8(CpuReg8::A)); // AF
    crc.add(self.get_reg_8(CpuReg8::F));

    crc.add(self.get_reg_8(CpuReg8::B)); // BC
    crc.add(self.get_reg_8(CpuReg8::C));

    crc.add(self.get_reg_8(CpuReg8::D)); // DE
    crc.add(self.get_reg_8(CpuReg8::E));

    crc.add(self.get_reg_8(CpuReg8::H)); // HL
    crc.add(self.get_reg_8(CpuReg8::L));
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
  fn get_reg_8(&self, n: CpuReg8) -> u8;

  /// Sets a register value.
  fn set_reg_8(&mut self, n: CpuReg8, val: u8);

  /// Gets a register value.
  fn get_reg_16(&self, n: CpuReg16) -> u16;

  /// Sets a register value.
  fn set_reg_16(&mut self, n: CpuReg16, val: u16);

  /// Runs an instruction that is available at `$0000`.
  fn run(&mut self);
}

/// Values used to reference the various 8-bit CPU registers of the `SM83` core.
pub enum CpuReg8 {
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

/// Values used to reference the various 16-bit CPU registers of the `SM83` core.
pub enum CpuReg16 {
  /// The `AF` register.
  AF,
  /// The `BC` register.
  BC,
  /// The `DE` register.
  DE,
  /// The `HL` register.
  HL,
  /// The `PC` register.
  PC,
  /// The `SP` register.
  SP,
}
