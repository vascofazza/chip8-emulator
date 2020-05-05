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
        cpu.sp -= 1;
        let return_point = cpu.stack[cpu.sp];
        cpu.pc = return_point as usize;
    }

    fn jump(nnn: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                cpu.pc = nnn;
            }
        )
    }

    fn call(nnn: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                cpu.stack[cpu.sp] = cpu.pc;
                cpu.sp += 1;
                cpu.pc =nnn;
            }
        )
    }

    // LD Vx, byte
    // Set Vx = kk.
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
                cpu.registers[x] = (cpu.registers[x] as u16 + nn as u16) as u8;
            }
        )
    }

    fn mov(x: usize, y: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                cpu.registers[x] = cpu.registers[y];
            }
        )
    }

    fn or(x: usize, y: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                cpu.registers[x] |= cpu.registers[y];
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

    // ADD Vx, Vy
    // The values of Vx and Vy are added together. If the result is
    // greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
    // Only the lowest 8 bits of the result are kept, and stored in Vx.
    fn add(x: usize, y: usize) -> Box<dyn Fn(&mut CPU)>
    {
        Box::new(move |cpu: &mut cpu::CPU|
            {
                if let Some(t) = cpu.registers[x].checked_add(cpu.registers[y])
                {
                    cpu.registers[x] = t;
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
                    cpu.registers[x] = t;
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
                cpu.registers[15] = cpu.registers[x] >> 7;
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
                cpu.registers[15] = 0;
                for i in 0..n
                {
                    let ram_address = cpu.i + i;
                    let y = 64 * ((cpu.registers[y] as usize + i) % 32);
                    for b in 0..8 {
                        let x = (cpu.registers[x] as usize + b) % 64;


                        let vram_address = x + y; //TODO refactor
                        let current_pixel = cpu.vram[vram_address];
                        let mask: u8 = (1 << 7 - b) as u8;
                        let new_pixel = current_pixel ^ (cpu.ram[ram_address] & mask > 0) as u8;
                        cpu.vram[vram_address] = new_pixel;
                        cpu.registers[15] |= (current_pixel > 0 && new_pixel == 0) as u8;
                    }
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
                cpu.keypad_dst = x;
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
                cpu.registers[15] = if cpu.i > 0xFFF { 1 } else { 0 };
                cpu.i &=0xFFF;

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