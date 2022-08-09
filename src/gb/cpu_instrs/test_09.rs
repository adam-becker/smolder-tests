use crate::{
  crc::Crc,
  gb::{mem::Mem, CpuReg8, CpuTestHarness},
};

pub fn op_r_r(cpu: &mut impl CpuTestHarness) {
  let mut crc = Crc::default();
  let mut mem = Mem::default();

  for (expected, instr) in INSTRS {
    for f in [0x00, 0xf0] {
      for n in 0..9 {
        for m in 0..9 {
          cpu.set_reg_8(CpuReg8::A, VALUES[n]); // AF
          cpu.set_reg_8(CpuReg8::F, f);
          cpu.set_reg_8(CpuReg8::B, VALUES[(m + 0) % VALUES.len()]); // BC
          cpu.set_reg_8(CpuReg8::C, VALUES[(m + 1) % VALUES.len()]);
          cpu.set_reg_8(CpuReg8::D, VALUES[(m + 4) % VALUES.len()]); // DE
          cpu.set_reg_8(CpuReg8::E, VALUES[(m + 5) % VALUES.len()]);
          cpu.set_reg_8(CpuReg8::H, VALUES[(m + 2) % VALUES.len()]); // HL
          cpu.set_reg_8(CpuReg8::L, VALUES[(m + 3) % VALUES.len()]);

          cpu.execute(&mut mem, instr);

          crc.add(cpu);
        }
      }
    }

    assert_eq!(expected, crc.take_val(), "instr={:?}", instr);
  }
}

const VALUES: [u8; 9] = [0x00, 0x01, 0x0f, 0x10, 0x1f, 0x7f, 0x80, 0xf0, 0xff];

#[rustfmt::skip]
const INSTRS: [(u32, &'static [u8]); 134] = [
  (0x05bd557c, &[0x00]),       // nop
  (0xd1acc7ba, &[0x2f]),       // cpl
  (0x4a826d74, &[0x37]),       // scf
  (0xc52a060f, &[0x3f]),       // ccf
  (0x9d9b97fa, &[0xb0]),       // or   b
  (0x78a032c3, &[0xb1]),       // or   c
  (0x699fc100, &[0xb2]),       // or   d
  (0xa1c2d1c0, &[0xb3]),       // or   e
  (0xc83f0d55, &[0xb4]),       // or   h
  (0x92977d09, &[0xb5]),       // or   l
  (0x563066ce, &[0xb7]),       // or   a
  (0xa101f395, &[0xb8]),       // cp   b
  (0x4c54975b, &[0xb9]),       // cp   c
  (0x89a0fc56, &[0xba]),       // cp   d
  (0x2a7bf842, &[0xbb]),       // cp   e
  (0x40037ce6, &[0xbc]),       // cp   h
  (0xa8c56045, &[0xbd]),       // cp   l
  (0xefb0bfb7, &[0xbf]),       // cp   a
  (0x4f1b7aa0, &[0x80]),       // add  b
  (0x33b422fb, &[0x81]),       // add  c
  (0xc7b53d06, &[0x82]),       // add  d
  (0x23d5a43c, &[0x83]),       // add  e
  (0x8b75bec1, &[0x84]),       // add  h
  (0xbb989be0, &[0x85]),       // add  l
  (0xe6d9750e, &[0x87]),       // add  a
  (0x66e2a782, &[0x88]),       // adc  b
  (0xe84f78cd, &[0x89]),       // adc  c
  (0x3e2dd48e, &[0x8a]),       // adc  d
  (0xc7585c88, &[0x8b]),       // adc  e
  (0xb95f20f9, &[0x8c]),       // adc  h
  (0x5ecae4a8, &[0x8d]),       // adc  l
  (0x9488dbc8, &[0x8f]),       // adc  a
  (0x60870da3, &[0x90]),       // sub  b
  (0x272bba8b, &[0x91]),       // sub  c
  (0xb1838841, &[0x92]),       // sub  d
  (0xd69e410a, &[0x93]),       // sub  e
  (0xb7198d98, &[0x94]),       // sub  h
  (0xbfd5c613, &[0x95]),       // sub  l
  (0x9f74ce83, &[0x97]),       // sub  a
  (0x5e073400, &[0x98]),       // sbc  b
  (0x681ae1f0, &[0x99]),       // sbc  c
  (0xa785ba8f, &[0x9a]),       // sbc  d
  (0xa50646a0, &[0x9b]),       // sbc  e
  (0x4883f975, &[0x9c]),       // sbc  h
  (0x031bef12, &[0x9d]),       // sbc  l
  (0xea79fbc8, &[0x9f]),       // sbc  a
  (0xa96c009b, &[0xa0]),       // and  b
  (0x57cb5e0d, &[0xa1]),       // and  c
  (0x0c4b1b41, &[0xa2]),       // and  d
  (0xe3d808b2, &[0xa3]),       // and  e
  (0x93e10743, &[0xa4]),       // and  h
  (0xc9237334, &[0xa5]),       // and  l
  (0xf9382f18, &[0xa7]),       // and  a
  (0x5aab3bd1, &[0xa8]),       // xor  b
  (0x03f8c6bf, &[0xa9]),       // xor  c
  (0x32a40c50, &[0xaa]),       // xor  d
  (0xfe7e066b, &[0xab]),       // xor  e
  (0x15d48bed, &[0xac]),       // xor  h
  (0x246d4629, &[0xad]),       // xor  l
  (0x1a155b6e, &[0xaf]),       // xor  a
  (0xb087ae32, &[0x05]),       // dec  b
  (0x4bac20dc, &[0x0d]),       // dec  c
  (0xc760632b, &[0x15]),       // dec  d
  (0xaa7592c1, &[0x1d]),       // dec  e
  (0x5317ca6f, &[0x25]),       // dec  h
  (0xea78c55a, &[0x2d]),       // dec  l
  (0x83100161, &[0x3d]),       // dec  a
  (0x78d808dd, &[0x04]),       // inc  b
  (0x1ff50bca, &[0x0c]),       // inc  c
  (0x01085592, &[0x14]),       // inc  d
  (0x9bcdea7f, &[0x1c]),       // inc  e
  (0x1773aa2a, &[0x24]),       // inc  h
  (0xbad09fe0, &[0x2c]),       // inc  l
  (0x3d7273e7, &[0x3c]),       // inc  a
  (0x3b2f95b7, &[0x07]),       // rlca
  (0x365078a7, &[0x17]),       // rla
  (0x9e5b0481, &[0x0f]),       // rrca
  (0xdda4de9a, &[0x1f]),       // rra
  (0x369bb221, &[0xcb, 0x00]), // rlc  b
  (0x32c8d79f, &[0xcb, 0x01]), // rlc  c
  (0xe5fc0e48, &[0xcb, 0x02]), // rlc  d
  (0x7553c355, &[0xcb, 0x03]), // rlc  e
  (0xe0a9eda4, &[0xcb, 0x04]), // rlc  h
  (0x1da7789e, &[0xcb, 0x05]), // rlc  l
  (0xd67cf4b8, &[0xcb, 0x07]), // rlc  a
  (0x87032a90, &[0xcb, 0x08]), // rrc  b
  (0x90d5d881, &[0xcb, 0x09]), // rrc  c
  (0x52c40263, &[0xcb, 0x0a]), // rrc  d
  (0xb385bec2, &[0xcb, 0x0b]), // rrc  e
  (0x2d9e9a32, &[0xcb, 0x0c]), // rrc  h
  (0x4722fbe3, &[0xcb, 0x0d]), // rrc  l
  (0x7308658e, &[0xcb, 0x0f]), // rrc  a
  (0x95735a72, &[0xcb, 0x10]), // rl   b
  (0x9d59eced, &[0xcb, 0x11]), // rl   c
  (0xf16867c8, &[0xcb, 0x12]), // rl   d
  (0xd541ed4b, &[0xcb, 0x13]), // rl   e
  (0xf3753968, &[0xcb, 0x14]), // rl   h
  (0x0def09fc, &[0xcb, 0x15]), // rl   l
  (0xa3432b20, &[0xcb, 0x17]), // rl   a
  (0x4f89aa69, &[0xcb, 0x18]), // rr   b
  (0x587b8784, &[0xcb, 0x19]), // rr   c
  (0xef560a42, &[0xcb, 0x1a]), // rr   d
  (0xca190e1b, &[0xcb, 0x1b]), // rr   e
  (0x17f91b6f, &[0xcb, 0x1c]), // rr   h
  (0xb24cb6ea, &[0xcb, 0x1d]), // rr   l
  (0xb1c0c41a, &[0xcb, 0x1f]), // rr   a
  (0x4e45b2e2, &[0xcb, 0x20]), // sla  b
  (0xae8d0a91, &[0xcb, 0x21]), // sla  c
  (0xa3553117, &[0xcb, 0x22]), // sla  d
  (0xd872691b, &[0xcb, 0x23]), // sla  e
  (0x8d55e903, &[0xcb, 0x24]), // sla  h
  (0x63362787, &[0xcb, 0x25]), // sla  l
  (0xd11285e6, &[0xcb, 0x27]), // sla  a
  (0x4d9732f2, &[0xcb, 0x28]), // sra  b
  (0xa908fab5, &[0xcb, 0x29]), // sra  c
  (0xc25a2a97, &[0xcb, 0x2a]), // sra  d
  (0x27a42dfd, &[0xcb, 0x2b]), // sra  e
  (0xbdec7c57, &[0xcb, 0x2c]), // sra  h
  (0x211967cc, &[0xcb, 0x2d]), // sra  l
  (0xd6cdd446, &[0xcb, 0x2f]), // sra  a
  (0xe2d455cb, &[0xcb, 0x30]), // swap b
  (0x2e32f39e, &[0xcb, 0x31]), // swap c
  (0xb3bbf8aa, &[0xcb, 0x32]), // swap d
  (0x08cc3af6, &[0xcb, 0x33]), // swap e
  (0x5fc28b64, &[0xcb, 0x34]), // swap h
  (0x67af6658, &[0xcb, 0x35]), // swap l
  (0x662c44b3, &[0xcb, 0x37]), // swap a
  (0x3f3be772, &[0xcb, 0x38]), // srl  b
  (0x170c875b, &[0xcb, 0x39]), // srl  c
  (0xa0b4e258, &[0xcb, 0x3a]), // srl  d
  (0xe6811870, &[0xcb, 0x3b]), // srl  e
  (0xce125642, &[0xcb, 0x3c]), // srl  h
  (0x3c4613bb, &[0xcb, 0x3d]), // srl  l
  (0x53fb5abe, &[0xcb, 0x3f]), // srl  a
];
