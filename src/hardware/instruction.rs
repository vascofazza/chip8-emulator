use super::cpu;
use std::fmt::Debug;
use crate::hardware::cpu::CPU;
use rand::Rng;

pub struct Instruction
{
    op_code: u16,
    execution: Execution
}

impl Instruction{
    pub fn decode(op_code: &u16) -> Instruction
    {
        let decoded_op = (
            ((op_code & 0xF000) >> 12) as u8,
            ((op_code & 0x0F00) >> 8) as u8,
            ((op_code & 0x00F0) >> 4) as u8,
            (op_code & 0x000F) as u8);

        let nnn = (op_code & 0x0FFF) as usize;
        let kk = (op_code & 0x00FF) as u8;
        let x = decoded_op.1 as usize;
        let y = decoded_op.2 as usize;
        let n = decoded_op.3 as usize;

        let execution: Box<dyn Fn(&mut CPU)> = match decoded_op {
            (0x00, 0x00, 0x0e, 0x00) => Box::new(Instruction::cls),  // CLS: Clear the display.
            (0x00, 0x00, 0x0e, 0x0e) => Box::new(Instruction::ret),
            (0x01, _, _, _) => Instruction::jump(nnn),
            (0x02, _, _, _) => Instruction::call(nnn),
            (0x03, _, _, _) => Instruction::beq(x, kk),
            (0x04, _, _, _) => Instruction::bne(x, kk),
            (0x05, _, _, 0x00) => Instruction::beqr(x, y),
            (0x06, _, _, _) => Instruction::load_i(x, kk),
            (0x07, _, _, _) => Instruction::add_i(x, kk),
            (0x08, _, _, 0x00) => Instruction::mov(x, y),
            (0x08, _, _, 0x01) => Instruction::or(x, y),
            (0x08, _, _, 0x02) => Instruction::and(x, y),
            (0x08, _, _, 0x03) => Instruction::xor(x, y),
            (0x08, _, _, 0x04) => Instruction::add(x, y),
            (0x08, _, _, 0x05) => Instruction::sub(x, y),
            (0x08, _, _, 0x06) => Instruction::slr(x),
            (0x08, _, _, 0x07) => Instruction::sub_inv(x, y),
            (0x08, _, _, 0x0e) => Instruction::sll(x),
            (0x09, _, _, 0x00) => Instruction::bner(x, y),
            (0x0a, _, _, _) => Instruction::set_addr(nnn),
            (0x0b, _, _, _) => Instruction::branch(nnn),
            (0x0c, _, _, _) => Instruction::rand(x, kk),
            (0x0d, _, _, _) => Instruction::draw(x, y, n),
            (0x0e, _, 0x09, 0x0e) => Instruction::key_pressed(x),
            (0x0e, _, 0x0a, 0x01) => Instruction::key_released(x),
            (0x0f, _, 0x00, 0x07) => Instruction::get_delay(x),
            (0x0f, _, 0x00, 0x0a) => Instruction::get_key(x),
            (0x0f, _, 0x01, 0x05) => Instruction::delay_timer(x),
            (0x0f, _, 0x01, 0x08) => Instruction::sound_timer(x),
            (0x0f, _, 0x01, 0x0e) => Instruction::inc_mem(x),
            (0x0f, _, 0x02, 0x09) => Instruction::load_sprite(x),
            (0x0f, _, 0x03, 0x03) => Instruction::BCD(x),
            (0x0f, _, 0x05, 0x05) => Instruction::reg_dump(x),
            (0x0f, _, 0x06, 0x05) => Instruction::reg_load(x),
            _ => Box::new(Instruction::unknown(op_code)),

        };

        Instruction
        {
            op_code: op_code.clone(),
            execution
        }
    }

    pub fn execute(&self, cpu: &mut cpu::CPU)
    {
        (self.execution)(cpu);
    }

    fn cls(cpu: &mut cpu::CPU)
    {
        for i in 0..cpu.vram.len()
        {
            cpu.vram[i] = 0;
        }
        cpu.video_flag = true;
    }

    fn ret(cpu: &mut cpu::CPU)
    {
        cpu.sp -= 1; //must not overflow
        let return_point = cpu.stack[cpu.sp];
        cpu.pc = /*cpu::CHIP8_START_POINT + */return_point as usize;
    }

    fn jump(nnn: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                cpu.pc = /*cpu::CHIP8_START_POINT +*/nnn;
            }
        )
    }

    fn call(nnn: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                cpu.stack[cpu.sp] = cpu.pc;
                cpu.sp += 1;
                cpu.pc = /*cpu::CHIP8_START_POINT + */nnn;
            }
        )
    }

    fn load_i(x: usize, nn: u8) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                cpu.registers[x] = nn;
            }
        )
    }

    fn beq(x: usize, nn: u8) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                if cpu.registers[x] == nn
                {
                    cpu.pc += 2; //skip
                }
            }
        )
    }

    fn bne(x: usize, nn: u8) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                if cpu.registers[x] != nn
                {
                    cpu.pc += 2; //skip
                }
            }
        )
    }

    fn beqr(x: usize, y: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                if cpu.registers[x] == cpu.registers[y]
                {
                    cpu.pc += 2; //skip
                }
            }
        )
    }

    fn bner(x: usize, y: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                if cpu.registers[x] != cpu.registers[y]
                {
                    cpu.pc += 2; //skip
                }
            }
        )
    }

    fn add_i(x: usize, nn: u8) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                cpu.registers[x] += nn;
            }
        )
    }

    fn mov(x: usize, y: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                cpu.registers[x] == cpu.registers[y];
            }
        )
    }

    fn or(x: usize, y: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                cpu.registers[x] != cpu.registers[y];
            }
        )
    }

    fn and(x: usize, y: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                cpu.registers[x] &= cpu.registers[y];
            }
        )
    }

    fn xor(x: usize, y: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                cpu.registers[x] ^= cpu.registers[y];
            }
        )
    }

    fn add(x: usize, y: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                if let Some(t) = cpu.registers[x].checked_add(cpu.registers[y])
                {
                    cpu.registers[x] += cpu.registers[y];
                    cpu.registers[15] = 0;
                }
                else
                {
                    cpu.registers[x] = (cpu.registers[x] as u16 + cpu.registers[y] as u16) as u8;
                    cpu.registers[15] = 1;
                }
            }
        )
    }

    fn sub(x: usize, y: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                if let Some(t) = cpu.registers[x].checked_sub(cpu.registers[y])
                {
                    cpu.registers[x] += cpu.registers[y];
                    cpu.registers[15] = 1;
                }
                else
                {
                    cpu.registers[x] = (cpu.registers[x] as i16 - cpu.registers[y] as i16) as u8;
                    cpu.registers[15] = 0;
                }
            }
        )
    }

    fn slr(x: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                cpu.registers[15] = cpu.registers[x] & 1;
                cpu.registers[x] >>= 1;
            }
        )
    }

    fn sub_inv(x: usize, y: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                if let Some(t) = cpu.registers[y].checked_sub(cpu.registers[x])
                {
                    cpu.registers[x] = cpu.registers[y] - cpu.registers[x];
                    cpu.registers[15] = 1;
                }
                else
                {
                    cpu.registers[x] = (cpu.registers[y] as i16 - cpu.registers[x] as i16) as u8;
                    cpu.registers[15] = 0;
                }
            }
        )
    }

    fn sll(x: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                cpu.registers[15] = cpu.registers[x] & 0x80u8;
                cpu.registers[x] <<= 1;
            }
        )
    }

    fn set_addr(nnn: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                cpu.i = nnn;
            }
        )
    }

    fn branch(nnn: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                cpu.pc = cpu.registers[0] as usize + nnn;
            }
        )
    }

    fn rand(x: usize, nn: u8) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                let mut rng = rand::thread_rng();
                let rand: u8 = rng.gen();
                cpu.registers[x] = rand & nn;
            }
        )
    }

    // DRW Vx, Vy, n
    // The interpreter reads n bytes from memory, starting at the address
    // stored in I. These bytes are then displayed as sprites on screen at
    // coordinates (Vx, Vy). Sprites are XORed onto the existing screen.
    // If this causes any pixels to be erased, VF is set to 1, otherwise
    // it is set to 0. If the sprite is positioned so part of it is outside
    // the coordinates of the display, it wraps around to the opposite side
    // of the screen.
    fn draw(x: usize, y: usize, n: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                for i in 0..n
                {
                    let ram_address = cpu.i + i;
                    let vram_address = 32 * (cpu.registers[x] as usize % 32) + (cpu.registers[y] as usize % 64); //TODO refactor
                    let current_display = cpu.vram[vram_address];
                    let new_display = current_display ^ cpu.ram[ram_address];
                    cpu.registers[15] = (new_display == 0) as u8;
                    cpu.vram[vram_address] = new_display;
                    //TODO add support for colors
                }
                cpu.video_flag = true;
            }
        )
    }

    fn key_pressed(x: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                if cpu.keypad[cpu.registers[x] as usize]
                {
                    cpu.pc += 2;
                }
            }
        )
    }

    fn key_released(x: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                if !cpu.keypad[cpu.registers[x] as usize]
                {
                    cpu.pc += 2;
                }
            }
        )
    }

    fn get_delay(x: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                cpu.registers[x] = cpu.delay_timer;
            }
        )
    }

    fn get_key(x: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                cpu.keypad_dst = x as u8;
                cpu.await_keypad = true;
            }
        )
    }

    fn delay_timer(x: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                cpu.delay_timer = x as u8;
            }
        )
    }

    fn sound_timer(x: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                cpu.sound_timer = x as u8;
            }
        )
    }

    fn inc_mem(x: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                cpu.i += cpu.registers[x] as usize;
                cpu.registers[15] = if cpu.i > 0x0F00 { 1 } else { 0 };
            }
        )
    }

    fn load_sprite(x: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                cpu.i = (cpu.registers[x] as usize) * 5;
            }
        )
    }

    fn BCD(x: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                cpu.ram[cpu.i] = cpu.registers[x] / 100;
                cpu.ram[cpu.i + 1] = (cpu.registers[x] % 100) / 10;
                cpu.ram[cpu.i + 2] = cpu.registers[x] % 10;
            }
        )
    }

    fn reg_dump(x: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                for i in 0..x + 1 {
                    cpu.ram[cpu.i + i] = cpu.registers[i];
                }
            }
        )
    }

    fn reg_load(x: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                for i in 0..x + 1 {
                    cpu.registers[i] = cpu.ram[cpu.i + i];
                }
            }
        )
    }

    fn nop(cpu: &mut cpu::CPU)
    {

    }

    fn unknown(op_code: &u16) -> fn(&mut cpu::CPU)
    {
        panic!("Unknown OP_CODE: {:#04X}", op_code);
        //Instruction::nop
    }
}

type Execution = Box<dyn Fn(&mut CPU)>;