pub mod instructions;
pub mod registers;
pub mod vm;

use crate::MEMORY_SIZE;
use instructions::execute;
use vm::VM;

pub fn execute_program(vm: &mut VM) {
    while vm.registers.pc < MEMORY_SIZE as u16 {
        let instr = vm.read_memory(vm.registers.pc.into());
        if instr == 0 {
            break;
        }
        vm.registers.pc += 1;
        execute(instr, vm)
    }
}
