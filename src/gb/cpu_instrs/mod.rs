use super::CpuTestHarness;

mod test_01;
mod test_04;
mod test_05;
mod test_06;
mod test_09;
mod test_10;
mod test_11;

/// Runs all of the tests from the `cpu_instrs` test suite.
///
/// The tests were originally authored by `Shay Green (blargg)`.
pub fn test_all(cpu: &mut impl CpuTestHarness) {
  test_01::special(cpu);
  test_04::op_r_imm(cpu);
  test_05::op_rp(cpu);
  test_06::ld_r_r(cpu);
  test_09::op_r_r(cpu);
  test_10::bit_ops(cpu);
  test_11::op_a_hl(cpu);
}
