extern crate sdl2;

/// LIST OF URGENT THINGS
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::surface;
use sdl2::video::Window;

//FIXME: Time to organise this per file
#[allow(dead_code, unused_imports)]
mod chip8
{
    // REVIEW: Should I remove this mod (probably not)
    pub mod CPU
    {
        // REVIEW: I have not decided if I should use imports
        // or I should inline them
        use sdl2::{render::Canvas, video::Window, rect::Point, pixels::Color};

        pub struct CPU
        {
            /// 16 registers that are 8 bit wide
            pub registers: [u8; 16],
            /// 4096 bytes of memory
            pub memory: [u8; 4096],
            /// The program counter holds the currently executing address (glorified index for an array)
            pub pc: usize,
            /// A CPU stack of 16 values,
            /// moves downwards lolz.\
            /// hey you know fun fact:\
            /// since this is an emulator the stack can be as long as we want it to
            /// as long as its 16 values in the stack (array)\
            /// We use usize values because PC is values
            /// and array/vec indexes in rust have to be usize
            /// We are using a vector now so we can push and pop :]
            pub stack: Vec<usize>,
            /// Stack pointer
            pub sp: usize,
            /// The index is a special register that stores addresses
            pub index: u16,
            /// 16 bit opcode
            pub opcode: u16,
            /// Display information
            pub display: super::display::VData,
            /// A timer that will delay haha
            pub delay_timer: u8,
            /// A timer that will sound haha
            pub sound_timer: u8,
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
        impl Default for CPU
        {
            fn default() -> Self { CPU::new() }
        }

        impl CPU
        {
            /// Constructor :)
            pub fn new() -> Self
            {
                let mut c8 = CPU {
                    registers: [0x0; 16],
                    memory: [0x0; 4096],
                    pc: MemLocations::Rom as usize,
                    stack: Vec::with_capacity(16),
                    sp: 0x0,
                    index: 0x0,
                    opcode: 0x0,
                    display: super::display::VData::new(1), /* REVIEW: Should this new take in the scale factor as a parameter? */
                    delay_timer: 0x0,
                    sound_timer: 0x0,
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

            /// # Cycle -> a cycle of the cpu lolz
            pub fn cycle(&mut self)
            {
                // We need to get the first 8 bits then the last 8
                // 4096 bytes :)
                // instructions are 16 bits
                // so we get the first 8 bits, move them to the left
                // then we get the last 8 (by adding 1 to pc) and OR | them
                //  pc      pc+1
                // [a 2] | [2 a]
                // op = [a 2 2 a]
                self.opcode = ((self.memory[self.pc] as u16) << 8)
                    | (self.memory[self.pc + 1] as u16);

                // Add 2 because the 16 bit instructions are handled in sets of 8 bit
                // and memory is an array of 8 bit values
                self.pc += 2;
            }

            pub fn op_0nnn(&mut self)
            {
            }
            /// # 00e0: Clear the display
            pub fn op_00e0(
                &mut self,
                surf: &mut Canvas<Window>,
            )
            {
                self.display.pixels.fill(0);
                surf.set_draw_color(Color::RGB(0, 0, 0));
                surf.clear();
                self.cycle();
            }
            /// # 00ee: RET
            /// Return from a subroutine.\
            /// Sets the PC to the top of the stack and subtracts 1 from the SP
            pub fn op_00ee(&mut self)
            {
                self.pc = *self
                    .stack
                    .last()
                    .expect("Couldn't obtain the last member of stack.");
                self.stack.pop();
                self.cycle();
            }
            /// # 1nnn: Jump to location nnn
            /// This instruction jumps (sets PC) to the specified location (nnn)
            pub fn op_1nnn(&mut self)
            {
                let addr = (self.opcode & 0x0FFF) as usize;
                self.pc = addr;
                self.cycle();
            }
            /// # 2nnn: CALL addr
            /// Call subroutine at nnn.\
            /// We increment the stack pointer
            /// save PC value at the top of the stack
            /// and PC is set to nnn.
            pub fn op_2nnn(&mut self)
            {
                let byte = self.opcode & 0x0FFF;
                self.stack.push(self.pc);
                self.pc = byte as usize;
                self.cycle();
            }
            /// # 3xkk: SE Vx, byte
            /// Skip next instruction if Vx == kk
            pub fn op_3xkk(&mut self)
            {
                let vx = ((self.opcode & 0x0F00) >> 8) as usize;
                let byte = (self.opcode & 0x00FF) as u8;
                if self.registers[vx] == byte
                {
                    self.pc += 2;
                }
                self.cycle();
            }
            /// # 4xkk: SNE Vx, byte
            /// Skip next instruction if Vx != kk
            pub fn op_4xkk(&mut self)
            {
                let vx = ((self.opcode & 0x0F00) >> 8) as usize;
                let byte = (self.opcode & 0x00FF) as u8;
                if self.registers[vx] != byte
                {
                    self.pc += 2;
                }
                self.cycle();
            }
            /// # 5xy0: SE Vx, Vy
            /// Skip next instruction if Vx == Vy
            pub fn op_5xy0(&mut self)
            {
                let vx = ((self.opcode & 0x0F00) >> 8) as usize;
                let vy = ((self.opcode & 0x00F0) >> 4) as usize;
                if self.registers[vx] == self.registers[vy]
                {
                    self.pc += 2;
                }
                self.cycle();
            }
            /// # 6xnn: Vx = nn
            /// This instruction sets Vx = nn
            pub fn op_6xnn(&mut self)
            {
                let vx = ((self.opcode & 0x0F00) >> 8) as usize;
                let byte = (self.opcode & 0x00FF) as u8;
                self.registers[vx] = byte;
                self.cycle();
            }
            /// # 7xnn: Vx = Vx + nn
            /// Sets Vx = Vx + nn
            /// Adds the value in Vx to nn then sets Vx to result
            /// This should be an addition with wrapping as the specification states
            /// That means that if we were add 1 to 255
            /// 0x01 + 0xFF
            /// we would preserve the lower 8 bits
            /// 0x0100 & 0x00FF
            /// 0x00
            /// (the bit behaviour is what the spec states)
            /// This is a wrapped addition!
            pub fn op_7xnn(&mut self)
            {
                // Better cast here than in every operation
                // (Rust requires array indexes to be usize)
                let vx = ((self.opcode & 0x0F00) >> 8) as usize;
                // (And this particular array stores u8 values)
                let byte = (self.opcode & 0x00FF) as u8;
                self.registers[vx] = self.registers[vx].wrapping_add(byte);

                self.cycle();
            }
            /// # 8xy0: LD Vx, Vy
            /// Sets Vx = Vy
            pub fn op_8xy0(&mut self)
            {
                let vx = ((self.opcode & 0x0F00) >> 8) as usize;
                let vy = ((self.opcode & 0x00F0) >> 4) as usize;

                self.registers[vx] = self.registers[vy];

                self.cycle();
            }
            /// # 8xy1: OR Vx, Vy
            /// Sets Vx = Vx | Vy
            pub fn op_8xy1(&mut self)
            {
                let vx = ((self.opcode & 0x0F00) >> 8) as usize;
                let vy = ((self.opcode & 0x00F0) >> 4) as usize;

                self.registers[vx] |= self.registers[vy];

                self.cycle();
            }
            /// # 8xy2: AND Vx, Vy
            /// Sets Vx = Vx & Vy
            pub fn op_8xy2(&mut self)
            {
                let vx = ((self.opcode & 0x0F00) >> 8) as usize;
                let vy = ((self.opcode & 0x00F0) >> 4) as usize;

                self.registers[vx] &= self.registers[vy];

                self.cycle();
            }
            /// # 8xy3: XOR Vx, Vy
            /// Sets Vx = Vx ^ Vy
            pub fn op_8xy3(&mut self)
            {
                let vx = ((self.opcode & 0x0F00) >> 8) as usize;
                let vy = ((self.opcode & 0x00F0) >> 4) as usize;

                self.registers[vx] ^= self.registers[vy];

                self.cycle();
            }
            /// # 8xy4: ADD Vx, Vy
            /// Sets Vx += Vy
            /// If the result is more than 8 bits, VF is set to 1
            pub fn op_8xy4(&mut self)
            {
                let vx = ((self.opcode & 0x0F00) >> 8) as usize;
                let vy = ((self.opcode & 0x00F0) >> 4) as usize;
                let vf = 0xF;

                // rust doesnt intend to implicitly typecast, not even when it makes sense
                // they evil but i get it
                let sum =
                    (self.registers[vx] as u16) + (self.registers[vy] as u16);

                if sum > 255
                {
                    self.registers[vf] = 0x1;
                }
                else
                {
                    self.registers[vf] = 0x0;
                }
                self.registers[vx] = (sum & 0x00FF) as u8;

                self.cycle();
            }
            /// # 8xy5: SUB Vx, Vy
            /// Sets Vx = Vx - Vy
            /// If borrow (Vy > Vx), VF = 0
            pub fn op_8xy5(&mut self)
            {
            }
            /// # 8xy6: SHR Vx {, Vy} [Should i also shift vy? lol]
            /// Shift Vx to the right 1 bit and store bit 0 (LS) in VF
            pub fn op_8xy6(&mut self)
            {
            }
            /// # 8xy7: SUBN Vx, Vy
            /// Sets Vx = Vy - Vx
            /// If borrow (Vx > Vy), VF = 0
            pub fn op_8xy7(&mut self)
            {
            }
            /// # 8xy6: SHL Vx {, Vy} [Should i also shift vy? lol]
            /// Shift Vx to the left 1 bit and store bit 15 (MS) in VF
            pub fn op_8xye(&mut self)
            {
            }
            /// # annn: LD I, addr
            /// Sets I = nnn
            /// Sets the Index register to the 12 bit address
            pub fn op_annn(&mut self)
            {
                let addr = self.opcode & 0x0FFF;
                self.index = addr;
                self.cycle();
            }
            /// dxyn: DRW Vx, Vy, nibble
            /// display n-byte(?) sprite that starts at location I
            /// at the X (Vx), Y (Vy)
            /// if collision (pixel XOR) VF = 1;
            /// The sprites are supposed to wrap around \
            /// The n-byte simply tells us how many sprites its gonna grab.
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
                // Potential solution: modulo to the points before doing the draw_point
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

                    // Its 8, period. Thats the size of each sprite, cant get it dynamically. For now...
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
                        let px_idx = ((self.display.width * (y_point + row))
                            + (x_point + column))
                            as usize;

                        // extract the current pixel bit inside of the pixel array
                        let pixel_bit = self.display.pixels[px_idx];
                        // Just checking if theres a value inside, nothing weird :)
                        if pixel_bit == 1
                        {
                            // set flag register to collision...
                            self.registers[vf] = 0x1;
                            // and xor the pixel!
                            self.display.pixels[px_idx] ^= 0x1;
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
                        let draw_bit = (mask >> column) & sprite;
                        // if theres something we saveit into its position!
                        if draw_bit != 0
                        {
                            self.display.pixels[px_idx] = 1;
                        }
                        // reset it
                        self.registers[vf] = 0x0;
                    }
                }
                for (i, v) in self.display.pixels.iter().enumerate()
                {
                    if *v != 0
                    {
                        // X pixel point, the x coordinate of the current pixel point
                        let x_px_point =
                            i % (super::display::C8_WIDTH as usize);
                        // Y pixel point, the x coordinate of the current pixel point
                        let y_px_point: f32 = (i as f32
                            / (super::display::C8_WIDTH as f32))
                            .floor();
                        surf.set_draw_color(Color::RGB(255, 0, 255));
                        surf.draw_point(Point::new(
                            x_px_point as i32,
                            y_px_point as i32,
                        ))
                        .expect("Unable to draw a point.");
                    }
                }
                surf.present();
                self.cycle();
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
                    pixelvec: vec![0; ((64 * sf) * (32 * sf)) as usize],
                    pixels: [0; 64 * 32],
                }
            }
        }
    }
}

fn main() -> Result<(), String>
{
    let mut c8 = chip8::CPU::CPU::new();

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

    c8.open("./rom/test3.ch8");
    surface_ctx.set_draw_color(Color::RGB(0, 0, 0));

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

        // TODO: Implement the decodification (?) of the instructions
        // so we can start using the correct instructions
        // Lets use a 4 element array for this, we should pattern match it :)
        let quartets = [
            ((c8.opcode & 0xF000) >> 12) as u8,
            ((c8.opcode & 0x0F00) >> 8) as u8,
            ((c8.opcode & 0x00F0) >> 4) as u8,
            (c8.opcode & 0x000F) as u8,
        ];
        match quartets
        {
            [0x0, 0x0, 0xe, 0x0] => c8.op_00e0(&mut surface_ctx),
            [0x0, 0x0, 0xe, 0xe] => c8.op_00ee(),
            [0x1, ..] => c8.op_1nnn(),
            [0x2, ..] => c8.op_2nnn(),
            [0x3, ..] => c8.op_3xkk(),
            [0x4, ..] => c8.op_4xkk(),
            [0x5, .., 0x0] => c8.op_5xy0(),
            [0x6, ..] => c8.op_6xnn(),
            [0x7, ..] => c8.op_7xnn(),
            [0x8, .., 0x0] => c8.op_8xy0(),
            [0x8, .., 0x1] => c8.op_8xy1(),
            [0x8, .., 0x2] => c8.op_8xy2(),
            [0x8, .., 0x3] => c8.op_8xy3(),
            [0x8, .., 0x4] => c8.op_8xy4(),
            [0xa, ..] => c8.op_annn(),
            [0xd, ..] => c8.op_dxyn(&mut surface_ctx),
            _ => c8.cycle(),
        }
    }

    Ok(())
}
