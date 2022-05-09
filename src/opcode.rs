use std::fmt;

/// CPU Executable Instructions
///
/// Refer: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.1
#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    /// `0nnn` - SYS addr
    /// Jump to a machine code routine at nnn.
    ///
    /// This instruction is only used on the old computers on which Chip-8 was
    /// originally implemented. It is ignored by modern interpreters.
    SysAddr,
    /// `00E0` - CLS
    /// Clear the display.
    Cls,
    /// `00EE` - RET
    /// Return from a subroutine.
    ///
    /// The interpreter sets the program counter to the address at the top of
    /// the stack, then subtracts 1 from the stack pointer.
    Ret,
    /// `1nnn` - JP addr
    /// Jump to location `nnn`.
    ///
    /// The interpreter sets the program counter to `nnn`.
    Jump(u16),
    /// `2nnn` - CALL addr
    /// Call subroutine at nnn.
    ///
    /// The interpreter increments the stack pointer, then puts the current PC
    /// on the top of the stack. The `PC` is then set to `nnn`.
    CallSubroutine(u16),
    /// `3xkk` - SE Vx, byte
    /// Skip next instruction if Vx = kk.
    ///
    /// The interpreter compares register Vx to kk, and if they are equal,
    /// increments the program counter by 2.
    CondEq(usize, u8),
    /// `4xkk` - SNE Vx, byte
    /// Skip next instruction if Vx != kk.
    ///
    /// The interpreter compares register Vx to kk, and if they are not equal,
    /// increments the program counter by 2.
    CondNotEq(usize, u8),
    /// `5xy0` - SE Vx, Vy
    /// Skip next instruction if Vx = Vy.
    ///
    /// The interpreter compares register Vx to register Vy, and if they are
    /// equal, increments the program counter by 2.
    CondEqVxVy(usize, usize),
    /// `6xkk` - LD Vx, byte
    /// Set Vx = kk.
    ///
    /// The interpreter puts the value kk into register Vx.
    ConstAssignVxToKk(usize, u8),
    /// `7xkk` - ADD Vx, byte
    /// Set Vx = Vx + kk.
    ///
    /// Adds the value kk to the value of register Vx, then stores the result
    /// in Vx.
    ConstAddVxToKk(usize, u8),
    /// `8xy0` - LD Vx, Vy
    /// Set Vx = Vy.
    ///
    /// Stores the value of register Vy in register Vx.
    AssignVxToVy(usize, usize),
    /// `8xy1` - OR Vx, Vy
    /// Set Vx = Vx OR Vy.
    ///
    /// Performs a bitwise OR on the values of Vx and Vy, then stores the result
    /// in Vx. A bitwise OR compares the corrseponding bits from two values, and
    /// if either bit is 1, then the same bit in the result is also 1.
    /// Otherwise, it is 0.
    BitOpOr(usize, usize),
    /// `8xy2` - AND Vx, Vy
    /// Set Vx = Vx AND Vy.
    ///
    /// Performs a bitwise AND on the values of Vx and Vy, then stores the
    /// result in Vx. A bitwise AND compares the corrseponding bits from two
    /// values, and if both bits are 1, then the same bit in the result is also
    /// 1. Otherwise, it is 0.
    BitOpAnd(usize, usize),
    /// `8xy3` - XOR Vx, Vy
    /// Set Vx = Vx XOR Vy.
    ///
    /// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores
    /// the result in Vx. An exclusive OR compares the corrseponding bits from
    /// two values, and if the bits are not both the same, then the
    /// corresponding bit in the result is set to 1. Otherwise, it is 0.
    BitOpXor(usize, usize),
    /// `8xy4` - ADD Vx, Vy
    /// Set Vx = Vx + Vy, set VF = carry.
    ///
    /// The values of Vx and Vy are added together. If the result is greater
    /// than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest
    /// 8 bits of the result are kept, and stored in Vx.
    MathAdd(usize, usize),
    /// `8xy5` - SUB Vx, Vy
    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    ///
    /// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted
    /// from Vx, and the results stored in Vx.
    MathSub(usize, usize),
    /// `8xy6` - SHR Vx {, Vy}
    /// Set Vx = Vx SHR 1.
    ///
    /// If the least-significant bit of Vx is 1, then VF is set to 1,
    /// otherwise 0. Then Vx is divided by 2.
    BitOpShr(usize),
    /// 8xy7 - SUBN Vx, Vy
    /// Set Vx = Vy - Vx, set VF = NOT borrow.
    ///
    /// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted
    /// from Vy, and the results stored in Vx.
    MathSubVyVx(usize, usize),
    /// `8xyE` - SHL Vx {, Vy}
    /// Set Vx = Vx SHL 1.
    ///
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise
    /// to 0. Then Vx is multiplied by 2.
    BitOpShl(usize),
    /// `9xy0` - SNE Vx, Vy
    /// Skip next instruction if Vx != Vy.
    ///
    /// The values of Vx and Vy are compared, and if they are not equal, the
    /// program counter is increased by 2.
    CondVxNotEqVy(usize, usize),
    /// `Annn` - LD I, addr
    /// Set I = nnn.
    ///
    /// The value of register I is set to nnn.
    Mem(u16),
    /// `Bnnn` - JP V0, addr
    /// Jump to location nnn + V0.
    ///
    /// The program counter is set to nnn plus the value of V0.
    JumpPcV0(u16),
    /// `Cxkk` - RND Vx, byte
    /// Set Vx = random byte AND kk.
    ///
    /// The interpreter generates a random number from 0 to 255, which is then
    /// ANDed with the value kk. The results are stored in Vx. See instruction
    /// `8xy2` for more information on AND.
    Rand(usize, u8),
    /// `Dxyn` - DRW Vx, Vy, nibble
    /// Display n-byte sprite starting at memory location I at (Vx, Vy),
    /// set VF = collision.
    ///
    /// The interpreter reads n bytes from memory, starting at the address
    /// stored in I. These bytes are then displayed as sprites on screen at
    /// coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If
    /// this causes any pixels to be erased, VF is set to 1, otherwise it is
    /// set to 0. If the sprite is positioned so part of it is outside the
    /// coordinates of the display, it wraps around to the opposite side of the
    /// screen. See instruction 8xy3 for more information on XOR, and section
    /// 2.4, Display, for more information on the Chip-8 screen and sprites.
    Draw(usize, usize, u8),
    /// `Ex9E` - SKP Vx
    /// Skip next instruction if key with the value of Vx is pressed.
    ///
    /// Checks the keyboard, and if the key corresponding to the value of Vx is
    /// currently in the down position, PC is increased by 2.
    SkipIfKeyPressed(usize),
    /// `ExA1` - SKNP Vx
    /// Skip next instruction if key with the value of Vx is not pressed.
    ///
    /// Checks the keyboard, and if the key corresponding to the value of Vx is
    /// currently in the up position, PC is increased by 2.
    KeyOpVxNotPressed(usize),
    /// `Fx07` - LD Vx, DT
    /// Set Vx = delay timer value.
    ///
    /// The value of DT is placed into Vx.
    SetVxEqToDt(usize),
    /// `Fx0A` - LD Vx, K
    /// Wait for a key press, store the value of the key in Vx.
    ///
    /// All execution stops until a key is pressed, then the value of that key
    /// is stored in Vx.
    WaitKeyPressAndStoreOnVx(usize),
    /// `Fx15` - LD DT, Vx
    /// Set delay timer = Vx.
    ///
    /// DT is set equal to the value of Vx.
    SetDtEqToVx(usize),
    /// `Fx18` - LD ST, Vx
    /// Set sound timer = Vx.
    ///
    /// ST is set equal to the value of Vx.
    SetStEqToVx(usize),
    /// `Fx1E` - ADD I, Vx
    /// Set I = I + Vx.
    ///
    /// The values of I and Vx are added, and the results are stored in I.
    SetIEqToIPlusVx(usize),
    /// `Fx29` - LD F, Vx
    /// Set I = location of sprite for digit Vx.
    ///
    /// The value of I is set to the location for the hexadecimal sprite
    /// corresponding to the value of Vx. See section 2.4, Display, for more
    /// information on the Chip-8 hexadecimal font.
    SetIEqToVx(usize),
    /// `Fx33` - LD B, Vx
    /// Store BCD representation of Vx in memory locations I, I+1, and I+2.
    ///
    /// The interpreter takes the decimal value of Vx, and places the hundreds
    /// digit in memory at location in I, the tens digit at location I+1, and
    /// the ones digit at location I+2.
    StoreBinaryCodedDecimal(usize),
    /// `Fx55` - LD [I], Vx
    /// Store registers V0 through Vx in memory starting at location I.
    ///
    /// The interpreter copies the values of registers V0 through Vx into
    /// memory, starting at the address in I.
    SetRegsInI(usize),
    /// `Fx65` - LD Vx, [I]
    /// Read registers V0 through Vx from memory starting at location I.
    ///
    /// The interpreter reads values from memory starting at location I into
    /// registers V0 through Vx.
    GetRegsInI(usize),
    /// An Instruction sent when an unknown opcode is encountered
    Unknown,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let instr_code = match &self {
            Instruction::Cls => "CLS",
            Instruction::SysAddr => "SYS",
            Instruction::Ret => "RET",
            Instruction::Jump(_) => "JP",
            Instruction::CallSubroutine(_) => "CALL",
            Instruction::CondEq(_, _) => "SE",
            Instruction::CondNotEq(_, _) => "SNE",
            Instruction::CondEqVxVy(_, _) => "SNER",
            Instruction::ConstAssignVxToKk(_, _) => "LDVX",
            Instruction::ConstAddVxToKk(_, _) => "ADD",
            Instruction::AssignVxToVy(_, _) => "LDVXVY",
            Instruction::BitOpOr(_, _) => "ORVXVY",
            Instruction::BitOpAnd(_, _) => "ANDVXVY",
            Instruction::BitOpXor(_, _) => "XORVXVY",
            Instruction::MathAdd(_, _) => "MADD",
            Instruction::MathSub(_, _) => "MSUB",
            Instruction::MathSubVyVx(_, _) => "MSUBVXVY",
            Instruction::BitOpShr(_) => "SHRT",
            Instruction::BitOpShl(_) => "SHLT",
            Instruction::CondVxNotEqVy(_, _) => "SNEVXVY",
            Instruction::Mem(_) => "MEM",
            Instruction::JumpPcV0(_) => "JPV0",
            Instruction::Rand(_, _) => "RND",
            Instruction::Draw(_, _, _) => "DRW",
            Instruction::SkipIfKeyPressed(_) => "SKP",
            Instruction::KeyOpVxNotPressed(_) => "SKNP",
            Instruction::SetVxEqToDt(_) => "LDVXDT",
            Instruction::WaitKeyPressAndStoreOnVx(_) => "LDVXK",
            Instruction::SetDtEqToVx(_) => "LDDTVX",
            Instruction::SetStEqToVx(_) => "LDSTVX",
            Instruction::SetIEqToIPlusVx(_) => "ADDIVX",
            Instruction::SetIEqToVx(_) => "LDFVX",
            Instruction::StoreBinaryCodedDecimal(_) => "LDBVX",
            Instruction::SetRegsInI(_) => "LDIVX",
            Instruction::GetRegsInI(_) => "LDVXI",
            Instruction::Unknown => "UNKWN",
        };

        write!(f, "{}", instr_code)
    }
}

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
/// Refer: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.0
#[derive(Debug)]
pub struct Opcode(u16);

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#06x}", self.0)
    }
}

impl Opcode {
    #[inline(always)]
    pub fn c(&self) -> u8 {
        ((self.0 & 0xF000) >> 12) as u8
    }

    #[inline(always)]
    pub fn d(&self) -> u8 {
        (self.0 & 0x000F) as u8
    }

    #[inline(always)]
    pub fn kk(&self) -> u8 {
        (self.0 & 0x00FF) as u8
    }

    #[inline(always)]
    pub fn n(&self) -> u8 {
        (self.0 & 0x000F) as u8
    }

    #[inline(always)]
    pub fn nnn(&self) -> u16 {
        self.0 & 0x0FFF
    }

    #[inline(always)]
    pub fn vx(&self) -> usize {
        ((self.0 & 0x0F00) >> 8) as usize
    }

    #[inline(always)]
    pub fn vy(&self) -> usize {
        ((self.0 & 0x00F0) >> 4) as usize
    }

    /// Decodes a `Opcode` as hexadecimal as an `Instruction` which can be
    /// processed by the CPU.
    pub fn decode(&self) -> Instruction {
        let nibbles = (
            (self.0 & 0xF000) >> 12 as u8,
            (self.0 & 0x0F00) >> 8 as u8,
            (self.0 & 0x00F0) >> 4 as u8,
            (self.0 & 0x000F) as u8,
        );

        let nnn = (self.0 & 0x0FFF) as u16;
        let kk = (self.0 & 0x00FF) as u8;
        let vx = nibbles.1 as usize;
        let vy = nibbles.2 as usize;
        let n = nibbles.3 as u8;

        match nibbles {
            (0x00, 0x00, 0x0e, 0x00) => Instruction::Cls,
            (0x00, 0x00, 0x0e, 0x0e) => Instruction::Ret,
            (0x01, _, _, _) => Instruction::Jump(nnn),
            (0x02, _, _, _) => Instruction::CallSubroutine(nnn),
            (0x03, _, _, _) => Instruction::CondEq(vx, kk),
            (0x04, _, _, _) => Instruction::CondNotEq(vx, kk),
            (0x05, _, _, 0x00) => Instruction::CondEqVxVy(vx, vy),
            (0x06, _, _, _) => Instruction::ConstAssignVxToKk(vx, kk),
            (0x07, _, _, _) => Instruction::ConstAddVxToKk(vx, kk),
            (0x08, _, _, 0x00) => Instruction::AssignVxToVy(vx, vy),
            (0x08, _, _, 0x01) => Instruction::BitOpOr(vx, vy),
            (0x08, _, _, 0x02) => Instruction::BitOpAnd(vx, vy),
            (0x08, _, _, 0x03) => Instruction::BitOpXor(vx, vy),
            (0x08, _, _, 0x04) => Instruction::MathAdd(vx, vy),
            (0x08, _, _, 0x05) => Instruction::MathSub(vx, vy),
            (0x08, _, _, 0x06) => Instruction::BitOpShr(vx),
            (0x08, _, _, 0x07) => Instruction::MathSubVyVx(vx, vy),
            (0x08, _, _, 0x0E) => Instruction::BitOpShl(vx),
            (0x09, _, _, 0x00) => Instruction::CondVxNotEqVy(vx, vy),
            (0x0A, _, _, _) => Instruction::Mem(nnn),
            (0x0B, _, _, _) => Instruction::JumpPcV0(nnn),
            (0x0C, _, _, _) => Instruction::Rand(vx, kk),
            (0x0D, _, _, _) => Instruction::Draw(vx, vy, n),
            (0x0E, _, 0x09, 0x0E) => Instruction::SkipIfKeyPressed(vx),
            (0x0E, _, 0x0A, 0x01) => Instruction::KeyOpVxNotPressed(vx),
            (0x0F, _, 0x00, 0x07) => Instruction::SetVxEqToDt(vx),
            (0x0F, _, 0x00, 0x0A) => Instruction::WaitKeyPressAndStoreOnVx(vx),
            (0x0F, _, 0x01, 0x05) => Instruction::SetDtEqToVx(vx),
            (0x0F, _, 0x01, 0x08) => Instruction::SetStEqToVx(vx),
            (0x0F, _, 0x01, 0x0E) => Instruction::SetIEqToIPlusVx(vx),
            (0x0F, _, 0x02, 0x09) => Instruction::SetIEqToVx(vx),
            (0x0F, _, 0x03, 0x03) => Instruction::StoreBinaryCodedDecimal(vx),
            (0x0F, _, 0x05, 0x05) => Instruction::SetRegsInI(vx),
            (0x0F, _, 0x06, 0x05) => Instruction::GetRegsInI(vx),
            _ => Instruction::Unknown,
        }
    }
}

impl From<u16> for Opcode {
    fn from(hexa: u16) -> Self {
        Opcode(hexa)
    }
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

    #[test]
    fn retrieves_variable_vx() {
        let hexa = 0xE1AD;
        let opcode = Opcode::from(hexa);
        let vx = opcode.vx();

        assert_eq!(vx, 1);
    }

    #[test]
    fn retrieves_variable_vy() {
        let hexa = 0xE1AD;
        let opcode = Opcode::from(hexa);
        let vy = opcode.vy();

        assert_eq!(vy, 10);
    }

    #[test]
    fn retrieves_variable_c() {
        let hexa = 0xE1AD;
        let opcode = Opcode::from(hexa);
        let c = opcode.c();

        assert_eq!(c, 14);
    }

    #[test]
    fn retrieves_variable_d() {
        let hexa = 0xE1AD;
        let opcode = Opcode::from(hexa);
        let d = opcode.d();

        assert_eq!(d, 13);
    }

    #[test]
    fn retrieves_variable_kk() {
        let hexa = 0xE1AD;
        let opcode = Opcode::from(hexa);
        let kk = opcode.kk();

        assert_eq!(kk, 173);
    }

    #[test]
    fn retrieves_variable_nnn() {
        let hexa = 0x2123;
        let opcode = Opcode::from(hexa);
        let nnn = opcode.nnn();

        assert_eq!(nnn, 291);
    }
}
