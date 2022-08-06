use crate::{
  crc::{Crc, CrcSource},
  gb::{CpuRegister, CpuTestHarness, RP_TEMP},
};

pub(crate) fn ld_r_r(cpu: &mut impl CpuTestHarness) {
  let mut crc = Crc::default();

  for (expected, instr) in INSTRS {
    cpu.set_mem_linear(0, instr);

    for af in [0xbc00, 0xbc10, 0xbce0, 0xbcf0] {
      cpu.set_mem(RP_TEMP, 0xde);

      let bc = 0x3456;
      let de = 0x789a;
      let hl = RP_TEMP;

      cpu.set_reg(CpuRegister::B, (bc >> 8) as u8);
      cpu.set_reg(CpuRegister::C, (bc >> 0) as u8);

      cpu.set_reg(CpuRegister::D, (de >> 8) as u8);
      cpu.set_reg(CpuRegister::E, (de >> 0) as u8);

      cpu.set_reg(CpuRegister::H, (hl >> 8) as u8);
      cpu.set_reg(CpuRegister::L, (hl >> 0) as u8);

      cpu.set_reg(CpuRegister::A, (af >> 8) as u8);
      cpu.set_reg(CpuRegister::F, (af >> 0) as u8);

      cpu.run();
      cpu.add(&mut crc);
      crc.add(cpu.get_mem(RP_TEMP));
    }

    assert_eq!(expected, crc.take_val(), "instr={:?}", instr);
  }
}

#[rustfmt::skip]
const INSTRS: [(u32, &'static [u8]); 63] = [
  (0x06af3a40, &[0x40]), // ld   b,b
  (0xabb2cbb6, &[0x41]), // ld   b,c
  (0x9b71ef6f, &[0x42]), // ld   b,d
  (0xb96ce375, &[0x43]), // ld   b,e
  (0xb726fb34, &[0x44]), // ld   b,h
  (0xce2fb95a, &[0x45]), // ld   b,l
  (0xb726fb34, &[0x46]), // ld   b,(hl)
  (0x1a3b0ac2, &[0x47]), // ld   b,a
  (0x7cd68a2a, &[0x48]), // ld   c,b
  (0x06af3a40, &[0x49]), // ld   c,c
  (0x70740aaf, &[0x4a]), // ld   c,d
  (0x6f6ea919, &[0x4b]), // ld   c,e
  (0xfefeda11, &[0x4c]), // ld   c,h
  (0x2b041018, &[0x4d]), // ld   c,l
  (0xfefeda11, &[0x4e]), // ld   c,(hl)
  (0x84876a7b, &[0x4f]), // ld   c,a
  (0x1234878b, &[0x50]), // ld   d,b
  (0x01de4500, &[0x51]), // ld   d,c
  (0x06af3a40, &[0x52]), // ld   d,d
  (0xc68fe293, &[0x53]), // ld   d,e
  (0x32907ddd, &[0x54]), // ld   d,h
  (0xa81b90ff, &[0x55]), // ld   d,l
  (0x32907ddd, &[0x56]), // ld   d,(hl)
  (0x217abf56, &[0x57]), // ld   d,a
  (0x06fac023, &[0x58]), // ld   e,b
  (0x80a01d3b, &[0x59]), // ld   e,c
  (0x9c1b443f, &[0x5a]), // ld   e,d
  (0x06af3a40, &[0x5b]), // ld   e,e
  (0xcd852556, &[0x5c]), // ld   e,h
  (0xf9dbb1d7, &[0x5d]), // ld   e,l
  (0xcd852556, &[0x5e]), // ld   e,(hl)
  (0x4bdff84e, &[0x5f]), // ld   e,a
  (0x18f9c3f0, &[0x60]), // ld   h,b
  (0x91f60f20, &[0x61]), // ld   h,c
  (0x46ce6971, &[0x62]), // ld   h,d
  (0x4d03a0f0, &[0x63]), // ld   h,e
  (0x06af3a40, &[0x64]), // ld   h,h
  (0x36e24729, &[0x65]), // ld   h,l
  (0x06af3a40, &[0x66]), // ld   h,(hl)
  (0x8fa0f690, &[0x67]), // ld   h,a
  (0xa926623d, &[0x68]), // ld   l,b
  (0x75c152a4, &[0x69]), // ld   l,c
  (0x4075ed45, &[0x6a]), // ld   l,d
  (0x56634d8a, &[0x6b]), // ld   l,e
  (0xfe2dbaaf, &[0x6c]), // ld   l,h
  (0x06af3a40, &[0x6d]), // ld   l,l
  (0xfe2dbaaf, &[0x6e]), // ld   l,(hl)
  (0x22ca8a36, &[0x6f]), // ld   l,a
  (0x65c28d34, &[0x70]), // ld   (hl),b
  (0x54ffdb1a, &[0x71]), // ld   (hl),c
  (0x55e8c032, &[0x72]), // ld   (hl),d
  (0x2f874aed, &[0x73]), // ld   (hl),e
  (0x06af3a40, &[0x74]), // ld   (hl),h
  (0xe681bc9d, &[0x75]), // ld   (hl),l
  (0x37926c6e, &[0x77]), // ld   (hl),a
  (0x29c3ecb1, &[0x78]), // ld   a,b
  (0xa19fc51d, &[0x79]), // ld   a,c
  (0xcd666f59, &[0x7a]), // ld   a,d
  (0x74fdfbb4, &[0x7b]), // ld   a,e
  (0x8ef313ec, &[0x7c]), // ld   a,h
  (0xed5f0c70, &[0x7d]), // ld   a,l
  (0x8ef313ec, &[0x7e]), // ld   a,(hl)
  (0x06af3a40, &[0x7f]), // ld   a,a
];
