/// LIST OF TODOS
/// TODO: Make a struct representing the layout of the cpu
/// TODO: Write the 5 basic instructions, initially just doing a "this is called"
/// TODO: Figure out how tf a function table works
/// TODO: Open file and read into memory
// #[allow(dead_code, unused_imports)]
mod chip8
{
    //     use std::{
    //         io::{Read, Seek, SeekFrom},
    //         fs::File,
    // };
    pub struct Chip8
    {
        /// 16 registers that are 8 bit wide
        pub registers: [u8; 16],
        /// 4096 bytes of memory
        pub memory: [u8; 4096],
        /// The display is 64 x 32
        pub display: [u32; 64 * 32],
        /// The program counter holds the currently executing address (glorified index for an arrays)
        pub pc: u16,
        /// The index is a special register that stores addresses
        pub index: u16,
        /// A stack of 16 plates (values),
        /// moves downwards,
        /// hey you know fun fact
        /// since this is an emulator the stack can be as long as we want it to
        /// as long as its min 16 values in the array
        pub stack: [u16; 16],
        pub delaytimer: u8,
        pub soundtimer: u8,
    }
    enum MemLocations
    {
        Rom = 0x200,
        RomEnd = 0xFFF,
    }

    const FONTSET_SIZE: usize = 80;
    const FONTSET: [u8; FONTSET_SIZE] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80, // F
    ];
    impl Chip8
    {
        fn open(
            &mut self,
            path: &str,
        )
        {
            let rom = std::fs::read(path).expect("Couldnt open rom.");
            print!("{:x?}", rom);
        }
    }

    impl Default for Chip8
    {
        fn default() -> Self
        {
            let mut c8 = Chip8 {
                registers: [0x0; 16],
                memory: [0x0; 4096],
                display: [0x0; 64 * 32],
                pc: MemLocations::Rom as u16,
                index: 0x0,
                stack: [0x0; 16],
                delaytimer: 0x0,
                soundtimer: 0x0,
            };
            c8
        }
    }
}
fn main()
{
    // let mut rom = File::open("./rom/IBM Logo.ch8");
}
