/// CHIP-8 CPU implementation
///
/// Refer: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
#[derive(Debug)]
struct Chip8 {
    operation: u16,
    registers: [u8; 2],
}

impl Chip8 {
    /// Creates a new empty CPU instance
    pub fn new() -> Self {
        Chip8 {
            operation: 0,
            registers: [0; 2],
        }
    }

    /// Retrieves the CPU current operation code
    pub fn opcode(&self) -> u16 {
        self.operation
    }

    /// Runs CPU main loop.
    ///
    /// 1. Reads the opcode from `current_operation`
    /// 2. Decodes the instruction
    /// 3. Dispatches the decoded instruction if matches
    pub fn run(&mut self) {
        let opcode = self.opcode();

        let c = ((opcode & 0xf000) >> 12) as u8;
        let x = ((opcode & 0x0f00) >> 8) as u8;
        let y = ((opcode & 0x00f0) >> 4) as u8;
        let d = ((opcode & 0x000f) >> 0) as u8;

        match (c, x, y, d) {
            (0x8, _, _, 0x4) => self.dispatch_addition(x, y),
            _ => todo!("opcode: {:04x}", opcode),
        }
    }

    fn dispatch_addition(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}

fn main() {
    let mut cpu = Chip8::new();
    println!("{:?}", cpu);

    cpu.operation = 0x8014;
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    println!("{:?}", cpu);

    cpu.run();

    println!("{:?}", cpu);
}
