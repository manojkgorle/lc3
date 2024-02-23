pub mod vm;
pub const MEMORY_SIZE: usize = u16::MAX as usize;
pub const PC_START: u16 = 0x3000;
use serde::{Deserialize, Serialize};
enum ConditionFlag {
    POS = 1 << 0, // Positive
    ZRO = 1 << 1, // Zero
    NEG = 1 << 2, // Negative
}

#[cfg(test)]
mod tests {
    use self::vm::execute_program;

    use super::*;
    use crate::vm::vm::VM;
    #[test]
    fn test_execute() {
        let mut vm: VM = VM::new();
        let instructions = [
            0b0001001000000000,
            0b0101001000100001,
            0b0000111000101000,
            0b0000001010101010,
            0b0000001100011111,
            0b1100100000000000,
            0b0100010000000000,
            0b0010101111111111,
            0b0110110101010010,
            0b1110010111111111,
            0b1001011010001000,
            0b0011100000000001,
            0b0111101111010101,
        ];
        for i in 0..instructions.len() {
            vm.write_memory(i, instructions[i]);
        }
        vm.registers.pc = PC_START;
        execute_program(&mut vm);
    }
}
