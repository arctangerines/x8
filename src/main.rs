extern crate sdl2;

/// LIST OF URGENT THINGS
/// TODO: Figure out how a function pointer table works in Rust
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::surface;
use sdl2::video::Window;

const FAKESET: [u8; 80] = [
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
//FIXME: Time to organise this per file
#[allow(dead_code, unused_imports)]
mod chip8
{
    pub mod CPU
    {
        // REVIEW: I have not decided if I should use imports
        // or I should inline them
        use sdl2::{render::Canvas, video::Window, rect::Point};

        /// TODO: Finish the struct thats representing the layout of the cpu
        /// NOTE: its kinda done?
        pub struct Chip8
        {
            /// 16 registers that are 8 bit wide
            pub registers: [u8; 16],
            /// 4096 bytes of memory
            pub memory: [u8; 4096],
            /// The display is 64 x 32
            pub display: [u8; 64 * 32],
            /// Initial implementation of canvasdata
            pub video: super::display::VData,
            /// The program counter holds the currently executing address (glorified index for an arrays)
            pub pc: u16,
            /// The index is a special register that stores addresses
            pub index: u16,
            /// A CPU stack of 16 values,
            /// moves downwards lolz.\
            /// hey you know fun fact:\
            /// since this is an emulator the stack can be as long as we want it to
            /// as long as its 16 values in the stack (array)
            pub stack: [u16; 16],
            /// A timer that will delay haha
            pub delay_timer: u8,
            /// A timer that will sound haha
            pub sound_timer: u8,
            /// 16 bit opcode
            pub opcode: u16,
        }

        /// # MemLocations
        /// These are the relevant memory locations
        pub enum MemLocations
        {
            /// Beginning of the interpreter (non existant)
            Interpreter = 0x000,
            /// End of the interpreter (RIP)
            InterpreterEnd = 0x01FF,
            /// Beginning of the fontset
            FontSet = 0x050,
            /// End of the font set
            FontSetEnd = 0x0A0,
            /// Start of the Rom, the actual memory we modify
            Rom = 0x200,
            /// End of the Rom
            RomEnd = 0xFFF,
        }
        // XXX: a way to pair the sprite with the sprite data, like the size of each char (bits)
        // this would allow for a sort of "dynamic" sprite system
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
        impl Default for Chip8
        {
            fn default() -> Self { Chip8::new() }
        }

        impl Chip8
        {
            /// Constructor :)
            pub fn new() -> Self
            {
                let mut c8 = Chip8 {
                    registers: [0x0; 16],
                    memory: [0x0; 4096],
                    display: [0x0; 64 * 32],
                    video: super::display::VData::new(1), /* REVIEW: Should this new take in the scale factor as a parameter? */
                    pc: MemLocations::Rom as u16,
                    index: 0x0,
                    stack: [0x0; 16],
                    delay_timer: 0x0,
                    sound_timer: 0x0,
                    opcode: 0x0,
                };
                for i in 0..FONTSET_SIZE
                {
                    c8.memory[(MemLocations::FontSet as usize) + i] =
                        FONTSET[i];
                }
                c8
            }
            pub fn open(
                &mut self,
                path: &str,
            )
            {
                let rom = std::fs::read(path).expect("Couldn't open file.");
                println!("Rom memory:");
                println!("{:x?}", rom);

                for i in 0..rom.len()
                {
                    self.memory[(MemLocations::Rom as usize) + i] = rom[i];
                }
                println!("Chip8 memory:");
                println!("{:x?}", self.memory);
            }

            // TODO: Implement the cycle function
            // - Cycle -> a cycle of the cpu lolz
            pub fn cycle(&mut self)
            {
                println!(
                    "Starting: {:#06x}, PC is at: {:#06x}",
                    self.memory[self.pc as usize], self.pc
                );
                // We need to get the first 8 bits then the last 8
                // 4096 bytes :)
                // instructions are 16 bits
                // so we get the first 8 bits, move them to the left
                // then we get the last 8 (by adding 1 to pc) and OR | them
                //  pc      pc+1
                // [a 2] | [2 a]
                // op = [a 2 2 a]
                self.opcode = ((self.memory[self.pc as usize] as u16) << 8)
                    | (self.memory[(self.pc + 1) as usize] as u16);
                println!("op: {:#06x}", self.opcode);

                // Add 2 because the 16 bit instructions are handled in sets of 8 bit
                // and memory is an array of 8 bit values
                self.pc += 2;
                println!(
                    "Ending: {:#06x}, PC is at: {:#06x}",
                    self.memory[self.pc as usize], self.pc
                );
            }

            // TODO: Implement these 6 instruction AND reading the instructions from rom
            // - 00E0 (clear screen)
            // - 1NNN (jump)
            // - 6XNN (set register VX)
            // - 7XNN (add to register VX)
            // - ANNN (set index register I)
            // - DXYN (display function / draww)
            // The reason for wanting to implement these first is that
            // we can use the IBM test rom :)
            // TODO: perhaps I should change nn -> kk since a lot of docs use kk
            pub fn op_00e0(&mut self)
            {
                println!("Hello 00e0");
            }
            /// # 1nnn: Jump to location nnn
            /// This instruction jumps (sets PC) to the specified location (nnn)
            pub fn op_1nnn(&mut self)
            {
                println!("Hello 1nnn");
                let addr = self.opcode & 0x0FFF;
                self.pc = addr;
            }
            /// # 6xnn: Vx = nn
            /// This instruction sets Vx = nn
            pub fn op_6xnn(&mut self)
            {
                println!("Hello 6xnn");
                let vx = ((self.opcode & 0x0F00) >> 8) as usize;
                let byte = (self.opcode & 0x00FF) as u8;
                self.memory[vx] = byte;
            }
            /// # 7xnn: Vx = Vx + nn
            /// Sets Vx = Vx + nn
            /// Adds the value in Vx to nn then sets Vx to result
            pub fn op_7xnn(&mut self)
            {
                println!("Hello 7xnnn");
                // Better cast here than in every operation
                // (Rust requires array indexes to be usize)
                let vx = ((self.opcode & 0x0F00) >> 8) as usize;
                // (And this particular array stores u8 values)
                let byte = (self.opcode & 0x00FF) as u8;
                self.memory[vx] = self.memory[vx] + byte;
            }
            /// # annn: LD I, addr
            /// Sets I = nnn
            /// Sets the Index register to the 12 bit address
            pub fn op_annn(&mut self)
            {
                println!("Hello annn");
                let addr = self.opcode & 0x0FFF;
                self.index = addr;
            }
            /// dxyn: DRW Vx, Vy, nibble
            /// display n-byte(?) sprite that starts at location I
            /// at the X (Vx), Y (Vy)
            /// if collision (pixel XOR) VF = 1;
            /// The sprites are supposed to wrap around
            pub fn op_dxyn(
                &mut self,
                surf: &mut Canvas<Window>,
            )
            {
                let vx = ((self.opcode & 0x0F00) >> 8) as usize;
                let vy = ((self.opcode & 0x00F0) >> 4) as usize;
                let height = (self.opcode & 0x000F) as u32;
                let vf = 0xF as usize;

                // Obtain the x coordinate and this modulo operation wraps it around if its above the width
                // REVIEW: Will this wrap for the subsequent pixels and not the starting point?
                // perhaps it needs to be done on the loop?!?!?
                let x_point =
                    (self.registers[vx] % super::display::C8_WIDTH) as u32;
                let y_point =
                    (self.registers[vy] % super::display::C8_HEIGHT) as u32;

                // The mask, as opposed to my initial implementation, starts from left to right
                // no need to subtract, only add!
                let mask = 0b1000_0000;

                for row in 0..height
                {
                    // separate number because it looks disgusting inside the brackets
                    let idx = (self.index + (row as u16)) as usize;
                    // obtain the next byte of the sprite to display so we can & it later
                    let sprite = self.memory[idx];

                    //XXX: Its 8, period. Thats the size of each sprite, cant get it dynamically. For now...
                    for column in 0..8
                    {
                        // This is the index of the pixel array/vector
                        // this uses the formula
                        // (width * (y + row) + (x + column))
                        // previous iteration of this did it like this
                        // (width * y) + x
                        // and we did x += 1 in the inner loop (column loop)
                        // and y += 1 in the outer loop (row loop)
                        // this does it in 1 line, more elegant and clean, same formula
                        // this formula basically tells us where the pixel would be
                        // if we were to place it in a 1 dimensional array
                        let px_idx = ((self.video.width * (y_point + row))
                            + (x_point + column))
                            as usize;

                        // extract the bit inside of the pixel array
                        // FIXME: RENAME THIS VAR
                        let live_bit = self.video.pixels[px_idx];
                        // Just checking if theres a value inside, nothing weird :)
                        if live_bit == 1
                        {
                            // set flag register to collision...
                            self.registers[vf] = 0x1;
                            // and xor the pixel!
                            self.video.pixels[px_idx] ^= 0x1;
                        }
                        // we move the mask by the amount of times we've looped
                        // if column = 2
                        // mask goes from
                        // 0b1000_0000;
                        // to
                        // 0b0010_0000;
                        // then we & it with the sprite byte
                        // we just wanna know if theres something,
                        // we dont care where it at
                        // REVIEW: should this be renamed? it is the drawing bit after all
                        let drwbit = (mask >> column) & sprite;
                        // if theres something we draw!
                        if drwbit != 0
                        {
                            surf.draw_point(Point::new(
                                (x_point + column) as i32,
                                (y_point + row) as i32,
                            ))
                            .expect("Unable to draw a point.");
                        }
                        // reset it
                        self.registers[vf] = 0x0;
                    }
                }
            }
        }
    }

    pub mod display
    {
        pub const C8_WIDTH: u8 = 64;
        pub const C8_HEIGHT: u8 = 32;

        /// ## VData
        /// Struct containing the various relevant canvas data
        /// Just a PoD (but not really because we got constructors and methods)
        /// to pass around to functions in an easy manner :D
        /// I didnt want to call it just Data
        pub struct VData
        {
            /// Chip8's default width (64)
            pub base_width: u8,
            /// Chip8's default height (32)
            pub base_height: u8,
            /// Width after scaling
            pub width: u32,
            /// Height after scaling
            pub height: u32,
            /// Scale factor that we will multiply with
            pub scale_factor: u32,
            /// A vector containing the amount of pixels in the screen after scaling
            pub pixelvec: Vec<u8>, // Rename once `pixels` does not exist
            /// An array containing the amount of pixels in the
            /// original chip8 screen.\
            /// This is to be used initially while developing before scaling.
            pub pixels: [u8; 64 * 32],
        }

        impl Default for VData
        {
            fn default() -> Self { VData::new(24) }
        }
        impl VData
        {
            /// Construct a new Canvas Data
            pub fn new(sf: u32) -> VData
            {
                VData {
                    base_width: C8_WIDTH,
                    base_height: C8_HEIGHT,
                    width: 64 * sf,
                    height: 32 * sf,
                    scale_factor: sf,
                    // FIXME: Too verbose.
                    pixelvec: vec![0; ((64 * sf) * (32 * sf)) as usize],
                    pixels: [0; 64 * 32],
                }
            }
        }
    }
}

fn main() -> Result<(), String>
{
    let mut c8 = chip8::CPU::Chip8::new();

    // sdl context
    let sld_ctx = sdl2::init()?;
    // video context
    let video_ctx = sld_ctx.video()?;

    let mut video_data = chip8::display::VData::new(1);

    let window = video_ctx
        .window("lmfao", video_data.width, video_data.height)
        .build()
        .map_err(|e| e.to_string())?;

    let mut surface_ctx =
        window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sld_ctx.event_pump()?;

    'running: loop
    {
        for event in event_pump.poll_iter()
        {
            match event
            {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseMotion { .. } =>
                {}
                e =>
                {
                    println!("{:?}", e);
                }
            }
        }

        surface_ctx.set_draw_color(Color::RGB(0, 0, 0));
        surface_ctx.clear();
        surface_ctx.set_draw_color(Color::RGB(255, 0, 255));
        for i in 0..video_data.width
        {
            surface_ctx
                .draw_point(Point::new(i as i32, i as i32))
                .expect("Unable to draw a point.");
        }
        surface_ctx.present();
        // im testing other stuff rn
        break;
    }

    c8.open("./rom/IBM Logo.ch8");
    c8.cycle();

    Ok(())
}
