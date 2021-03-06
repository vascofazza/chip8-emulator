use super::font::CHIP8_FONTSET;
use super::instruction::Instruction;
use std::fmt::{Debug, Formatter, Result};
use std::time::Instant;

const CHIP8_TIMER_DELAY: u128 = ((1. / 60. * 1000.) + 0.) as u128;
const CHIP8_RAM_SIZE: usize = 4096;
const CHIP8_VRAM_SIZE: usize = 64 * 32;
pub const CHIP8_START_POINT: usize = 0x200;

pub struct CPU {
    pub(crate) registers: [u8; 16], // last register contains carry flag
    pub(crate) i: usize,            //memory index
    pub(crate) pc: usize,
    pub(crate) sp: usize,

    pub(crate) ram: [u8; CHIP8_RAM_SIZE],
    pub vram: [u8; CHIP8_VRAM_SIZE],
    pub vram_flag: bool,
    pub(crate) stack: [usize; 16],

    pub(crate) keypad: [bool; 16],
    pub(crate) keypad_dst: usize,

    pub(crate) delay_timer: u8,
    pub(crate) sound_timer: u8,
    pub(crate) await_keypad: bool,
    timer_delay: Instant,
}

impl Debug for CPU {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("CPU")
            .field("pc", &self.pc)
            .field("sp", &self.sp)
            .field("registers", &self.registers)
            .field("delay_timer", &self.delay_timer)
            .field("sound_timer", &self.sound_timer)
            .field("await_keypad", &self.await_keypad)
            .finish()
    }
}

impl CPU {
    pub fn new() -> Self {
        //load font-set
        let mut ram = [0u8; CHIP8_RAM_SIZE];
        for i in 0..CHIP8_FONTSET.len() {
            ram[i] = CHIP8_FONTSET[i];
        }

        CPU {
            registers: [0u8; 16],
            i: 0,
            pc: CHIP8_START_POINT,
            sp: 0,
            ram,
            vram: [0u8; CHIP8_VRAM_SIZE],
            vram_flag: false,
            stack: [0; 16],
            keypad: [false; 16],
            keypad_dst: 0,
            delay_timer: 0,
            sound_timer: 0,
            await_keypad: false,
            timer_delay: Instant::now(),
        }
    }

    pub fn load_memory(&mut self, data: &[u8]) {
        for i in 0..data.len() {
            self.ram[i + CHIP8_START_POINT] = data[i];
        }
    }

    pub fn emulate_cycle(&mut self, keypad: [bool; 16]) -> CpuState {
        self.keypad = keypad;
        self.vram_flag = false;
        if self.await_keypad {
            for (i, &key) in keypad.iter().enumerate() {
                if key {
                    self.await_keypad = false;
                    self.registers[self.keypad_dst] = i as u8;
                    break;
                }
            }
        }
        if self.await_keypad {
            return CpuState {
                updated_vram: false,
                beep: false,
            };
        }
        //fetch
        let op_code = self.fetch_instruction();
        //println!("{:#04X}", op_code);
        //decode
        let instruction = Instruction::decode(&op_code);
        //execute

        if self.timer_delay.elapsed().as_millis() > CHIP8_TIMER_DELAY {
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }

            if self.sound_timer > 0 {
                self.sound_timer -= 1;
            }
            self.timer_delay = Instant::now();
        }
        instruction.execute(self);

        return CpuState {
            updated_vram: self.vram_flag,
            beep: self.sound_timer > 0,
        };
    }

    fn fetch_instruction(&mut self) -> u16 {
        let op_code = (self.ram[self.pc] as u16) << 8 | self.ram[self.pc + 1] as u16;
        self.pc += 2;
        op_code
    }
}

pub struct CpuState {
    pub(crate) updated_vram: bool,
    pub(crate) beep: bool,
}
