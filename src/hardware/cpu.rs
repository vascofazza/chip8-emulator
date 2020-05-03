use super::font::CHIP8_FONTSET;
use super::instruction::Instruction;

const CHIP8_RAM_SIZE: usize = 4096;
const CHIP8_VRAM_SIZE: usize = 4096;
pub const CHIP8_START_POINT: usize = 0x200;

pub struct CPU
{
    registers: [u8; 16], // last register contains carry flag
    i: u8, //register index
pub(crate) pc: usize,
    pub(crate) sp: usize,

    ram: [u8; CHIP8_RAM_SIZE],
    pub vram: [u8; CHIP8_VRAM_SIZE],
    pub video_flag: bool,
    pub(crate) stack: [usize; 16],

    delay_timer: u8,
    sound_timer: u8,
}

impl CPU{
    pub fn new() -> Self
    {
        //load font-set
        let mut ram = [0u8; CHIP8_RAM_SIZE];
        for i in 0..CHIP8_FONTSET.len()
        {
            ram[i] = CHIP8_FONTSET[i];
        }

        CPU
        {
            registers: [0u8; 16],
            i: 0,
            pc: CHIP8_START_POINT,
            sp: 0,
            ram: [0u8; CHIP8_RAM_SIZE],
            vram: [0u8; CHIP8_VRAM_SIZE],
            video_flag: false,
            stack: [0; 16],
            delay_timer: 0,
            sound_timer: 0
        }
    }

    pub fn load_memory(&mut self, data: &[u8])
    {
        for i in 0..data.len()
        {
            self.ram[i + CHIP8_START_POINT] = data[i];
        }
    }

    pub fn emulate_cycle(&mut self)
    {
        //fetch
        let op_code = self.fetch_instruction();
        let instruction = Instruction::decode(&op_code);

    }

    fn fetch_instruction(&mut self) -> u16
    {
        let op_code = (self.ram[self.pc] as u16) << 8 | self.ram[self.pc + 1] as u16;
        self.pc += 2;
        op_code
    }
}