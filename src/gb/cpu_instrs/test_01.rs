use crate::{gb::{CpuTestHarness, CpuReg16, CpuReg8}, crc::Crc};

const START_PC: u16 = 0x0000;
const START_SP: u16 = 0xfffe;

pub(crate) fn special(cpu: &mut impl CpuTestHarness) {
  jr_negative(cpu);
  jr_positive(cpu);
  ld_pc_hl(cpu);
  pop_af(cpu);
  daa(cpu);
}

fn jr_negative(cpu: &mut impl CpuTestHarness) {
  cpu.set_mem_linear(START_PC, &[0x18, 0xfe, 0x00]);
  cpu.set_reg_16(CpuReg16::PC, START_PC);
  cpu.run();

  assert_eq!(0, cpu.get_reg_16(CpuReg16::PC), "JR negative");
}

fn jr_positive(cpu: &mut impl CpuTestHarness) {
  cpu.set_mem_linear(START_PC, &[0x18, 0x02, 0x00]);
  cpu.set_reg_16(CpuReg16::PC, START_PC);
  cpu.run();

  assert_eq!(4, cpu.get_reg_16(CpuReg16::PC), "JR positive");
}

fn ld_pc_hl(cpu: &mut impl CpuTestHarness) {
  cpu.set_mem_linear(START_PC, &[0xe9, 0x00]);
  cpu.set_reg_16(CpuReg16::HL, 0xaa55);
  cpu.set_reg_16(CpuReg16::PC, START_PC);
  cpu.run();

  assert_eq!(0xaa55, cpu.get_reg_16(CpuReg16::PC), "LD PC,HL");
}

fn pop_af(cpu: &mut impl CpuTestHarness) {
  cpu.set_mem_linear(START_PC, &[0xf1, 0x00]);
  cpu.set_mem_linear(START_SP, &[0xf0, 0xff]);
  cpu.set_reg_16(CpuReg16::SP, START_SP);
  cpu.set_reg_16(CpuReg16::PC, START_PC);
  cpu.run();

  assert_eq!(0xfff0, cpu.get_reg_16(CpuReg16::AF), "POP AF");
}

fn daa(cpu: &mut impl CpuTestHarness) {
  let mut crc = Crc::default();

  for f in (0..256).step_by(16) {
    for a in 0..256 {
      cpu.set_mem_linear(START_PC, &[0x27, 0x00]);
      cpu.set_reg_16(CpuReg16::PC, START_PC);
      cpu.set_reg_8(CpuReg8::A, a as u8);
      cpu.set_reg_8(CpuReg8::F, f as u8);
      cpu.run();

      crc.add(cpu.get_reg_8(CpuReg8::A));
      crc.add(cpu.get_reg_8(CpuReg8::F));
    }
  }

  assert_eq!(!0x6a9f8d8a, crc.take_val(), "DAA");
}
