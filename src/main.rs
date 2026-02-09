use anyhow::Result;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::{
    fs::File,
    io::{self, Read},
};

#[repr(u16)]
#[derive(Debug, FromPrimitive, Copy, Clone)]
enum INST {
    HALT = 0,
    SET = 1,
    PUSH = 2,
    POP = 3,
    EQ = 4,
    GT = 5,
    JMP = 6,
    JT = 7,
    JF = 8,
    ADD = 9,
    MULT = 10,
    MOD = 11,
    AND = 12,
    OR = 13,
    NOT = 14,
    RMEM = 15,
    WMEM = 16,
    CALL = 17,
    RET = 18,
    OUT = 19,
    IN = 20,
    NOOP = 21,
}

struct VM {
    memory: [u16; 32768],
    stack: Vec<u16>,
    ip: usize,
    program: Vec<u16>,
}

impl VM {
    fn read_reg(&self, r: u16) -> u16 {
        if r < 32768 {
            self.memory[r as usize]
        } else {
            r
        }
    }

    fn write_reg(&mut self, r: u16, value: u16) {
        if r < 32768 {
            self.memory[r as usize] = value;
        }
    }

    fn read_addr(&mut self, addr: u16) -> usize {
        if addr < 32767 {
            addr as usize
        } else {
            self.read_reg(addr) as usize
        }
    }

    fn next_u16(&mut self) -> u16 {
        let val = self.program[self.ip];
        self.ip += 1;
        val
    }

    fn run_instruction(&mut self, inst: INST) {
        match inst {
            INST::HALT => self.ip = self.program.len(),
            INST::NOOP => {}

            // 3-operand: reg, a, b
            INST::SET => {
                let r = self.next_u16();
                let val = self.next_u16();

                self.write_reg(r, val)
            }
            INST::EQ => {
                let r = self.next_u16();
                let a_raw = self.next_u16();
                let b_raw = self.next_u16();
                let a = self.read_reg(a_raw);
                let b = self.read_reg(b_raw);
                self.write_reg(r, if a == b { 1 } else { 0 });
            }
            INST::GT => {
                let r = self.next_u16();
                let a_raw = self.next_u16();
                let b_raw = self.next_u16();
                let a = self.read_reg(a_raw);
                let b = self.read_reg(b_raw);
                self.write_reg(r, if a > b { 1 } else { 0 });
            }
            INST::ADD => {
                let r = self.next_u16();
                let a_raw = self.next_u16();
                let b_raw = self.next_u16();
                let a = self.read_reg(a_raw);
                let b = self.read_reg(b_raw);
                self.write_reg(r, (a + b) % 32768);
            }

            INST::MULT => {
                let r = self.next_u16();
                let a_raw = self.next_u16();
                let b_raw = self.next_u16();
                let a = self.read_reg(a_raw);
                let b = self.read_reg(b_raw);
                self.write_reg(r, (a * b) % 32768);
            }
            INST::MOD => {
                let r = self.next_u16();
                let a_raw = self.next_u16();
                let b_raw = self.next_u16();
                let a = self.read_reg(a_raw);
                let b = self.read_reg(b_raw);
                self.write_reg(r, a % b);
            }
            INST::AND => {
                let r = self.next_u16();
                let a_raw = self.next_u16();
                let b_raw = self.next_u16();
                let a = self.read_reg(a_raw);
                let b = self.read_reg(b_raw);
                self.write_reg(r, a & b);
            }
            INST::OR => {
                let r = self.next_u16();
                let a_raw = self.next_u16();
                let b_raw = self.next_u16();
                let a = self.read_reg(a_raw);
                let b = self.read_reg(b_raw);
                self.write_reg(r, a | b);
            }
            INST::NOT => {
                let r = self.next_u16();
                let a_raw = self.next_u16();
                let a = self.read_reg(a_raw);
                self.write_reg(r, (!a) & 0x7FFF);
            }

            // 2-operand: a, b
            INST::RMEM => {
                let a = self.next_u16();
                let b = self.next_u16();
                self.write_reg(a, self.memory[self.read_reg(b) as usize]);
            }
            INST::WMEM => {
                let a = self.next_u16();
                let b = self.next_u16();
                let addr = self.read_reg(a) as usize;
                self.memory[addr] = self.read_reg(b);
            }

            // 1-operand: address
            INST::JMP => {
                let a_raw = self.next_u16();
                self.ip = self.read_addr(a_raw)
            }
            INST::CALL => {
                self.stack.push(self.ip as u16);
                let a_raw = self.next_u16();
                self.ip = self.read_addr(a_raw);
            }
            INST::RET => {
                self.ip = if let Some(pop) = self.stack.pop() {
                    pop as usize
                } else {
                    self.stack.len()
                }
            }
            INST::JT => {
                let a_raw = self.next_u16();
                let c = self.read_reg(a_raw);
                if c != 0 {
                    let b_raw = self.next_u16();
                    self.ip = self.read_addr(b_raw);
                } else {
                    self.ip += 1;
                }
            }
            INST::JF => {
                let a_raw = self.next_u16();
                let c = self.read_reg(a_raw);
                let b_raw = self.next_u16();
                if c == 0 {
                    self.ip = self.read_addr(b_raw);
                } else {
                    self.ip += 1;
                }
            }

            INST::PUSH => {
                let a_raw = self.next_u16();
                self.stack.push(self.read_reg(a_raw))
            }
            INST::POP => {
                if let Some(val) = self.stack.pop() {
                    let a_raw = self.next_u16();
                    self.write_reg(a_raw, val);
                }
            }
            INST::OUT => {
                let a_raw = self.next_u16();
                let a = self.read_reg(a_raw);
                println!("{}", a as u8 as char)
            }
            INST::IN => {
                let mut buf = String::new();
                let stdin = io::stdin();
                stdin.read_line(&mut buf).unwrap();

                let r = self.next_u16();
                let val = buf.trim().parse().unwrap();
                self.write_reg(r, val);

                dbg!(&self.memory[..10]);
            }
        }
    }
}

fn main() -> Result<()> {
    let mut fd = File::open("challenge.bin")?;
    let mut bytes = vec![];
    fd.read_to_end(&mut bytes)?;
    let program: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes(chunk.try_into().unwrap()))
        .collect();

    let mut vm = VM {
        memory: [0u16; 32768],
        stack: Vec::new(),
        ip: 0,
        program,
    };

    while vm.ip < vm.program.len() {
        let opcode = vm.program[vm.ip];
        vm.ip += 1;
        if let Some(inst) = INST::from_u16(opcode) {
            vm.run_instruction(inst);
        } else {
            eprintln!("Invalid opcode: {}", opcode);
            break;
        }
    }
    Ok(())
}
