use super::vm::VM;

pub const MEMORY_SIZE: usize = u16::MAX as usize;

#[derive(Debug)]
pub enum OpCode {
    BR = 0, // branch.
    ADD,    // add .
    LD,     // load.
    ST,     // store.
    JSR,    // jump register.
    AND,    // bitwise and.
    LDR,    // load register.
    STR,    // store register.
    RTI,    // unused.
    NOT,    // bitwise not.
    LDI,    // load indirect.
    STI,    // store indirect.
    JMP,    // jump.
    RES,    // reserved (unused)
    LEA,    // load effective address.
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
        Some(OpCode::RTI) => rti(instr, vm),
        Some(OpCode::RES) => res(instr, vm),
        _ => {}
    }
}

pub fn add(instr: u16, vm: &mut VM) {
    let dr = ((instr >> 9) & 0b111) as u8;
    let sr1 = ((instr >> 6) & 0b111) as u8;
    let registers = &mut vm.registers;
    let sr1_value = registers.get(sr1).unwrap();
    if (instr >> 5) & 0b1 == 0 {
        let sr2 = (instr >> 3 & 0b111) as u8;
        let sr2_value = registers.get(sr2).unwrap();
        registers.update(dr, sr1_value + sr2_value);
    } else {
        let imm5 = (instr >> 5) & 0b11111;
        registers.update(dr, sr1_value + imm5);
    }
    registers.update_cond_register(dr)
}

pub fn and(instr: u16, vm: &mut VM) {
    let dr = ((instr >> 9) & 0b111) as u8;
    let sr1 = ((instr >> 6) & 0b111) as u8;
    let registers = &mut vm.registers;
    let sr1_value = registers.get(sr1).unwrap();
    if (instr >> 5) & 0b1 == 0 {
        let sr2 = (instr >> 3 & 0b111) as u8;
        let sr2_value = registers.get(sr2).unwrap();
        let sr1_and_sr2 = sr1_value & sr2_value;
        registers.update(dr, sr1_and_sr2);
    }
    registers.update_cond_register(dr)
}

pub fn not(instr: u16, vm: &mut VM) {
    let dr = ((instr >> 9) & 0b111) as u8;
    let sr = ((instr >> 6) & 0b111) as u8;
    let registers = &mut vm.registers;
    let sr_value = registers.get(sr).unwrap();

    registers.update(dr, !sr_value);
    registers.update_cond_register(dr)
}

//loop
pub fn br(instr: u16, vm: &mut VM) {
    let registers = &mut vm.registers;
    let pc_offset_9 = sign_extend(instr & 0x1fff, 9);

    let flag = (instr >> 9) & 0b111;
    if flag & registers.cond != 0 {
        let val = registers.pc as u32 + pc_offset_9 as u32;
        registers.update(8, val as u16);
    }
}

pub fn jmp(instr: u16, vm: &mut VM) {
    let base_r = ((instr >> 6) & 0b111) as u8;
    let registers = &mut vm.registers;
    let base_r_val = registers.get(base_r).unwrap();
    registers.update(8, base_r_val); // update program counter
}

pub fn jsr(instr: u16, vm: &mut VM) {
    let registers = &mut vm.registers;
    registers.update(7, registers.pc);
    if (instr >> 11) & 0b1 == 0 {
        let base_r = ((instr >> 6) & 0b111) as u8;
        let base_r_value = registers.get(base_r).unwrap();
        registers.update(8, base_r_value);
    } else {
        let val = registers.pc as u32 + sign_extend(instr & 0b11111111111, 11) as u32;
        registers.update(8, val as u16);
    }
}

pub fn ld(instr: u16, vm: &mut VM) {
    let dr = ((instr >> 9) & 0b111) as u8;
    let pc_offset_9 = sign_extend(instr & 0b111111111, 9);
    let mem_val = vm.read_memory((vm.registers.pc + pc_offset_9) as usize);
    vm.registers.update(dr, mem_val);
    vm.registers.update_cond_register(dr);
}

pub fn ldi(instr: u16, vm: &mut VM) {
    let dr = ((instr >> 9) & 0b111) as u8;
    let pc_offset_9 = sign_extend(instr & 0b111111111, 9);
    let pc = vm.registers.pc;
    let mem_addr = vm.read_memory(pc_offset_9.into()) + pc;
    vm.registers.update(dr, vm.read_memory(mem_addr.into()));
    vm.registers.update_cond_register(dr)
}

pub fn ldr(instr: u16, vm: &mut VM) {
    let dr = ((instr >> 9) & 0b111) as u8;
    let base_r = ((instr >> 6) & 0b111) as u8;
    let pc_offset_6 = sign_extend(instr & 0b111111, 6);
    let base_r_val = vm.registers.get(base_r).unwrap();
    let mem_val = vm.read_memory((base_r_val + pc_offset_6).into());
    vm.registers.update(dr, mem_val);
    vm.registers.update_cond_register(dr)
}

pub fn lea(instr: u16, vm: &mut VM) {
    let dr = ((instr >> 9) & 0b111) as u8;
    let pc_offset_9 = sign_extend(instr & 0b111111111, 9);
    let val = vm.registers.pc + pc_offset_9;
    vm.registers.update(dr, val);
    vm.registers.update_cond_register(dr)
}

pub fn st(instr: u16, vm: &mut VM) {
    let sr = ((instr >> 9) & 0b111) as u8;
    let pc_offset_9 = sign_extend(instr & 0b111111111, 9);
    let addr = vm.registers.pc + pc_offset_9;
    let sr_val = vm.registers.get(sr).unwrap();
    vm.write_memory(addr.into(), sr_val)
}

pub fn sti(instr: u16, vm: &mut VM) {
    let sr = ((instr >> 9) & 0b111) as u8;
    let pc_offset_9 = sign_extend(instr & 0b111111111, 9);
    let addr = vm.registers.pc + pc_offset_9;
    let sr_val = vm.registers.get(sr).unwrap();
    vm.write_memory(vm.read_memory(addr.into()).into(), sr_val)
}

pub fn str(instr: u16, vm: &mut VM) {
    let sr = ((instr >> 9) & 0b111) as u8;
    let base_r = ((instr >> 6) & 0b111) as u8;
    let pc_offset_6 = sign_extend(instr & 0b111111, 6);
    let str_val = vm.registers.get(sr).unwrap();
    let base_val = vm.registers.get(base_r).unwrap();
    vm.write_memory((pc_offset_6 + base_val).into(), str_val);
}

pub fn rti(_instr: u16, _vm: &mut VM) {}

pub fn res(_instr: u16, _vm: &mut VM) {}

fn sign_extend(mut x: u16, bit_count: u8) -> u16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        x |= 0xFFFF << bit_count;
    }
    x
}
