/// CHIP-8 CPU implementation
///
/// Refer: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
#[derive(Debug)]
pub struct Chip8 {
    // The current operation
    pub program_counter: usize,
    // Registers to store operation bytes on
    pub registers: [u8; 16],
    // 4096 bytes of RAM
    pub memory: [u8; 0x1000],
}

impl Chip8 {
    /// Creates a new empty CPU instance
    pub fn new() -> Self {
        Chip8 {
            program_counter: 0,
            registers: [0; 16],
            memory: [0; 0x1000],
        }
    }

    /// Retrieves the CPU current operation code
    pub fn opcode_from_memory(&self) -> u16 {
        let counter = self.program_counter;
        let ho_byte = self.memory[counter] as u16;
        let lo_byte = self.memory[counter + 1] as u16;

        ho_byte << 8 | lo_byte
    }

    /// Runs CPU main loop.
    ///
    /// 1. Reads the opcode from `current_operation`
    /// 2. Decodes the instruction
    /// 3. Dispatches the decoded instruction if matches
    pub fn run(&mut self) {
        loop {
            let opcode = self.opcode_from_memory();

            self.program_counter += 2;

            let c = ((opcode & 0xf000) >> 12) as u8;
            let x = ((opcode & 0x0f00) >> 8) as u8;
            let y = ((opcode & 0x00f0) >> 4) as u8;
            let d = ((opcode & 0x000f) >> 0) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    // The 0x0000 opcode terminates the execution.
                    return;
                }
                (0x8, _, _, 0x4) => self.dispatch_addition(x, y),
                _ => todo!("opcode: {:04x}", opcode),
            }
        }
    }

    fn dispatch_addition(&mut self, x: u8, y: u8) {
        let r1 = self.registers[x as usize];
        let r2 = self.registers[y as usize];
        let (value, is_overflow) = r1.overflowing_add(r2);

        self.registers[x as usize] = value;

        if is_overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
}
