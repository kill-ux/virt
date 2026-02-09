use std::{fs::File, io::Read, process::exit};

use anyhow::Result;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

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

fn main() -> Result<()> {
    let mut fd = File::open("challenge.bin")?;
    let mut bytes = vec![];
    fd.read_to_end(&mut bytes)?;
    let buf: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes(chunk.try_into().unwrap()))
        .collect();

    let mut memory = [0u16; 32776];
    let mut stack = vec![];
    // let mut peeks = buf.into_iter().peekable();
    let mut iter = buf.iter();
    let mut ip = 0;
    loop {
        let data = buf[ip];
        if let Some(inst) = INST::from_u16(data) {
            dbg!(&inst);
            match inst {
                INST::HALT => break,
                INST::NOOP => ip += 1,
                INST::SET => {
                    ip += 1;
                    let reg = buf[ip];
                    ip += 1;
                    let value = buf[ip];
                    memory[reg as usize] = value;
                }
                INST::PUSH => {
                    ip += 1;
                    let value = buf[ip];
                    stack.push(value);
                }
                INST::POP => {
                    ip += 1;
                    let reg = buf[ip];
                    if let Some(value) = stack.pop() {
                        memory[reg as usize] = value;
                    }
                }
                INST::EQ => {
                    ip += 1;
                    let reg = buf[ip];
                    ip += 1;
                    let a = buf[ip];
                    ip += 1;
                    let b = buf[ip];

                    memory[reg as usize] = if a == b { 1 } else { 0 };
                }
                INST::GT => {
                    ip += 1;
                    let reg = buf[ip];
                    ip += 1;
                    let a = buf[ip];
                    ip += 1;
                    let b = buf[ip];
                    memory[reg as usize] = if a > b { 1 } else { 0 };
                }
                INST::JMP => {
                    ip += 1;
                    let add = buf[ip] as usize;
                    if add < 32767 {
                        ip = add
                    } else {
                        ip = memory[add] as usize
                    }
                }
                INST::JT => {
                    ip += 1;
                    let add1 = buf[ip] as usize;
                    ip += 1;
                    let add2 = buf[ip] as usize;


                    if add1 < 32767 && memory[add1] != 0 {
                        ip = add1
                    } else {
                        ip = memory[add1] as usize
                    }


                    if memory[add1] != 0 {
                        ip = num
                    } else {
                        if add2 > 32767 {
                            ip = add2
                        } else {
                            ip = memory[add2] as usize
                        }
                    }
                }
                INST::JF => {}
                INST::ADD => {}
                INST::MULT => {}
                INST::MOD => {}
                INST::AND => {}
                INST::OR => {}
                INST::NOT => {}
                INST::RMEM => {}
                INST::WMEM => {}
                INST::CALL => {}
                INST::RET => {}
                INST::OUT => {}
                INST::IN => {}
            };
        }
    }

    println!("Hello, world!");
    Ok(())
}
