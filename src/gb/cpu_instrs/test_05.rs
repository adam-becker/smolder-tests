use crate::{
  crc::{Crc, CrcSource},
  gb::{CpuRegister, CpuTestHarness},
};

pub(crate) fn op_rp(cpu: &mut impl CpuTestHarness) {
  let mut crc = Crc::default();

  for (expected, instr) in INSTRS {
    cpu.set_mem_linear(0, instr);

    for f in [0x00, 0x10, 0xe0, 0xf0] {
      for n in 0..15 {
        let hl = VALUES[n];

        for n in 0..15 {
          let bc = VALUES[n + 0];
          let de = VALUES[n + 1];
          let af = VALUES[n + 2];

          cpu.set_reg(CpuRegister::A, (af >> 0) as u8); // AF
          cpu.set_reg(CpuRegister::F, f);

          cpu.set_reg(CpuRegister::B, (bc >> 8) as u8); // BC
          cpu.set_reg(CpuRegister::C, (bc >> 0) as u8);

          cpu.set_reg(CpuRegister::D, (de >> 8) as u8); // DE
          cpu.set_reg(CpuRegister::E, (de >> 0) as u8);

          cpu.set_reg(CpuRegister::H, (hl >> 8) as u8); // HL
          cpu.set_reg(CpuRegister::L, (hl >> 0) as u8);

          cpu.run();
          cpu.add(&mut crc);
        }
      }
    }

    assert_eq!(expected, crc.take_val());
  }
}

#[rustfmt::skip]
const INSTRS: [(u32, &'static [u8]); 9] = [
  (0xa336a1c0, &[0x0b]), // dec bc
  (0x2bb815be, &[0x1b]), // dec de
  (0xc2c6939f, &[0x2b]), // dec hl
  (0x8107c086, &[0x03]), // inc bc
  (0x3835750f, &[0x13]), // inc de
  (0x1b0ac76b, &[0x23]), // inc hl
  (0x424b6806, &[0x09]), // add hl,bc
  (0x188cb464, &[0x19]), // add hl,de
  (0x94316cfb, &[0x29]), // add hl,hl
];

#[rustfmt::skip]
const VALUES: [u16; 30] = [
  0x0000, 0x0001, 0x000f, 0x0010, 0x001f, 0x007f, 0x0080, 0x00ff, 0x0100, 0x0f00, 0x1f00, 0x1000, 0x7fff, 0x8000, 0xffff,
  0x0000, 0x0001, 0x000f, 0x0010, 0x001f, 0x007f, 0x0080, 0x00ff, 0x0100, 0x0f00, 0x1f00, 0x1000, 0x7fff, 0x8000, 0xffff,
];
