use super::cpu;
use std::fmt::Debug;
use crate::hardware::cpu::CPU;

pub struct Instruction
{
    op_code: u16,
    execution: Execution
}

/*trait Instruction: Debug
{
    fn get_op_code() -> u16;
    fn execute(cpu: &CPU);
}
 */

//struct Jump;

/*impl Instruction for Jump
{
    fn get_op_code() -> u16 {
        0u16
    }

    fn execute(cpu: &CPU) {
        unimplemented!()
    }
}
 */

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

        let execution = match decoded_op {
            (0x00, 0x00, 0x0e, 0x00) => Instruction::cls,  // CLS: Clear the display.
            (0x00, 0x00, 0x0e, 0x0e) => Instruction::ret,
            (0x01, _, _, _) => Instruction::jump(nnn),
            (0x02, _, _, _) => Instruction::call(nnn),
            /*(0x03, _, _, _) => |cpu: CPU| {},
            (0x04, _, _, _) => |cpu: CPU| {},
            (0x05, _, _, 0x00) => |cpu: CPU| {},
            (0x06, _, _, _) => |cpu: CPU| {},
            (0x07, _, _, _) => |cpu: CPU| {},
            (0x08, _, _, 0x00) => |cpu: CPU| {},
            (0x08, _, _, 0x01) => |cpu: CPU| {},
            (0x08, _, _, 0x02) => |cpu: CPU| {},
            (0x08, _, _, 0x03) => |cpu: CPU| {},
            (0x08, _, _, 0x04) => |cpu: CPU| {},
            (0x08, _, _, 0x05) => |cpu: CPU| {},
            (0x08, _, _, 0x06) => |cpu: CPU| {},
            (0x08, _, _, 0x07) => |cpu: CPU| {},
            (0x08, _, _, 0x0e) => |cpu: CPU| {},
            (0x09, _, _, 0x00) => |cpu: CPU| {},
            (0x0a, _, _, _) => |cpu: CPU| {},
            (0x0b, _, _, _) => |cpu: CPU| {},
            (0x0c, _, _, _) => |cpu: CPU| {},
            (0x0d, _, _, _) => |cpu: CPU| {},
            (0x0e, _, 0x09, 0x0e) => |cpu: CPU| {},
            (0x0e, _, 0x0a, 0x01) => |cpu: CPU| {},
            (0x0f, _, 0x00, 0x07) => |cpu: CPU| {},
            (0x0f, _, 0x00, 0x0a) => |cpu: CPU| {},
            (0x0f, _, 0x01, 0x05) => |cpu: CPU| {},
            (0x0f, _, 0x01, 0x08) => |cpu: CPU| {},
            (0x0f, _, 0x01, 0x0e) => |cpu: CPU| {},
            (0x0f, _, 0x02, 0x09) => |cpu: CPU| {},
            (0x0f, _, 0x03, 0x03) => |cpu: CPU| {},
            (0x0f, _, 0x05, 0x05) => |cpu: CPU| {},
            (0x0f, _, 0x06, 0x05) => |cpu: CPU| {},
            */
            _ => Instruction::unknown(op_code),

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
        let return_point = cpu.stack[cpu.sp];
        cpu.sp += 1;
        cpu.pc = cpu::CHIP8_START_POINT + return_point as usize;
    }

    fn jump(nnn: usize) -> fn(&mut cpu::CPU)
    {
        fn a (cpu: &mut cpu::CPU)
            {
                cpu.pc = cpu::CHIP8_START_POINT + nnn;
            }
        a
    }

    fn call(nnn: usize) -> fn(&mut cpu::CPU)
    {
        fn a (cpu: &mut cpu::CPU)
            {
                cpu.stack[cpu.sp] = cpu.pc;
                cpu.sp -= 1;
                cpu.pc = cpu::CHIP8_START_POINT + nnn;
            }
        a
    }

    fn unknown(op_code: &u16) -> fn(&mut cpu::CPU)
    {
        panic!("Unknown OP_CODE: {}", op_code);
    }
}

type Execution = fn(&mut cpu::CPU);