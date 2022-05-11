use rand::random;

use crate::display::buffer::DisplayBuffer;
use crate::display::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::keypad::KeypadState;
use crate::memory::{Memory, USER_SPACE_STR};
use crate::opcode::{Instruction, Opcode};
use crate::register_set::RegisterSet;
use crate::rom::Rom;
use crate::stack::Stack;

pub const CLOCK_RATE: f32 = 600.0;

pub struct CycleOutput {
    pub beep: bool,
    pub display_buffer: DisplayBuffer,
    pub display_update: bool,
}

pub struct Cpu {
    /// System available memory.
    pub(crate) ram: Memory,
    /// Program Counter
    pub(crate) pc: u16,
    /// Index reigster
    pub(crate) i: u16,
    /// Stack of 16 8-bit spaces
    pub(crate) stack: Stack,
    /// Stack Pointer
    pub(crate) sp: u16,
    /// General Purpose Variable Registers
    ///
    /// 16 8-bit variable registers numbered from 0 through F.
    pub(crate) registers: RegisterSet,
    /// Delay Timer (DT)
    pub(crate) dt: u8,
    /// Sound Time (ST)
    pub(crate) st: u8,
    /// Display Buffer to hold bytes mapped to output display
    pub(crate) display_buffer: DisplayBuffer,
    /// Keys pressed at the moment of the cycle execution
    pub(crate) keypad_state: KeypadState,
    /// Stores the a key to expect the user to input if `Some`
    pub(crate) keypad_await: Option<usize>,
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu {
    /// Initializes a new CHIP-8 CPU instance with default memory layout
    /// (fonts loaded), an empty stack and Program Counter (PC) pointing
    /// to memory's user space (0x200).
    pub fn new() -> Self {
        Self {
            ram: Memory::default(),
            pc: USER_SPACE_STR as u16,
            registers: RegisterSet::default(),
            i: 0x0000,
            stack: Stack::default(),
            sp: 0,
            dt: 0,
            st: 0,
            display_buffer: DisplayBuffer::default(),
            keypad_state: KeypadState::new(),
            keypad_await: None,
        }
    }

    /// Loads ROM bytes into memory
    pub fn load(&mut self, rom: Rom) {
        self.ram.load(rom.bytes());
    }

    /// Runs a CPU Cycle.
    ///
    /// First fetches the next instruction pointed out by the PC, then decodes
    /// the instruction and finally executes the instruction.
    pub fn cycle(&mut self, keypad_state: KeypadState) -> CycleOutput {
        let mut display_update = false;

        self.keypad_state = keypad_state;

        if let Some(register) = self.keypad_await {
            for index in 0..16_usize {
                if keypad_state[index] {
                    self.keypad_await = None;
                    self.registers[register] = index as u8;
                    break;
                }
            }
        } else {
            if self.dt > 0 {
                self.dt -= 1;
            }

            if self.st > 0 {
                self.st -= 1;
            }

            let opcode = &self.fetch_opcode();
            let instr = opcode.decode();

            if matches!(instr, Instruction::Cls) || matches!(instr, Instruction::Draw(_, _, _)) {
                display_update = true;
            }

            self.execute(instr);

            println!(
                "============================================================================================================================",
            );
            println!(
                "PC: {}\nOPCODE: {} ({})\nREGISTERS: {}\nIP:{}\tSP:{}\nTIMERS: DT:{}\tST:{}\nKB: {}",
                self.pc, opcode, instr, self.registers, self.i, self.sp, self.dt, self.st, self.keypad_state
            );
            println!(
                "============================================================================================================================",
            );
        }

        CycleOutput {
            beep: self.st > 0,
            display_buffer: self.display_buffer.clone(),
            display_update,
        }
    }

    pub fn load_and_exec(&mut self, opcode: u16) {
        self.load(vec![(opcode >> 8) as u8, (opcode & 0xff) as u8].into());
        self.cycle(KeypadState::new());
    }

    /// Executes the provided instruction
    pub fn execute(&mut self, instr: Instruction) {
        match instr {
            Instruction::Cls => self.display_buffer.reset(),
            Instruction::Ret => {
                self.pc = self.stack.pop();

                if self.sp > 0 {
                    self.sp -= 1;
                }
            }
            Instruction::SysAddr => println!("WARN: COSMAC VIP Only Instruction. Skipping."),
            Instruction::Jump(address) => self.pc = address,
            Instruction::CallSubroutine(address) => {
                self.sp += 1;
                self.stack.push(self.pc);
                self.pc = address;
            }
            Instruction::Rand(vx, kk) => self.registers[vx] = kk & random::<u8>(),
            Instruction::CondEq(vx, kk) => {
                if self.registers[vx] == kk {
                    self.pc += 2;
                }
            }
            Instruction::CondNotEq(vx, kk) => {
                if self.registers[vx] != kk {
                    self.pc += 2;
                }
            }
            Instruction::CondEqVxVy(vx, vy) => {
                if self.registers[vx] == self.registers[vy] {
                    self.pc += 2;
                }
            }
            Instruction::ConstAssignVxToKk(vx, kk) => self.registers[vx] = kk,
            Instruction::ConstAddVxToKk(vx, kk) => {
                self.registers[vx] = kk.wrapping_add(self.registers[vx])
            }
            Instruction::AssignVxToVy(vx, vy) => self.registers[vx] = self.registers[vy],
            Instruction::BitOpOr(vx, vy) => {
                self.registers[vx] = self.registers[vx] | self.registers[vy];
                self.pc += 2;
            }
            Instruction::BitOpAnd(vx, vy) => {
                self.registers[vx] = self.registers[vx] & self.registers[vy];
                self.pc += 2;
            }
            Instruction::BitOpXor(vx, vy) => {
                self.registers[vx] = self.registers[vx] ^ self.registers[vy]
            }
            Instruction::MathAdd(vx, vy) => {
                let (result, overflows) = self.registers[vx].overflowing_add(self.registers[vy]);

                self.registers[0xF] = overflows as u8;
                self.registers[vx] = result;
            }
            Instruction::MathSub(vx, vy) => {
                let (result, overflows) = self.registers[vx].overflowing_sub(self.registers[vy]);

                self.registers[0xF] = overflows as u8;
                self.registers[vx] = result;
            }
            Instruction::BitOpShr(vx) => self.registers[vx] = self.registers[vx] >> 1,
            Instruction::MathSubVyVx(vx, vy) => self.registers[vx] = self.registers[vx] - vy as u8,
            Instruction::BitOpShl(vx) => {
                self.registers[vx] = self.registers[vx] << 1;
            }
            Instruction::CondVxNotEqVy(vx, vy) => {
                if self.registers[vx] != self.registers[vy] {
                    self.pc += 2;
                }
            }
            Instruction::Mem(nnn) => {
                self.i = nnn;
            }
            Instruction::Draw(vx, vy, n) => {
                // Set the X coordinate to the value in VX modulo 64 (or,
                // equivalently, VX & 63, where & is the binary AND operation)
                let x = self.registers[vx] & 63;
                // Set the Y coordinate to the value in VY modulo 32
                // (or VY & 31)
                let y = self.registers[vy] & 31;

                // Set VF to 0
                self.registers[0x0F] = 0x0;

                for row in 0..n {
                    let bits = self.ram[(self.i + row as u16) as usize];
                    let this_y = (y + row as u8) as u32 % SCREEN_HEIGHT;

                    for col in 0..8 {
                        let this_x = (x + col as u8) as u32 % SCREEN_WIDTH;
                        let current_color =
                            self.display_buffer[(this_y * SCREEN_WIDTH + this_x) as usize];
                        let mask = 0x01 << 7 - col;
                        let color = bits & mask;

                        if color > 0 {
                            if current_color > 0 {
                                self.display_buffer[(this_y * SCREEN_WIDTH + this_x) as usize] = 0;
                                self.registers[0x0F] = 1;
                            } else {
                                self.display_buffer[(this_y * SCREEN_WIDTH + this_x) as usize] = 1;
                            }
                        }

                        if this_x == SCREEN_WIDTH - 1 {
                            break;
                        }
                    }

                    if this_y == SCREEN_HEIGHT - 1 {
                        break;
                    }
                }
            }
            Instruction::SetDtEqToVx(vx) => self.dt = self.registers[vx],
            Instruction::SetStEqToVx(vx) => {
                self.st = self.registers[vx];
                self.pc += 2;
            }
            Instruction::SetIEqToIPlusVx(vx) => {
                self.i = self.i + self.registers[vx] as u16;
            }
            Instruction::SetIEqToVx(vx) => {
                self.i = self.registers[vx] as u16 * 0x05;
            }
            Instruction::StoreBinaryCodedDecimal(vx) => {
                let value = self.registers[vx];
                let h = value / 100;
                let t = (value - h * 100) / 10;
                let o = value - h * 100 - t * 10;
                let i = self.i as usize;

                self.ram[i] = h;
                self.ram[i + 1] = t;
                self.ram[i + 2] = o;
            }
            Instruction::SetRegsInI(vx) => {
                for reg in 0..vx + 1 {
                    self.ram[self.i as usize + reg] = self.registers[reg];
                }
            }
            Instruction::GetRegsInI(vx) => {
                for reg in 0..vx + 1 {
                    self.registers[reg] = self.ram[self.i as usize + reg];
                }
            }
            Instruction::SetVxEqToDt(vx) => {
                self.registers[vx] = self.dt;
            }
            Instruction::WaitKeyPressAndStoreOnVx(vx) => {
                self.keypad_await = Some(vx);
                self.pc += 2;
            }
            Instruction::SkipIfKeyPressed(vx) => {
                if self.keypad_state[self.registers[vx] as usize] {
                    self.pc += 4;
                }

                self.pc += 2;
            }
            Instruction::KeyOpVxNotPressed(vx) => {
                if !self.keypad_state[self.registers[vx] as usize] {
                    self.pc += 4;
                }

                self.pc += 2;
            }
            Instruction::JumpPcV0(nnn) => self.pc = nnn + (self.registers[0x0] as u16),
            Instruction::Unknown => {
                self.pc += 2;
            }
        }
    }

    /// Fetches an OpCode from memory based on Program Counter (PC) and then
    /// updates the PC position 2 points ahead.
    ///
    /// # Building the OpCode
    ///
    /// To build an `OpCode` two bytes are taken from the memory and merged to
    /// build a 16-bit `OpCode`.
    ///
    /// 1. The value at memory address pointed by the PC is shifted 8-bits
    /// to the left and stored in a 16-bit variable.
    ///
    /// 2. The value at memory address pointed by the PC + 1 is merged with
    /// the value created at step 1 using the OR operator.
    fn fetch_opcode(&mut self) -> Opcode {
        let pc = self.pc as usize;
        let hexa: u16 = (self.ram[pc] as u16) << 8 | (self.ram[pc + 1] as u16);

        self.pc += 2;
        Opcode::from(hexa)
    }
}

#[cfg(test)]
mod tests {
    use crate::display::buffer::DisplayBuffer;
    use crate::keypad::{Keypad, KeypadState};
    use crate::memory::{Memory, USER_SPACE_STR};
    use crate::register_set::RegisterSet;
    use crate::stack::Stack;

    use super::Cpu;

    #[test]
    fn new_instance() {
        let cpu = Cpu::new();

        assert_eq!(cpu.ram, Memory::default());
        assert_eq!(cpu.pc, USER_SPACE_STR as u16);
        assert_eq!(cpu.i, 0);
        assert_eq!(cpu.stack, Stack::default());
        assert_eq!(cpu.sp, 0);
        assert_eq!(cpu.registers, RegisterSet::default());
        assert_eq!(cpu.dt, 0);
        assert_eq!(cpu.st, 0);
        assert_eq!(cpu.display_buffer, DisplayBuffer::default());
        assert_eq!(cpu.keypad_state, KeypadState::new());
        assert_eq!(cpu.keypad_await, None);
    }

    #[test]
    fn load_rom_into_memory() {
        let mut cpu = Cpu::new();
        let rom = vec![0x001, 0x002, 0x003, 0x004];

        cpu.load(rom.into());

        assert_eq!(cpu.ram[USER_SPACE_STR], 0x001);
        assert_eq!(cpu.ram[USER_SPACE_STR + 1], 0x002);
        assert_eq!(cpu.ram[USER_SPACE_STR + 2], 0x003);
        assert_eq!(cpu.ram[USER_SPACE_STR + 3], 0x004);
    }

    #[test]
    fn instr_cls() {
        let mut cpu = Cpu::new();
        let initial_display_buffer = cpu.display_buffer;
        let rom = vec![
            // Writes to Display Buffer
            0xDF, 0xB8, // Clears Display Buffer
            0x00, 0xE0,
        ];

        cpu.load(rom.into());

        // Runs first cycle of CPU with 0xDFB8
        cpu.cycle(KeypadState::new());

        let written_display_buffer = cpu.display_buffer;

        // Runs second cycle of CPU with 0x00E0
        cpu.cycle(KeypadState::new());

        let cleared_display_buffer = cpu.display_buffer;

        assert!(
            initial_display_buffer.0.iter().all(|x| *x == 0),
            "Initially all bytes are 0"
        );

        assert_ne!(
            written_display_buffer.0.iter().fold(0, |acc, x| acc + x),
            0,
            "Bytes were written"
        );

        assert!(
            cleared_display_buffer.0.iter().all(|x| *x == 0),
            "Bytes were cleared"
        );
    }

    #[test]
    fn instr_ret() {
        let mut cpu = Cpu::new();

        cpu.stack.push(0x1234);
        cpu.sp = 0x0003;
        cpu.load_and_exec(0x00EE);

        assert_eq!(cpu.sp, 2);
        assert_eq!(cpu.pc, 0x1234);
    }

    #[test]
    fn instr_jump() {
        let mut cpu = Cpu::new();

        cpu.load_and_exec(0x12CD);

        assert_eq!(cpu.pc, 0x02CD, "Jump to address on NNN");
    }

    #[test]
    fn instr_call_subroutine() {
        let mut cpu = Cpu::new();

        cpu.load_and_exec(0x2123);

        assert_eq!(cpu.pc, 0x0123, "The value of PC is the one set by NNN");
        assert_eq!(cpu.sp, 1, "Stack Pointer is back to 1");
        assert_eq!(
            cpu.stack.pop(),
            0x200 + 2,
            "The PC (which starts on 0x200) is popped out of the stack"
        );
    }

    #[test]
    fn instr_cond_eq() {
        let mut cpu = Cpu::new();

        cpu.load_and_exec(0x3200);
        assert_eq!(cpu.pc, 0x200 + 4, "Skips if condition is equal");
    }

    #[test]
    fn instr_cond_not_eq() {
        let mut cpu = Cpu::new();

        cpu.load_and_exec(0x3212);
        assert_eq!(cpu.pc, 0x200 + 2, "Doesn't skips if condition is not equal");
    }

    #[test]
    fn instr_cond_eq_vx_vy() {
        let mut cpu = Cpu::new();
        let rom = vec![
            // Assigns Vx to 11
            0x6B, 0x11, // Assigns Vx to 11
            0x6A, 0x10, // Conditional Eq for Vx to Vy
            0x5B, 0xA0,
        ];

        cpu.load(rom.into());
        cpu.cycle(KeypadState::new());
        cpu.cycle(KeypadState::new());

        assert_eq!(
            cpu.pc,
            0x200 + 4,
            "Doesn't skips a WORD because register value at Vx is not equal to KK."
        );
    }

    #[test]
    fn instr_const_assign_vx_to_kk() {
        let mut cpu = Cpu::new();
        let rom = vec![
            // Assigns Vx to 11
            0x6B, 0x0B,
        ];

        cpu.load(rom.into());
        cpu.cycle(KeypadState::new());

        assert_eq!(
            cpu.registers[0x0b], 0x0b,
            "Assigns KK to the register on address VX"
        );
    }

    #[test]
    fn instr_assign_vx_to_vy() {
        let mut cpu = Cpu::new();
        let rom = vec![
            // Assigns Vx to 8, this will be our next Vy register
            0x6A, 0x08, // Assigns Vx to value on Register Vy (0x6A)
            0x8B, 0xA0,
        ];

        cpu.load(rom.into());
        cpu.cycle(KeypadState::new());
        cpu.cycle(KeypadState::new());

        assert_eq!(
            cpu.registers[0x0A], 0x08,
            "Register on 0x0A has is set to 0x08"
        );
        assert_eq!(
            cpu.registers[0x0B], 0x08,
            "Register 0x0B has value of 0x0A (0x08)"
        );
    }

    #[test]
    fn instr_bit_op_or() {
        let mut cpu = Cpu::new();
        let rom = vec![
            // Assigns 0x0a to 4
            0x6A, 0x04, // Assigns 0x0b to 2
            0x6B, 0x02, // Perform OR on 0x0a | 0x0b
            0x8A, 0xB1,
        ];

        cpu.load(rom.into());
        cpu.cycle(KeypadState::new());
        cpu.cycle(KeypadState::new());
        cpu.cycle(KeypadState::new());

        assert_eq!(
            cpu.registers[0x0a], 0x06,
            "Register on 0x0A is set to 0x06 due to the result from 4 | 2"
        );
    }

    #[test]
    fn instr_bit_op_and() {
        let mut cpu = Cpu::new();
        let rom = vec![
            // Assigns 0x0a to 10
            0x6A, 0x0A, // Assigns 0x0b to 11
            0x6B, 0x0B, // Perform AND on 0x0a | 0x0b
            0x8A, 0xB2,
        ];

        cpu.load(rom.into());
        cpu.cycle(KeypadState::new());
        cpu.cycle(KeypadState::new());
        cpu.cycle(KeypadState::new());

        assert_eq!(
            cpu.registers[0x0a], 0x0a,
            "Register on 0x0A is set to 0x0A due to the result from 10 & 11"
        );
    }

    #[test]
    fn instr_bit_xor_and() {
        let mut cpu = Cpu::new();
        let rom = vec![
            // Assigns 0x0a to 4
            0x6A, 0x04, // Assigns 0x0b to 8
            0x6B, 0x08, // Perform AND on 0x0a | 0x0b
            0x8A, 0xB3,
        ];

        cpu.load(rom.into());
        cpu.cycle(KeypadState::new());
        cpu.cycle(KeypadState::new());
        cpu.cycle(KeypadState::new());

        assert_eq!(
            cpu.registers[0x0a], 0x0c,
            "Register on 0x0A is set to 0x0C due to the result from 4 ^ 8"
        );
    }

    #[test]
    fn instr_math_add() {
        let mut cpu = Cpu::new();
        let rom = vec![
            // Assigns 0x0a to 10
            0x6A, 0x0A, // Assigns 0x0b to 13
            0x6B, 0x0D, // Perform AND on 0x0a | 0x0b
            0x8A, 0xB4,
        ];

        cpu.load(rom.into());
        cpu.cycle(KeypadState::new());
        cpu.cycle(KeypadState::new());
        cpu.cycle(KeypadState::new());

        assert_eq!(
            cpu.registers[0x0a], 0x17,
            "Register on 0x0A is set to 0x17 due to the result from 10 + 13"
        );
        assert_eq!(
            cpu.registers[0xF], 0,
            "Register VF is set to 0 due to lack of overflow"
        );
    }

    #[test]
    fn instr_math_sub() {
        let mut cpu = Cpu::new();
        let rom = vec![
            // Assigns 0x0a to 13
            0x6E, 0x0A, // Assigns 0x0b to 10
            0x6D, 0x0D, // Perform on 0x0a + 0x0b
            0x8D, 0xE5,
        ];

        cpu.load(rom.into());
        cpu.cycle(KeypadState::new());
        cpu.cycle(KeypadState::new());
        cpu.cycle(KeypadState::new());

        assert_eq!(
            cpu.registers[0x0d], 0x03,
            "Register on 0x0A is set to 0x03 due to the result from 13 - 10"
        );
        assert_eq!(
            cpu.registers[0xF], 0,
            "Register VF is set to 0 due to lack of overflow"
        );
    }

    #[test]
    fn instr_math_add_with_overflow() {
        let mut cpu = Cpu::new();
        let rom = vec![
            // Assigns 0x0a to 13
            0x61, 0xFF, // Assigns 0x0b to 10
            0x62, 0xFF, // Perform on 0x0a + 0x0b
            0x81, 0x24,
        ];

        cpu.load(rom.into());
        cpu.cycle(KeypadState::new());
        cpu.cycle(KeypadState::new());
        cpu.cycle(KeypadState::new());

        assert_eq!(
            cpu.registers[0x1], 0xFE,
            "Register on 0x0A is set to 0x00 due to the overflow"
        );
        assert_eq!(
            cpu.registers[0xF], 1,
            "Register VF is set to 1 due to the overflow"
        );
    }

    #[test]
    fn instr_math_sub_with_overflow() {
        let mut cpu = Cpu::new();
        let rom = vec![
            // Assigns 0x01 to 1
            0x61, 0x01, // Assigns 0x0d to 13
            0x62, 0x0D, // Perform on 0x0a + 0x0b
            0x81, 0x25,
        ];

        cpu.load(rom.into());
        cpu.cycle(KeypadState::new());
        cpu.cycle(KeypadState::new());
        cpu.cycle(KeypadState::new());

        assert_eq!(
            cpu.registers[0x1], 0xF4,
            "Register on 0x01 is set to 0xF4 due to the overflow from 1 - 13"
        );
        assert_eq!(
            cpu.registers[0xF], 1,
            "Register VF is set to 1 due to the overflow"
        );
    }

    #[test]
    fn instr_bit_op_shr() {
        let mut cpu = Cpu::new();
        let rom = vec![
            // Assigns 0x0a to 10
            0x6A, 0x0A, // Perform SHR on 0x0a >> 1
            0x8A, 0xB6,
        ];

        cpu.load(rom.into());
        cpu.cycle(KeypadState::new());
        cpu.cycle(KeypadState::new());

        assert_eq!(
            cpu.registers[0x0a], 0x05,
            "Register on 0x0A is set to 0x05 due to the result from 10 >> 1"
        );
    }

    #[test]
    fn instr_math_sub_vy_vx() {
        let mut cpu = Cpu::new();
        let rom = vec![
            // Assigns 0x0a to 10
            0x6A, 0x0A, // Perform 0x0a - 10
            0x8A, 0xA7,
        ];

        cpu.load(rom.into());
        cpu.cycle(KeypadState::new());
        cpu.cycle(KeypadState::new());

        assert_eq!(
            cpu.registers[0x0a], 0x00,
            "Register on 0x0A is set to 0x00 due to the result from 10 - 10"
        );
    }

    #[test]
    fn instr_bit_op_shl() {
        let mut cpu = Cpu::new();
        let rom = vec![
            // Assigns 0x0a to 10
            0x6A, 0x0A, // Perform SHR on 0x0a << 1
            0x8A, 0xBE,
        ];

        cpu.load(rom.into());
        cpu.cycle(KeypadState::new());
        cpu.cycle(KeypadState::new());

        assert_eq!(
            cpu.registers[0x0a], 0x14,
            "Register on 0x0A is set to 0x14 due to the result from 10 << 1"
        );
    }

    #[test]
    fn instr_cond_vx_not_eq_vy() {
        let mut cpu = Cpu::new();
        let initial_pc = cpu.pc;
        let rom = vec![
            // Assigns 0x0a to 10
            0x6A, 0x0A, // Assigns 0x0b to 1
            0x6B, 0x01, // Perform != on 0x0a != 0x0b
            0x9A, 0xB0,
        ];

        cpu.load(rom.into());
        cpu.cycle(KeypadState::new());
        cpu.cycle(KeypadState::new());
        cpu.cycle(KeypadState::new());

        assert_eq!(
            cpu.pc,
            initial_pc + 8,
            "Program Counter is set to it's initial value plus 3 cycles and the skip"
        );
    }

    #[test]
    fn instr_mem() {
        let mut cpu = Cpu::new();
        let rom = vec![
            // Sets Index Register to Address 123
            0xA1, 0x23,
        ];

        cpu.load(rom.into());
        cpu.cycle(KeypadState::new());

        assert_eq!(cpu.i, 0x0123, "Index register is set to 0x0123");
    }

    #[test]
    fn instr_set_vx_eq_to_dt() {
        let mut cpu = Cpu::new();

        cpu.dt = 0x05A;

        let rom = vec![
            // Set the value of DT into Register 0x0a
            0xFA, 0x07,
        ];

        cpu.load(rom.into());
        cpu.cycle(KeypadState::new());

        assert_eq!(
            cpu.dt, cpu.registers[0x0A],
            "The Register A is set to the value of the DT"
        );
    }

    #[test]
    fn instr_set_dt_eq_to_vx() {
        let mut cpu = Cpu::new();

        cpu.dt = 0x05A;

        let rom = vec![
            // Set the value of Register 0x0a to DT
            0xFA, 0x15,
        ];

        cpu.load(rom.into());
        cpu.cycle(KeypadState::new());

        assert_eq!(
            cpu.dt, cpu.registers[0x0A],
            "The DT is set to the value of the Register on VX"
        );
    }

    #[test]
    fn instr_set_st_eq_to_vx() {
        let mut cpu = Cpu::new();

        cpu.registers[0x3] = 0x10;

        let rom = vec![0xF3, 0x18];

        cpu.load(rom.into());
        cpu.cycle(KeypadState::new());

        assert_eq!(cpu.st, 0x10);
    }
}
