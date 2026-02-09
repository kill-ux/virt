use std::{fs::File, io::Read, process::exit};

use anyhow::Result;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[repr(u16)]
#[derive(Debug, FromPrimitive, Copy, Clone)]
enum INST {
    HALT = 0, SET = 1, PUSH = 2, POP = 3, EQ = 4, GT = 5, JMP = 6, JT = 7, JF = 8,
    ADD = 9, MULT = 10, MOD = 11, AND = 12, OR = 13, NOT = 14, RMEM = 15, WMEM = 16,
    CALL = 17, RET = 18, OUT = 19, IN = 20, NOOP = 21,
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
    while let Some(&data) = iter.next() {
        if let Some(inst) = INST::from_u16(data) {
            dbg!(&inst);
            match inst {
                INST::HALT => break,
                INST::NOOP => continue,
                INST::SET => {
                    if let Some(&reg) = iter.next() && let Some(&value) = iter.next()  {
                        memory[reg as usize] = value;
                    }
                }
                INST::PUSH => {
                    if let Some(&value) = iter.next()  {
                        stack.push(value);
                    }
                }
                INST::POP => {}
                INST::EQ => {}
                INST::GT => {}
                INST::JMP => {}
                INST::JT => {}
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
            }
        }

    }
   
    // while let Some(&data) = peeks.peek() {
        
    //     peeks.next();
    // }

    println!("Hello, world!");
    Ok(())
}
