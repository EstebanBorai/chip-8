use anyhow::Error;
use std::fs;

use crate::config::Config;
use crate::memory::{Memory, USER_SPACE_STR};
use crate::opcode::{Instruction, Opcode};
use crate::stack::Stack;

pub type Rom = Vec<u8>;

pub struct Cpu {
    /// System available memory.
    pub(crate) ram: Memory,
    /// Program Counter
    pub(crate) pc: u16,
    /// Index reigster
    pub(crate) i: u16,
    /// Stack of 16 8-bit spaces
    pub(crate) stack: Stack,
    /// General Purpose Variable Registers
    ///
    /// 16 8-bit variable registers numbered from 0 through F.
    pub(crate) registers: [u8; 0x0016],
    /// Sound Timer (ST)
    pub(crate) st: u8,
    /// Delay Timer (DT)
    pub(crate) dt: u8,
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<Config> for Cpu {
    type Error = Error;

    fn try_from(config: Config) -> Result<Self, Self::Error> {
        let mut cpu = Cpu::new();
        let bytes = fs::read(config.rom).map_err(|err| Error::msg(err.to_string()))?;

        cpu.load(&bytes);

        Ok(cpu)
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
            registers: [0x0000; 0x0016],
            i: 0x0000,
            stack: Stack::default(),
            st: 0,
            dt: 0,
        }
    }

    /// Loads ROM bytes into memory
    pub fn load(&mut self, rom: &[u8]) {
        self.ram.load(rom);
    }

    /// Runs a CPU Cycle.
    ///
    /// First fetches the next instruction pointed out by the PC, then decodes
    /// the instruction and finally executes the instruction.
    pub fn cycle(&mut self) {
        let opcode = self.fetch_opcode();
        let instr = opcode.decode();

        self.execute(instr);
    }

    pub fn execute(&mut self, instr: Instruction) {
        match instr {
            Instruction::Cls => {
                todo!("Clear display turning all pixels to 0")
            }
            Instruction::Ret => self.pc = self.stack.pop().expect("Out of bounds!"),
            Instruction::SysAddr => println!("WARN: COSMAC VIP Only Instruction. Skipping."),
            Instruction::Jump(address) => self.pc = address,
            Instruction::CallSubroutine(address) => {
                self.stack.push(self.pc);
                self.pc = address;
            }
            Instruction::CondEq(vx, kk) => {
                println!(
                    "CondEq: VX: {:#04x} KK: {:#04x} REGVX: {:#04x}",
                    vx, kk, self.registers[vx]
                );
                if self.registers[vx] == kk {
                    self.pc += 2;
                }
            }
            Instruction::CondNotEq(vx, kk) => {
                println!(
                    "CondNotEq: VX: {:#04x} KK: {:#04x} REGVX: {:#04x}",
                    vx, kk, self.registers[vx]
                );
                if self.registers[vx] != kk {
                    self.pc += 2;
                }
            }
            Instruction::CondEqVxVy(vx, vy) => {
                println!(
                    "CondEqVxVy: VX: {:#04x} VY: {:#04x} REGVX: {:#04x} REGVY: {:#04x}",
                    vx, vy, self.registers[vx], self.registers[vy]
                );
                if self.registers[vx] == self.registers[vy] {
                    self.pc += 2;
                }
            }
            Instruction::ConstAssignVxToKk(vx, kk) => {
                println!("ConstAssignVxToKk: VX: {:#04x} KK: {:#04x}", vx, kk);
                self.registers[vx] = kk
            }
            Instruction::ConstAddVxToKk(vx, kk) => {
                self.registers[vx] = kk.wrapping_add(self.registers[vx])
            }
            Instruction::AssignVxToVy(vx, vy) => {
                println!("AssignVxToVy: VX: {:#04x} VY: {:#04x}", vx, vy);
                self.registers[vx] = self.registers[vy]
            }
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
                self.registers[vx] = self.registers[vx] + self.registers[vy]
            }
            Instruction::MathSub(vx, vy) => {
                self.registers[vx] = self.registers[vx] - self.registers[vy]
            }
            Instruction::BitOpShr(vx) => self.registers[vx] = self.registers[vx] >> 1,
            Instruction::MathSubVyVx(vx, vy) => {
                self.registers[vx] = self.registers[vx] - self.registers[vy]
            }
            Instruction::BitOpShl(vx) => self.registers[vx] = self.registers[vx] << 1,
            Instruction::CondVxNotEqVy(vx, vy) => {
                if self.registers[vx] != self.registers[vy] {
                    self.pc += 2;
                }
            }
            Instruction::Mem(nnn) => self.i = nnn,
            _ => {}
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
    use crate::memory::USER_SPACE_STR;

    use super::Cpu;

    #[test]
    fn load_rom_into_memory() {
        let mut cpu = Cpu::new();
        let rom = vec![0x001, 0x002, 0x003, 0x004];

        cpu.load(&rom);

        assert_eq!(cpu.ram[USER_SPACE_STR], 0x001);
        assert_eq!(cpu.ram[USER_SPACE_STR + 1], 0x002);
        assert_eq!(cpu.ram[USER_SPACE_STR + 2], 0x003);
        assert_eq!(cpu.ram[USER_SPACE_STR + 3], 0x004);
    }

    #[test]
    #[should_panic(expected = "Clear display turning all pixels to 0")]
    fn instr_cls() {
        let mut cpu = Cpu::new();
        let rom = vec![0x00, 0xE0];

        cpu.load(&rom);
        cpu.cycle();
    }

    #[test]
    fn instr_ret() {
        let mut cpu = Cpu::new();
        let rom = vec![
            // Sets a subroutine into the stack
            0x21, 0x23, // Returns from subroutine
            0x00, 0xEE,
        ];

        cpu.load(&rom);
        cpu.cycle();
        cpu.cycle();
    }

    #[test]
    fn instr_jump() {
        let mut cpu = Cpu::new();
        let rom = vec![0x12, 0xCD];

        cpu.load(&rom);
        cpu.cycle();

        assert_eq!(cpu.pc, 0x2CD, "Jump to address on NNN");
    }

    #[test]
    fn instr_call_subroutine() {
        let mut cpu = Cpu::new();
        let rom = vec![0x21, 0x23];

        cpu.load(&rom);
        cpu.cycle();

        assert_eq!(
            cpu.stack.pop().unwrap(),
            0x200 + 2,
            "The PC (which starts on 0x200) is popped out of the stack"
        );
        assert_eq!(cpu.pc, 0x123, "The value of PC is the one set by NNN");
    }

    #[test]
    fn instr_condeq() {
        let mut cpu = Cpu::new();
        let rom = vec![
            0x6B, // Assigns Vx to 2
            0x02, 0x3B, // Cond Eq for REGVX to KK
            0x02,
        ];

        cpu.load(&rom);
        cpu.cycle();
        cpu.cycle();

        assert_eq!(
            cpu.pc,
            0x200 + 6,
            "Skips a WORD given that register value at VX is equal to KK."
        );
    }

    #[test]
    fn instr_condnoteq() {
        let mut cpu = Cpu::new();
        let rom = vec![0x32, 0x04];

        cpu.load(&rom);
        cpu.cycle();

        assert_eq!(
            cpu.pc,
            0x200 + 2,
            "Doesn't skips a WORD because register value at VX is not equal to KK."
        );
    }

    #[test]
    fn instr_condeqvxvy() {
        let mut cpu = Cpu::new();
        let rom = vec![
            // Assigns Vx to 11
            0x6B, 0x11, // Conditional Eq for Vx to Vy
            0x51, 0x11,
        ];

        cpu.load(&rom);
        cpu.cycle();
        cpu.cycle();

        assert_eq!(
            cpu.pc,
            0x200 + 6,
            "Doesn't skips a WORD because register value at Vx is not equal to KK."
        );
    }

    #[test]
    fn instr_constassignvxtokk() {
        let mut cpu = Cpu::new();
        let rom = vec![
            // Assigns Vx to 11
            0x6B, 0x0B,
        ];

        cpu.load(&rom);
        cpu.cycle();

        assert_eq!(
            cpu.registers[0x0b], 0x0b,
            "Assigns KK to the register on address VX"
        );
    }

    #[test]
    fn instr_assignvxtovy() {
        let mut cpu = Cpu::new();
        let rom = vec![
            // Assigns Vx to 8, this will be our next Vy register
            0x6A, 0x08, // Assigns Vx to value on Register Vy (0x6A)
            0x8B, 0xA0,
        ];

        cpu.load(&rom);
        cpu.cycle();
        cpu.cycle();

        assert_eq!(
            cpu.registers[0x0A], 0x08,
            "Register on 0x0A has is set to 0x08"
        );
        assert_eq!(
            cpu.registers[0x0B], 0x08,
            "Register 0x0B has value of 0x0A (0x08)"
        );
    }
}
