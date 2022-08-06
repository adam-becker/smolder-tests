use crate::{
  crc::{Crc, CrcSource},
  gb::{CpuRegister, CpuTestHarness, RP_TEMP},
};

pub(crate) fn op_r_imm(cpu: &mut impl CpuTestHarness) {
  let mut crc = Crc::default();

  cpu.set_mem(RP_TEMP, 0);

  for (expected, instr) in INSTRS {
    cpu.set_mem_linear(0, instr);

    for f in [0x00, 0x10, 0xe0, 0xf0] {
      for a in VALUES {
        for value in VALUES {
          cpu.set_mem(1, value);

          cpu.set_reg(CpuRegister::B, 0x12);
          cpu.set_reg(CpuRegister::C, 0x34);

          cpu.set_reg(CpuRegister::D, 0x56);
          cpu.set_reg(CpuRegister::E, 0x78);

          cpu.set_reg(CpuRegister::H, (RP_TEMP >> 8) as u8);
          cpu.set_reg(CpuRegister::L, (RP_TEMP >> 0) as u8);

          cpu.set_reg(CpuRegister::A, a);
          cpu.set_reg(CpuRegister::F, f);

          cpu.run();
          cpu.add(&mut crc);
          crc.add(cpu.get_mem(RP_TEMP));
        }
      }
    }

    assert_eq!(expected, crc.take_val(), "instr={:?}", instr);
  }
}

#[rustfmt::skip]
const INSTRS: [(u32, &[u8]); 16] = [
  (0xb7057f7f, &[0x36]), // ld  (hl),0x00
  (0xb6948285, &[0x06]), // ld  b,0x00
  (0xf5d60ad8, &[0x0e]), // ld  c,0x00
  (0x2a378c44, &[0x16]), // ld  d,0x00
  (0xfa0546fb, &[0x1e]), // ld  e,0x00
  (0xc19e2fbd, &[0x26]), // ld  h,0x00
  (0xda2a565a, &[0x2e]), // ld  l,0x00
  (0xba14eed0, &[0x3e]), // ld  a,0x00
  (0xd23642ea, &[0xf6]), // or  0x00
  (0x30ab2887, &[0xfe]), // cp  0x00
  (0xc663a24d, &[0xc6]), // add 0x00
  (0x08554e34, &[0xce]), // adc 0x00
  (0x0e971c9b, &[0xd6]), // sub 0x00
  (0xd473f849, &[0xde]), // sbc 0x00
  (0xc6dcc786, &[0xe6]), // and 0x00
  (0x2143bf03, &[0xee]), // xor 0x00
];

#[rustfmt::skip]
const VALUES: [u8; 9] = [
  0x00, 0x01, 0x0f,
  0x10, 0x1f, 0x7f,
  0x80, 0xf0, 0xff,
];
