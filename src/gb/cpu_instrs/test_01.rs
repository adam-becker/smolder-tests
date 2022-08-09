use crate::{
  crc::Crc,
  gb::{mem::Mem, CpuReg8, CpuTestHarness, CpuReg16},
};

const SP: u16 = 0xffc0;

pub fn special(cpu: &mut impl CpuTestHarness) {
  jr_negative(cpu);
  jr_positive(cpu);
  ld_pc_hl(cpu);
  pop_af(cpu);
  daa(cpu);
}

fn jr_negative(cpu: &mut impl CpuTestHarness) {
  let mut mem = Mem::default();
  cpu.execute(&mut mem, &[0x18, 0xfe, 0x00]);

  assert_eq!(0, cpu.get_reg_16(CpuReg16::PC), "JR negative");
}

fn jr_positive(cpu: &mut impl CpuTestHarness) {
  let mut mem = Mem::default();
  cpu.execute(&mut mem, &[0x18, 0x7e, 0x00]);

  assert_eq!(0x80, cpu.get_reg_16(CpuReg16::PC), "JR positive");
}

fn ld_pc_hl(cpu: &mut impl CpuTestHarness) {
  let mut mem = Mem::default();

  cpu.set_reg_8(CpuReg8::H, 0xaa);
  cpu.set_reg_8(CpuReg8::L, 0x55);
  cpu.execute(&mut mem, &[0xe9, 0x00]);

  assert_eq!(0xaa55, cpu.get_reg_16(CpuReg16::PC), "LD PC,HL");
}

fn pop_af(cpu: &mut impl CpuTestHarness) {
  let mut mem = Mem::default();
  mem.set(SP + 0, 0xf0);
  mem.set(SP + 1, 0xff);

  cpu.set_reg_16(CpuReg16::SP, SP);
  cpu.execute(&mut mem, &[0xf1, 0x00]);

  assert_eq!(0xff, cpu.get_reg_8(CpuReg8::A), "POP AF (A)");
  assert_eq!(0xf0, cpu.get_reg_8(CpuReg8::F), "POP AF (F)");
}

fn daa(cpu: &mut impl CpuTestHarness) {
  let mut crc = Crc::default();
  let mut mem = Mem::default();

  for f in (0..256).step_by(16) {
    for a in 0..256 {
      cpu.set_reg_8(CpuReg8::A, a as u8);
      cpu.set_reg_8(CpuReg8::F, f as u8);

      cpu.execute(&mut mem, &[0x27, 0x00]);

      crc.add(&cpu.get_reg_8(CpuReg8::A));
      crc.add(&cpu.get_reg_8(CpuReg8::F));
    }
  }

  assert_eq!(0x95607275, crc.take_val(), "DAA");
}
