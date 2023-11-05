extern crate sdl2;

/// LIST OF URGENT THINGS
/// FIXME: Naming conventions used for variables throughout the file
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
#[allow(dead_code, unused_imports)]
mod chip8
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
        pub display: [u32; 64 * 32],
        /// The program counter holds the currently executing address (glorified index for an arrays)
        pub pc: u16,
        /// The index is a special register that stores addresses
        pub index: u16,
        /// A CPU stack of 16 values,
        /// moves downwards lolz.\
        /// hey you know fun fact\
        /// since this is an emulator the stack can be as long as we want it to
        /// as long as its min 16 values in the array
        pub stack: [u16; 16],
        pub delaytimer: u8,
        pub soundtimer: u8,
        /// 16 bit opcode
        pub opcode: u16,
    }

    enum MemLocations
    {
        FontSet = 0x050,
        Rom = 0x200,
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
                pc: MemLocations::Rom as u16,
                index: 0x0,
                stack: [0x0; 16],
                delaytimer: 0x0,
                soundtimer: 0x0,
            };
            let fsstart = MemLocations::FontSet as usize;
            for i in 0..FONTSET_SIZE
            {
                c8.memory[fsstart + i] = FONTSET[i];
            }
            c8
        }
        pub fn open(
            &mut self,
            path: &str,
        )
        {
            let rom = std::fs::read(path).expect("Couldn't open file.");
            let romstart = MemLocations::Rom as usize;
            println!("Rom memory:");
            println!("{:x?}", rom);

            for i in 0..rom.len()
            {
                self.memory[romstart + i] = rom[i];
            }
            println!("Chip8 memory:");
            println!("{:x?}", self.memory);
        }

        // TODO: Implement the cycle function
        // - Cycle -> a cycle of the cpu lolz
        pub fn cycle(&mut self)
        {
            println!("{}", self.memory[self.pc as usize]);
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
        pub fn op_00e0(&mut self)
        {
            println!("Hello 00e0");
        }
        pub fn op_1nnn(&mut self)
        {
            println!("Hello 1nnn");
        }
        pub fn op_6xnn(&mut self)
        {
            println!("Hello 6xnn");
        }
        pub fn op_7xnn(&mut self)
        {
            println!("Hello 7xnn");
        }
        pub fn op_annn(&mut self)
        {
            println!("Hello annn");
        }
        pub fn op_dxyn(&mut self)
        {
            println!("Hello dxyn");
        }
    }
}

/// ## CanvasData
/// Struct containing the various relevant canvas data
/// Just a PoD (but not really because we got constructors and methods)
/// to pass around to functions in an easy manner :D
struct CanvasData
{
    width: u32,
    height: u32,
    scalefactor: u8,
    pixelvec: Vec<u8>,
}
impl CanvasData
{
    /// Construct a new Canvas Data
    pub fn new(
        sf: u8,
    ) -> CanvasData
    {
        CanvasData {
            width: (64 * sf) as u32,
            height: (32 * sf) as u32,
            scalefactor: sf,
            pixelvec: vec![0; (w * h) as usize],
        }
    }
}
impl Default for CanvasData
{
    fn default() -> Self { CanvasData::new(0, 0, 0) }
}
/// ### Draw function (dxyn)
/// In its final form it should be able to just deal with the opcode
/// extract the X and Y
/// then the last byte is used for n.\
/// n is the height, its so we can basically chop up the sprite.
/// this is an extremely BARE skeleton
pub fn draww(
    surfctx: &Canvas<Window>,
    x: u16, // x position
    y: u16, // y position
    // TODO: rename
    sprc: &[u8], // slice! (Font slice for the characters) (sprite chars)
    p_v: &mut [u8], /* everything is a slice if u think abt it bro (im passing a vector)
                    p_v means pixel vector */
)
{
    // TODO: perhaps after the opcode extracts the coordinates
    // we just operate on these points as opposed to moving the values around
    // big think
    let spoint = Point::new(x as i32, y as i32);

    // this is the masking bit, pretty straight forward
    let mut mask = 0b1;
    let sprc_len = sprc.len();

    // this is the length of the sprite bytes
    // its 8 for now but this needs to actually be
    // multiplied by the scale factor
    // because we start at the end if that makes sense
    // TODO: Make sure the math for the scale factor is sound.
    let byte_len = 8;

    // drwbit this bit tells us if we draw a pixel or not
    let mut drwbit = 0b0;

    // point coordinates that will be changed(offset?[RIP Takeoff]) around
    // while we draw
    // we add byte_len to xp because this is the max width of the height
    // and we then start drawing from right to left, decreasing xp
    // we do this in the innermost loop
    // more info there
    let mut xp = x + byte_len;
    let mut yp = y;

    // NOTE: these values start off at the LAST pixel from right to left
    println!("X: {}, Y: {}", x, y);
    'ycoord: for i in 0..sprc.len()
    {
        println!();
        println!("n[i]:{:#010b}, hex: {:#x}", sprc[i], sprc[i]);
        'xcoord: for j in 0..byte_len
        {
            println!("xp:{}", xp);
            println!("loopn: {}", j);

            drwbit = sprc[i] & mask;

            println!("x: {}\nx(b): {:#010b}", drwbit, drwbit);
            println!("mask: {:#010b}", mask);

            // we move the mask, parsing the byte, bit by bit...
            mask <<= 1;
            // drwbit without the mask
            println!("k(w): {:#010b}", drwbit);
            // drwbit is the variable that indicates we should print
            // itll always either be 1 or 0
            // drwbit needs to be bitshifted as many times
            // as the loop has ran
            // FIXME: This can actually be put into the line where drwbit is declared
            drwbit >>= j;
            // drwbit but when its bitshifted
            println!("bits: {:#010b}", drwbit);
            xp -= 1;
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
        xp = xp + byte_len;
        yp += 1;
        mask = 1;
        println!();
        println!();
    }
}

// let mut rom = File::open("./rom/IBM Logo.ch8");
fn main() -> Result<(), String>
{
    // sdl context
    let sdlctx = sdl2::init()?;
    // video context
    let videoctx = sdlctx.video()?;
    let width = 64;
    let height = 32;
    // scale factor lolz
    let scalefactor = 24;
    // FIXME: now that this is available, some of these vars need to be removed
    let mut canvas_cd = CanvasData::new(width, height, scalefactor);
    // scaled width
    let swidth: u32 = width * (scalefactor as u32);
    // scaled height
    let sheight: u32 = height * (scalefactor as u32);
    // array of the pixels
    // represnting them in 1 line so we can
    // XOR them bc thats how a chip8 works lolz
    // (its actually a vector)
    let mut pxl_arr = vec![0; (swidth * sheight) as usize];

    let window = videoctx
        .window("lmfao", swidth, sheight)
        .build()
        .map_err(|e| e.to_string())?;

    let mut srfctx = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdlctx.event_pump()?;

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

        srfctx.set_draw_color(Color::RGB(0, 0, 0));
        srfctx.clear();
        srfctx.set_draw_color(Color::RGB(255, 0, 255));
        for i in 0..swidth
        {
            srfctx.draw_point(Point::new(i as i32, i as i32));
        }
        srfctx.present();
        // im testing other stuff rn
        break;
    }

    draww(&srfctx, 103, 56, &FAKESET[0..6], pxl_arr.as_mut_slice());
    let mut c8 = chip8::Chip8::new();
    c8.open("./rom/IBM Logo.ch8");

    c8.cycle();

    Ok(())
}
