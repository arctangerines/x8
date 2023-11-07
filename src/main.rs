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
                    video: super::display::VData::new(24), /* REVIEW: Should this new take in the scale factor as a parameter? */
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
            ///TODO...
            pub fn op_dxyn(&mut self)
            {
                println!("Hello dxyn");
            }
        }
    }

    pub mod display
    {
        const C8_WIDTH: u8 = 64;
        const C8_HEIGHT: u8 = 32;

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

/// ### Draw function (dxyn)
/// In its final form it should be able to just deal with the opcode
/// extract the X and Y
/// then the last byte is used for n.\
/// n is the height, its so we can basically chop up the sprite.
/// this is an extremely BARE skeleton
// TODO: p_v is implemented in CanvasData, but pixel vec
// is something thats part of the cpu?
// so I need to decide if I should implement it on the CPU struct
// or on canvas data
// I am aware (now) that the CPU struct implements an array
// thats the same but thats for the 64 * 32 canvas
pub fn draww(
    surf_ctx: &Canvas<Window>,
    x: u16, // x position
    y: u16, // y position
    // TODO: rename
    sprite_chars: &[u8], /* slice! (Font slice for the characters) (sprite chars) */
    pixel_vec: &mut [u8], /* everything is a slice if u think abt it bro (im passing a vector)
                          p_v means pixel vector */
)
{
    // this is the masking bit, pretty straight forward
    let mut mask = 0b1;
    let sprc_len = sprite_chars.len();

    // this is the length of the sprite bytes
    // its 8 for now but this needs to actually be
    // multiplied by the scale factor
    // because we start at the end if that makes sense
    // TODO: Make sure the math for the scale factor is sound.
    let byte_len = 8;

    // drwbit this bit tells us if we draw a pixel or not
    let mut drawing_bit = 0b0;

    // point coordinates that will be changed(offset?[RIP Takeoff]) around
    // while we draw
    // we add byte_len to xp because this is the max width of the height
    // and we then start drawing from right to left, decreasing xp
    // we do this in the innermost loop
    // more info there
    let mut x_point = x + byte_len;
    let mut y_point = y;

    // NOTE: these values start off at the LAST pixel from right to left
    println!("X: {}, Y: {}", x, y);
    'ycoord: for i in 0..sprite_chars.len()
    {
        println!();
        println!(
            "n[i]:{:#010b}, hex: {:#x}",
            sprite_chars[i], sprite_chars[i]
        );
        'xcoord: for j in 0..byte_len
        {
            println!("xp:{}", x_point);
            println!("loopn: {}", j);

            drawing_bit = sprite_chars[i] & mask;

            println!("x: {}\nx(b): {:#010b}", drawing_bit, drawing_bit);
            println!("mask: {:#010b}", mask);

            // we move the mask, parsing the byte, bit by bit...
            mask <<= 1;
            // drwbit without the mask
            println!("k(w): {:#010b}", drawing_bit);
            // drwbit is the variable that indicates we should print
            // itll always either be 1 or 0
            // drwbit needs to be bitshifted as many times
            // as the loop has ran
            // FIXME: This can actually be put into the line where drwbit is declared
            drawing_bit >>= j;
            // drwbit but when its bitshifted
            println!("bits: {:#010b}", drawing_bit);
            x_point -= 1;
            print!("-------------------------------");

            // So the idea behind this implementation is that
            // (which I think is pretty common)
            // say we got a 5 x 4 display
            // thats 20 pixels, if we put it in a straight line
            // but we chop it up in sections of 4\
            // xxxxx xxoxx xxxxx xxxxx \
            // like so: \
            // xxxxx \
            // xxoxx \
            // xxxxx \
            // xxxxx \
            // say we are looking for the o in the array of pixels
            // we know the position is 2, 1 (remember it starts at 0,0)
            // so what we do is
            // we need to figure out in what row it is
            // so we multiply the width (in this case 5) by the y
            // since we wanna know where we need to start
            // in this case 5 * 1 gives us 5
            // this is the starting point for the x in y coordinate
            // we need to offset it (add to it) until we get to the position
            // oh whats that? we know the offset for x (its coordinate)
            // thats 2
            // 5 + 2 = 7
            // 7 is our position if we were to put all the pixels in 1 line
            // knowing this, we can XOR it QUICK :)
            // However, this is just the starting point, we need to take into consideration
            // the formula idx = (width * y) + x
            // doesnt need alteration because we do the addition of the y_point
            // and the subtraction (or addition depending on implementation look at op_dxyn) of x_point
            // in their respective loops

            // index: look at the long ass comment above
            // TODO: i need to take into consideration the scale factor
            // so i cant implement this just yet
            // px_idx = ()
        }
        // this is basically resetting the x
        // because we are moving down (y) and we need
        // to start counting (drawing) from rightmost pixel to left again
        // yp gets added 1
        //
        // 11110000 <- before adding 1 yp (end of first cycle)
        // 10000000 <- after adding to yp (beginning of second cycle)
        // 11110000
        // 10000000
        // 10000000
        // This is how the Chip8 draws sprites
        x_point = x_point + byte_len;
        y_point += 1;
        // we need to reset everything
        mask = 0x1;
        println!();
        println!();
    }
}

fn main() -> Result<(), String>
{
    let mut c8 = chip8::CPU::Chip8::new();

    // sdl context
    let sld_ctx = sdl2::init()?;
    // video context
    let video_ctx = sld_ctx.video()?;

    let mut video_data = chip8::display::VData::new(24);

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

    draww(
        &surface_ctx,
        103,
        56,
        &FAKESET[0..6],
        video_data.pixelvec.as_mut_slice(),
    );

    c8.open("./rom/IBM Logo.ch8");
    c8.cycle();

    Ok(())
}
