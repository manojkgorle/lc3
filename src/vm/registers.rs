use crate::{ConditionFlag, PC_START};
use core::panic;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Registers {
    pub r0: u16,
    pub r1: u16,
    pub r2: u16,
    pub r3: u16,
    pub r4: u16,
    pub r5: u16,
    pub r6: u16,
    pub r7: u16,
    pub pc: u16,
    pub cond: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            pc: PC_START,
            cond: 0,
        }
    }
    pub fn get(&mut self, index: u8) -> Result<u16, String> {
        match index {
            0 => Ok(self.r0),
            1 => Ok(self.r1),
            2 => Ok(self.r2),
            3 => Ok(self.r3),
            4 => Ok(self.r4),
            5 => Ok(self.r5),
            6 => Ok(self.r6),
            7 => Ok(self.r7),
            8 => Ok(self.pc),
            9 => Ok(self.cond),
            _ => Err("get performed on index out of range".to_string()),
        }
    }
    pub fn update(&mut self, index: u8, value: u16) {
        match index {
            1 => {
                self.r1 = value;
            }
            2 => {
                self.r2 = value;
            }
            3 => {
                self.r3 = value;
            }
            4 => {
                self.r4 = value;
            }
            5 => {
                self.r5 = value;
            }
            6 => {
                self.r6 = value;
            }
            7 => {
                self.r7 = value;
            }
            8 => {
                self.pc = value;
            }
            9 => {
                self.cond = value;
            }
            _ => panic!("update register index out of range"),
        }
    }
    pub fn update_cond_register(&mut self, r: u8) {
        if self.get(r).unwrap() == 0 {
            self.update(9, ConditionFlag::ZRO as u16);
        } else if self.get(r).unwrap() >> 15 != 0 {
            self.update(9, ConditionFlag::NEG as u16);
        } else {
            self.update(9, ConditionFlag::POS as u16);
        }
    }
}
