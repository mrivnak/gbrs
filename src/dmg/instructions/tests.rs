use crate::dmg::registers::Registers;

use super::*;

#[test]
fn test_NOP() {
    let base_mem = MemoryBus::create();
    let mut out_mem = base_mem.clone();
    let base_reg = Registers::create();
    let mut out_reg = base_reg.clone();

    execute_instruction(0x00, &mut out_reg, &mut out_mem);

    assert_eq!(base_mem, out_mem);
    assert_eq!(base_reg, out_reg);
}
