/// Runs all of the tests from the `cpu_instrs` test suite, originally authored
/// by `Shay Green (blargg)`.
pub mod cpu_instrs;

pub mod mem;

pub trait CpuTestHarness {
  fn get_reg_8(&self, n: CpuReg8) -> u8;
  fn set_reg_8(&mut self, n: CpuReg8, val: u8);

  fn get_reg_16(&self, n: CpuReg16) -> u16;
  fn set_reg_16(&mut self, n: CpuReg16, val: u16);

  fn run(&mut self, mem: &mut mem::Mem);

  fn execute(&mut self, mem: &mut mem::Mem, instr: &[u8]) {
    self.set_reg_16(CpuReg16::PC, 0);

    for (n, &val) in instr.iter().enumerate() {
      let address = n as u16;
      mem.set(address, val);
    }

    self.run(mem)
  }
}

pub enum CpuReg8 {
  A,
  F,
  B,
  C,
  D,
  E,
  H,
  L,
}

pub enum CpuReg16 {
  PC,
  SP,
}

impl<T: CpuTestHarness> super::crc::CrcHashable for T {
  fn add_to(&self, crc: &mut crate::crc::Crc) {
    crc.add(&self.get_reg_8(CpuReg8::A));
    crc.add(&self.get_reg_8(CpuReg8::F));
    crc.add(&self.get_reg_8(CpuReg8::B));
    crc.add(&self.get_reg_8(CpuReg8::C));
    crc.add(&self.get_reg_8(CpuReg8::D));
    crc.add(&self.get_reg_8(CpuReg8::E));
    crc.add(&self.get_reg_8(CpuReg8::H));
    crc.add(&self.get_reg_8(CpuReg8::L));
  }
}
