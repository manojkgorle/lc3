pub mod vm;
pub const MEMORY_SIZE: usize = u16::MAX as usize;
pub const PC_START: u16 = 0x3000;
enum ConditionFlag {
    POS = 1 << 0, // Positive
    ZRO = 1 << 1, // Zero
    NEG = 1 << 2, // Negative
}
