/// Chip8 opcodes are 16-bit hexadecimal values which represent CPU
/// instructions. These are decoded and interpreted accordingly based on the
/// structure of the hexadecimal value.
///
/// # Anatomy of a Chip8 Opcode
///
/// The 16-bit value can be unpacked into 2 bytes, the High Byte and the
/// Low Byte. These 2 bytes can also be represented as 4 nibbles (4-bit values),
/// each byte would also contain a High Nibble and a Low Nibble.
///
/// ```ignore
///      HB  LB
///     _^_ _^_
/// 0 x 1 2 C D
///     | | | |_ Low nibble
///     | | |___ High nibble
///     | |_____ Low nibble
///     |_______ High nibble
///
/// HB: High-Byte
/// LB: Low-Byte
/// ```
///
/// # Opcode Variables
///
/// An opcode can hold variables, based on the opcode purpose a byte or nibble
/// could represent a value for the purpose of the instruction the opcode is
/// intended to execute. For instance, the `0x1NNN` opcode holds the variable
/// `NNN` which represents a memory address.
///
/// ```ignore
/// |  Variable  |                Position             |     Description     |
/// |------------|-------------------------------------|---------------------|
/// | n          | Low Byte, Low Nibble                | Number of bytes     |
/// | vx         | High Byte, Low Nibble               | CPU Register        |
/// | vy         | Low Byte, High Nibble               | CPU Register        |
/// | c          | High Byte, High Nibble              | Opcode Group        |
/// | d          | Low Byte, Low Nibble                | Opcode Subgroup     |
/// | kk         | Low Byte                            | Integer             |
/// | nnn        | High Byte, Low Nibble and Low Byte  | Memory Address      |
/// ```
///
/// Refer: https://en.wikipedia.org/wiki/CHIP-8#Opcode_table
pub struct Opcode(u16);

impl Opcode {
    pub fn n(&self) -> u8 {
        (self.0 & 0x000F) as u8
    }

    pub fn vx(&self) -> u8 {
        ((self.0 & 0x0F00) >> 8) as u8
    }

    pub fn vy(&self) -> u8 {
        ((self.0 & 0x00F0) >> 4) as u8
    }

    pub fn c(&self) -> u8 {
        ((self.0 & 0xF000) >> 12) as u8
    }

    pub fn d(&self) -> u8 {
        (self.0 & 0x000F) as u8
    }

    pub fn kk(&self) -> u8 {
        (self.0 & 0x00FF) as u8
    }

    pub fn nnn(&self) -> u16 {
        self.0 & 0x0FFF
    }

    pub fn decode(&self) -> Instruction {
        match self.0 {
            0x0000 => Instruction::NoOp,
            // 00EE Flow Jumps to address NNN.
            0x1000..=0x1FFF => Instruction::Jump(self.nnn()),
            // 2NNN Flow *(0xNNN)() Calls subroutine at NNN.
            0x2000..=0x2FFF => Instruction::CallSubroutine(self.nnn()),
            // 3XNN Cond if (Vx == NN) Skips the next instruction if VX
            // equals NN. (Usually the next instruction is a jump to skip a
            // code block);
            0x3000..=0x3FFF => {
                let vx = self.vx();
                let kk = self.kk();

                Instruction::CondEq(vx, kk)
            }
            _ => panic!("OpCode: {:#04x} not supported", self.0),
        }
    }
}

impl From<u16> for Opcode {
    fn from(hexa: u16) -> Self {
        Opcode(hexa)
    }
}

pub enum Instruction {
    NoOp,
    /// (`0x1NNN`) Sets the PC address to `NNN`
    Jump(u16),
    /// (`0x2NNN`) Calls subroutine at NNN.
    CallSubroutine(u16),
    /// (`0x3NNN`) Calls subroutine at NNN. `if (Vx == NN)`
    CondEq(u8, u8),
}

#[cfg(test)]
mod tests {
    use super::Opcode;

    #[test]
    fn retrieves_variable_n() {
        let hexa = 0x1234;
        let opcode = Opcode::from(hexa);
        let n = opcode.n();

        assert_eq!(n, 4);
    }
}
