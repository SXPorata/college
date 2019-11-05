fn main() {
    println!("Hello, Chip-8!");

    let cpu = Cpu::new();
    //above line is cleaner than below
    /* see impl block (defines behavior associated with types)
    let cpu = Cpu {
        opcode: 0,
        v: [0; 16],
        i: 0x200,
        sound_timer: 0,
        delay_timer: 0,
        pc: 0x200,
        sp: 0,
        memory: [0; 4096]
    };
    */
    cpu.load_program(vec![0x13, 0xC5]);
}

struct Cpu {
    opcode          :u16,  // opcode
    v:              [u8; 16], // registers
    i:              u16, // index register
    gfx:            [64 * 32]; // graphics, fix this!!
    sound_timer:    u8,
    delay_timer:    u8,
    pc:             u16, // program counter
    sp:             u8, // stack pointer
    stack:          [u8, 16],
    memory:         [u8; 4096]
}

impl Cpu {
    // keeping functions together because its more obvious
    // of what I want the CPU to do
    fn load_program(&mut self, program: Vec<u8>) {
        let data = vec![0; 0x200];

        //places data after the first 200 bytes of empty data
        for byte in program {
            data.push(byte);
        }
        // TODO: above can be done using append look something like this
        /*
            append(data, byte);

            fn append<T>(data: &Vec<T>, byte: ) {

            }

        */
        //places memory object in program memory
        for (index, &byte) in data.iter().enumerate() {
            self.memory[index] = byte;
        }
    }
    fn new() -> Cpu {
        Cpu {
            opcode: 0, // # representation of instruction
            v: [0; 16],
            i: 0x200,
            sound_timer: 0,
            delay_timer: 0,
            pc: 0x200,
            sp: 0,
            memory: [0; 4096]
        }
    }
}