pub mod vm;
pub const MEMORY_SIZE: usize = u16::MAX as usize;
pub const PC_START: u16 = 0x3000;
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
        let instructions = vec![
            0b0001010001100111, // ADD R2, R1, #7  ; Add 7 to R1, all registers are initialized to 0
            0b0001010010100111,
            0b0001010010100111,
            0b0001010010100111,
            0b0001010010100111,
            0b0001010010100111,
            0b0001010010100111,
        ];
        for i in 0..instructions.len() {
            vm.write_memory(i + (PC_START as usize), instructions[i]);
        }
        vm.write_memory((PC_START as usize) + instructions.len(), 7);
        vm.registers.pc = PC_START;
        execute_program(&mut vm);
        let registers_output = vm.registers;
        println!("{:?}", registers_output);
        assert_eq!(registers_output.r2, 49);
    }

    #[test]
    fn test_add() {
        let mut vm: VM = VM::new();
        vm.registers.r1 = 5;
        vm.registers.r2 = 10;
        vm.registers.pc = PC_START;

        // ADD R3, R1, R2 ; R3 = R1 + R2
        // Instruction : OP | DR | SR1 | (000 | SR2) or 1 | imm5
        let instruction = 0b0001_011_001_000_010; // ADD opcode
        let imm_instruction = 0b0001_100_001_1_01111; // ADD opcode with immediate value
        vm.write_memory(PC_START as usize, instruction);
        vm.write_memory(PC_START as usize + 1, imm_instruction);
        execute_program(&mut vm);

        assert_eq!(vm.registers.r3, 15);
        assert_eq!(vm.registers.r4, 20);
    }

    #[test]
    fn test_and() {
        let mut vm: VM = VM::new();
        vm.registers.r1 = 0b1010;
        vm.registers.r2 = 0b1100;
        vm.registers.pc = PC_START;

        // AND R3, R1, R2 ; R3 = R1 & R2
        // Instruction : OP | DR | SR1 | (000 | SR2) or 1 | imm5
        let instruction = 0b0101_011_001_000_010; // AND opcode
        let imm_instruction = 0b0101_100_001_1_11111; // AND opcode with immediate value
        vm.write_memory(PC_START as usize, instruction);
        vm.write_memory(PC_START as usize + 1, imm_instruction);
        execute_program(&mut vm);

        assert_eq!(vm.registers.r3, 0b1010 & 0b1100);
        assert_eq!(vm.registers.r4, 0b1010 & 0b11111);
    }

    #[test]
    fn test_not() {
        let mut vm: VM = VM::new();
        vm.registers.r1 = 0b1010;
        vm.registers.pc = PC_START;

        // NOT R2, R1 ; R2 = !R1
        // Instruction : OP | DR | SR1 | 111111
        let instruction = 0b1001_010_001_111_111; // NOT opcode
        vm.write_memory(PC_START as usize, instruction);
        execute_program(&mut vm);

        assert_eq!(vm.registers.r2, !0b1010);
    }

    #[test]
    fn test_ld() {
        let mut vm: VM = VM::new();
        vm.registers.pc = PC_START;

        // LD R1, 5 ; R1 = mem[PC + 5]
        // INSTRUCTION : OP | DR | PCoffset9
        let instruction = 0b0010_001_000_000_101; // LD opcode
        vm.write_memory(PC_START as usize, instruction);
        // pc + 6 as pc is incremented by 1 after fetching the instruction
        vm.write_memory((PC_START as usize) + 6 as usize, 42);
        execute_program(&mut vm);

        assert_eq!(vm.registers.r1, 42);
    }

    #[test]
    fn test_st() {
        let mut vm: VM = VM::new();
        vm.registers.r1 = 42;
        vm.registers.pc = PC_START;

        // ST R1, 5 ; mem[PC + 5] = R1
        // INSTRUCTION : OP | SR | PCoffset9
        let instruction = 0b0011_001_000_000_101; // ST opcode
        vm.write_memory(PC_START as usize, instruction);
        execute_program(&mut vm);

        assert_eq!(vm.read_memory((PC_START as usize) + 6), 42);
    }

    #[test]
    fn test_ldi() {
        let mut vm: VM = VM::new();
        vm.registers.pc = PC_START;

        // LDI R1, 5 ; R1 = mem[mem[PC + 5]]
        // INSTRUCTION : OP | DR | PCoffset9
        let instruction = 0b1010_001_000_000_101; // LDI opcode
        vm.write_memory(PC_START as usize, instruction);
        vm.write_memory((PC_START as usize) + 6, PC_START as u16 + 10); // address to load
        vm.write_memory(PC_START as usize + 10, 42); // value to load
        execute_program(&mut vm);

        assert_eq!(vm.registers.r1, 42);
    }

    #[test]
    fn test_sti() {
        let mut vm: VM = VM::new();
        vm.registers.r1 = 42;
        vm.registers.pc = PC_START;

        // STI R1, 5 ; mem[mem[PC + 5]] = R1
        // INSTRUCTION : OP | SR | PCoffset9
        let instruction = 0b1011_001_000_000_101; // STI opcode
        vm.write_memory(PC_START as usize, instruction);
        // pc + 6 as pc is incremented by 1 after fetching the instruction
        vm.write_memory((PC_START as usize) + 6, PC_START as u16 + 10); // address to store
        execute_program(&mut vm);

        assert_eq!(vm.read_memory(PC_START as usize + 10), 42);
    }

    #[test]
    fn test_jmp() {
        let mut vm: VM = VM::new();
        vm.registers.pc = PC_START;

        // JMP R1 ; PC = R1
        let instruction = 0b1100_000_001_000_000; // JMP opcode
        vm.write_memory(PC_START as usize, instruction);
        vm.registers.r1 = PC_START as u16 + 10; // address to jump
        execute_program(&mut vm);

        assert_eq!(vm.registers.pc, PC_START as u16 + 10);
    }

    #[test]
    fn test_jsr() {
        let mut vm: VM = VM::new();
        vm.registers.pc = PC_START;
        vm.registers.r4 = PC_START + 5;
    
        // JSR 5 ; R7 = PC, PC = PC + 5
        let jsr_instruction =  0b0100_1_00000000100; // JSR opcode
        vm.write_memory(PC_START as usize, jsr_instruction);
        execute_program(&mut vm);
    
        assert_eq!(vm.registers.pc, PC_START + 5);
    }

    #[test]
    fn test_rti() {
        // RTI is a special instruction, testing its execution directly is not practical
        // It relies on the state of the hardware and interrupts, which are not easily simulated in unit tests.
        // Integration testing with more comprehensive scenarios would be more suitable.
    }

    #[test]
    fn test_ldr() {
        let mut vm: VM = VM::new();
        vm.registers.r2 = 5;
        vm.registers.pc = PC_START;

        // LDR R1, R2, 2 ; R1 = mem[R2 + 2]
        // INSTRUCTION : OP | DR | BaseR | offset6
        let instruction = 0b0110_001_010_000010; // LDR opcode
        vm.write_memory(PC_START as usize, instruction);
        vm.write_memory(7, 42); // value at R2 + 2
        execute_program(&mut vm);

        assert_eq!(vm.registers.r1, 42);
    }

    #[test]
    fn test_str() {
        let mut vm: VM = VM::new();
        vm.registers.r1 = 42;
        vm.registers.r2 = 5;
        vm.registers.pc = PC_START;

        // STR R1, R2, 2 ; mem[R2 + 2] = R1
        let instruction = 0b0111_001_010_000_010; // STR opcode
        vm.write_memory(PC_START as usize, instruction);
        execute_program(&mut vm);

        assert_eq!(vm.read_memory(7), 42);
    }

    #[test]
    fn test_br() {
        let mut vm: VM = VM::new();
        vm.registers.cond = ConditionFlag::POS as u16; // Set Positive flag
        vm.registers.pc = PC_START;

        // BRnzp 5 ; branch always
        // INSTRUCTION : OP | n | z | p | PCoffset9
        let instruction = 0b0000_111_000000_001; // BR opcode
        vm.write_memory(PC_START as usize, instruction);
        execute_program(&mut vm);

        assert_eq!(vm.registers.pc, PC_START as u16 + 2); // Branch taken
    }

    #[test]
    fn test_jmp_ret() {
        let mut vm: VM = VM::new();
        vm.registers.r7 = PC_START + 5; // Set return address in R7
        vm.registers.pc = PC_START;

        // JMPR R7 ; PC = R7
        // INSTRUCTION : OP | BaseR | 000000
        let instruction = 0b1100_000_111_000_000; // JMPR opcode
        vm.write_memory(PC_START as usize, instruction);
        execute_program(&mut vm);

        assert_eq!(vm.registers.pc, PC_START as u16 + 5);
    }

    #[test]
    fn test_lea() {
        let mut vm: VM = VM::new();
        vm.registers.pc = PC_START;

        // LEA R1, 5 ; R1 = PC + 5
        // INSTRUCTION : OP | DR | PCoffset9
        let instruction = 0b1110_001_000_000_101; // LEA opcode
        vm.write_memory(PC_START as usize, instruction);
        execute_program(&mut vm);

        // PC + 6 as pc is incremented by 1 after fetching the instruction
        assert_eq!(vm.registers.r1, PC_START as u16 + 6);
    }
}
