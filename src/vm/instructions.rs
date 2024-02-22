use super::vm::VM;

pub const MEMORY_SIZE: usize = u16::MAX as usize;

#[derive(Debug)]
pub enum OpCode {
    BR = 0, // branch
    ADD,    // add
    LD,     // load
    ST,     // store
    JSR,    // jump register
    AND,    // bitwise and
    LDR,    // load register
    STR,    // store register
    RTI,    // unused
    NOT,    // bitwise not
    LDI,    // load indirect
    STI,    // store indirect
    JMP,    // jump
    RES,    // reserved (unused)
    LEA,    // load effective address
}

// Each instruction is 16 bits long, with the left 4 bits storing the opcode.
// The rest of the bits are used to store the parameters.
// To extract left 4 bits out of the instruction, we'll use a right bit shift `>>`
// operator and shift to the right the first 4 bits 12 positions.
pub fn get_op_code(instruction: &u16) -> Option<OpCode> {
    match instruction >> 12 {
        0 => Some(OpCode::BR),
        1 => Some(OpCode::ADD),
        2 => Some(OpCode::LD),
        3 => Some(OpCode::ST),
        4 => Some(OpCode::JSR),
        5 => Some(OpCode::AND),
        6 => Some(OpCode::LDR),
        7 => Some(OpCode::STR),
        8 => Some(OpCode::RTI),
        9 => Some(OpCode::NOT),
        10 => Some(OpCode::LDI),
        11 => Some(OpCode::STI),
        12 => Some(OpCode::JMP),
        13 => Some(OpCode::RES),
        14 => Some(OpCode::LEA),
        _ => None,
    }
}

pub fn execute(instr: u16, vm: &mut VM) {
    let op_code = get_op_code(&instr);

    match op_code {
        Some(OpCode::ADD) => add(instr, vm),
        Some(OpCode::AND) => and(instr, vm),
        Some(OpCode::NOT) => not(instr, vm),
        Some(OpCode::BR) => br(instr, vm),
        Some(OpCode::JMP) => jmp(instr, vm),
        Some(OpCode::JSR) => jsr(instr, vm),
        Some(OpCode::LD) => ld(instr, vm),
        Some(OpCode::LDI) => ldi(instr, vm),
        Some(OpCode::LDR) => ldr(instr, vm),
        Some(OpCode::LEA) => lea(instr, vm),
        Some(OpCode::ST) => st(instr, vm),
        Some(OpCode::STI) => sti(instr, vm),
        Some(OpCode::STR) => str(instr, vm),
        _ => {}
    }
}

pub fn add(instr: u16, vm: &mut VM) {}

pub fn and(instr: u16, vm: &mut VM) {}

pub fn not(instr: u16, vm: &mut VM) {}

pub fn br(instr: u16, vm: &mut VM) {}

pub fn jmp(instr: u16, vm: &mut VM) {}

pub fn jsr(instr: u16, vm: &mut VM) {}

pub fn ld(instr: u16, vm: &mut VM) {}

pub fn ldi(instr: u16, vm: &mut VM) {}

pub fn ldr(instr: u16, vm: &mut VM) {}

pub fn lea(instr: u16, vm: &mut VM) {}

pub fn st(instr: u16, vm: &mut VM) {}

pub fn sti(instr: u16, vm: &mut VM) {}

pub fn str(instr: u16, vm: &mut VM) {}

