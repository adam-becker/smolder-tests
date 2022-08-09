use crate::{
  crc::Crc,
  gb::{mem::Mem, CpuReg8, CpuTestHarness},
};

const BC: u16 = 0xdef4;
const DE: u16 = 0xdef5;
const HL: u16 = 0xdef6;

pub fn op_a_hl(cpu: &mut impl CpuTestHarness) {
  let mut crc = Crc::default();
  let mut mem = Mem::default();

  for (expected, instr) in INSTRS {
    for f in [0x00, 0x10, 0xe0, 0xf0] {
      for n in 0..VALUES.len() {
        for m in 0..VALUES.len() {
          cpu.set_reg_8(CpuReg8::A, VALUES[n]); // AF
          cpu.set_reg_8(CpuReg8::F, f);
          cpu.set_reg_8(CpuReg8::B, (BC >> 8) as u8); // BC
          cpu.set_reg_8(CpuReg8::C, (BC >> 0) as u8);
          cpu.set_reg_8(CpuReg8::D, (DE >> 8) as u8); // DE
          cpu.set_reg_8(CpuReg8::E, (DE >> 0) as u8);
          cpu.set_reg_8(CpuReg8::H, (HL >> 8) as u8); // HL
          cpu.set_reg_8(CpuReg8::L, (HL >> 0) as u8);

          mem.set(BC, VALUES[(m + 0) % VALUES.len()]);
          mem.set(DE, VALUES[(m + 1) % VALUES.len()]);
          mem.set(HL, VALUES[(m + 2) % VALUES.len()]);

          cpu.execute(&mut mem, instr);

          crc.add(&cpu.get_reg_8(CpuReg8::A));
          crc.add(&cpu.get_reg_8(CpuReg8::F));
          crc.add(&cpu.get_reg_8(CpuReg8::H));
          crc.add(&cpu.get_reg_8(CpuReg8::L));

          crc.add(&mem.get(BC));
          crc.add(&mem.get(DE));
          crc.add(&mem.get(HL));
        }
      }
    }

    assert_eq!(expected, crc.take_val(), "instr={:?}", instr);
  }
}

const VALUES: [u8; 14] = [
  0x00, 0x01, 0x0f, 0x10, 0x1f, 0x7f, 0x80, 0xf0, 0xff, 0x02, 0x04, 0x08, 0x20, 0x40,
];

const INSTRS: [(u32, &'static [u8]); 51] = [
  (0xa709e5e0, &[0x0a]),       // ld   a,(bc)
  (0xae0d28fb, &[0x1a]),       // ld   a,(de)
  (0xd891bbac, &[0x02]),       // ld   (bc),a
  (0xc4afe2b3, &[0x12]),       // ld   (de),a
  (0x0702b53d, &[0x2a]),       // ld   a,(hl+)
  (0x7e5b6e4f, &[0x3a]),       // ld   a,(hl-)
  (0x14e702ae, &[0x22]),       // ld   (hl+),a
  (0x6dbed9dc, &[0x32]),       // ld   (hl-),a
  (0x42a948f1, &[0xb6]),       // or   (hl)
  (0x57fe0867, &[0xbe]),       // cp   (hl)
  (0xb1a96a06, &[0x86]),       // add  (hl)
  (0xf084a5fd, &[0x8e]),       // adc  (hl)
  (0xa924fc82, &[0x96]),       // sub  (hl)
  (0xe2bb1da8, &[0x9e]),       // sbc  (hl)
  (0xde8c23f8, &[0xa6]),       // and  (hl)
  (0xd1641d0e, &[0xae]),       // xor  (hl)
  (0x4124e005, &[0x35]),       // dec  (hl)
  (0x55477553, &[0x34]),       // inc  (hl)
  (0x6a10d9f4, &[0xcb, 0x06]), // rlc  (hl)
  (0xd8281638, &[0xcb, 0x0e]), // rrc  (hl)
  (0xe0a328d1, &[0xcb, 0x16]), // rl   (hl)
  (0xfeb805a2, &[0xcb, 0x1e]), // rr   (hl)
  (0x8ff5f4b0, &[0xcb, 0x26]), // sla  (hl)
  (0xb003394b, &[0xcb, 0x2e]), // sra  (hl)
  (0x90ba078a, &[0xcb, 0x36]), // swap (hl)
  (0x78a79925, &[0xcb, 0x3e]), // srl  (hl)
  (0x49d19ae6, &[0xcb, 0x46]), // bit  0,(hl)
  (0xe5a3b2c9, &[0xcb, 0x4e]), // bit  1,(hl)
  (0x5acb3436, &[0xcb, 0x56]), // bit  2,(hl)
  (0x09714297, &[0xcb, 0x5e]), // bit  3,(hl)
  (0xec258739, &[0xcb, 0x66]), // bit  4,(hl)
  (0xb3c5ee54, &[0xcb, 0x6e]), // bit  5,(hl)
  (0xbd6fb5fc, &[0xcb, 0x76]), // bit  6,(hl)
  (0x6f46d80b, &[0xcb, 0x7e]), // bit  7,(hl)
  (0x9f81276a, &[0xcb, 0x86]), // res  0,(hl)
  (0x71e238f8, &[0xcb, 0x8e]), // res  1,(hl)
  (0x83211955, &[0xcb, 0x96]), // res  2,(hl)
  (0x4b9f854b, &[0xcb, 0x9e]), // res  3,(hl)
  (0x601478a1, &[0xcb, 0xa6]), // res  4,(hl)
  (0x57d90858, &[0xcb, 0xae]), // res  5,(hl)
  (0x9a838c11, &[0xcb, 0xb6]), // res  6,(hl)
  (0x90d1019f, &[0xcb, 0xbe]), // res  7,(hl)
  (0x5a0b82e8, &[0xcb, 0xc6]), // set  0,(hl)
  (0x218675bd, &[0xcb, 0xce]), // set  1,(hl)
  (0x23e983df, &[0xcb, 0xd6]), // set  2,(hl)
  (0xd17fb61e, &[0xcb, 0xde]), // set  3,(hl)
  (0x8ea5184a, &[0xcb, 0xe6]), // set  4,(hl)
  (0x51cacfcf, &[0xcb, 0xee]), // set  5,(hl)
  (0x96a4033f, &[0xcb, 0xf6]), // set  6,(hl)
  (0x889e1fc3, &[0xcb, 0xfe]), // set  7,(hl)
  (0xb11fdf0c, &[0x27]),       // daa
];
