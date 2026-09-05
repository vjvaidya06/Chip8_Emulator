mod constants;
mod operations;
mod stack;
mod timer;
mod key;
use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::stdout;
use std::time::Duration;
use crossterm::QueueableCommand;
use crossterm::event::KeyModifiers;
use crossterm::terminal::disable_raw_mode;
use crossterm::{
    execute,
    style::{self, Stylize, Print}, cursor, terminal,
    event::{self, Event, KeyCode},
};
use simply_colored::*;
use winit::window::Window;
use std::rc::Rc;
//Refactor later if needed
//Set things back to private at the end
#[allow(non_snake_case)]
pub struct CPU{
    pub memory: [u8;4096],
    registers: [u8;16],
    I: u16,
    pub pc: usize,
    stack: stack::Chip8Stack,
    pub keypad: [bool;16],
    gfx: [bool;64*32],
    legacy_mode: bool,
    wrapping: bool,
    delay_timer: timer::Chip8Timer,
    sound_timer: timer::Chip8Timer,
    //If blocking, set this to Some(register)
    //Register is the register the next key press will be stored in
    blocking : Option<u8>,
    on_char: u8,
    off_char: u8,
    bindings: HashMap<char, usize>,
    pub draw: bool
}
impl Default for CPU{
    fn default() -> Self{
        Self::new(None)
    }
}
impl CPU{
    pub fn new(bindings: Option<HashMap<char, usize>>) -> CPU {
        let bindings = match bindings{
            Some(map) => map,
            None => {
                HashMap::from([
                ('1', 1),
                ('2', 2),
                ('a', 7),
                ('3', 3),
                ('4', 12),
                ('c', 11),
                ('q', 4),
                ('x', 0),
                ('v', 15),
                ('w', 5),
                ('d', 9),
                ('r', 13),
                ('e', 6),
                ('f', 14),
                ('s', 8),
                ('z', 10),
            ])
            }
        };
        let mut c = CPU {
            memory: [0;4096],
            registers: [0;16],
            I: 0,
            pc: 0x200,
            stack: stack::Chip8Stack::new(),
            keypad: [false;16],
            gfx: [false;64*32],
            legacy_mode: false,
            wrapping: false,
            delay_timer: timer::Chip8Timer::new(None, 0.0),
            sound_timer: timer::Chip8Timer::new(None, 0.0),
            blocking: None,
            on_char: 16,
            off_char: 1,
            bindings,
            draw: false

        };
        for i in 0..80{
            c.memory[i] = constants::FONTSET[i];
        }
        c
    }
    //8XY6 and 8XYE can behave differently depending on the implementation
    pub fn toggle_legacy_mode(&mut self){
        self.legacy_mode = !self.legacy_mode;
        println!("Legacy Mode: {}", self.legacy_mode);
    }
    pub fn toggle_wrapping(&mut self){
        self.wrapping = !self.wrapping;
        println!("Wrapping: {}", self.wrapping);
    }

    //How should the emulator behave when open fails?
    pub fn load_game(&mut self, path: &str) -> Result<(), std::io::Error>{
        let mut file = File::open(path)?;
        file.read(&mut self.memory[512..])?;
        Ok(())
    }
    pub fn draw_screen(&self, debug: bool){
        let mut stdout = stdout();
        if !debug{
            if let Err(e) = execute!(stdout, terminal::Clear(terminal::ClearType::All), cursor::MoveTo(1, 1)){
            // if let Err(e) = stdout.queue(cursor::MoveTo(1, 1)){
                eprintln!("Terminal clear or cursor move failed, threw error: {e}");
                std::process::exit(1);
            }
        }
        // println!("\r");
        // println!("{}\r", "-".repeat(64));
        // TODO: might not need all these if lets
        if let Err(e) = stdout.queue(Print(format!("\r\n{}\r", "-".repeat(64)))){
            eprintln!("stdout failure: returned {e}");
        }
        for i in 0..32{
            // print!("|");
            if let Err(e) = stdout.queue(Print("|")){
                eprintln!("stdout failure: returned {e}");
            }
            for j in 0..64{
                let wrchar = match self.gfx[(64*i)+j]{
                    true => constants::COLORS[self.on_char as usize],
                    false => constants::COLORS[self.off_char as usize],
                };
                if let Err(e) = stdout.queue(style::PrintStyledContent( "█".with(wrchar))){
                    eprintln!("Terminal print failed, threw error: {e}");
                    std::process::exit(1);
                }
            }
            if let Err(e) = stdout.queue(Print("|\r\n")){
                eprintln!("stdout failure: returned {e}");
            }
            // print!("|");
            // println!("\r");
        }
        // println!("{}\r", "-".repeat(64));
        if let Err(e) = stdout.queue(Print(format!("{}\r", "-".repeat(64)))){
            eprintln!("stdout failure: returned {e}");
        }
        if let Err(e) = stdout.flush(){
            eprintln!("Flush failure, returned {e}");
        }
    }
    pub fn decrement_timers(&mut self){
        self.delay_timer.update();
        if self.sound_timer.update(){
            // println!("BEEP\r");
        }
    }
    pub fn valid_character(&self, key: char) -> bool{
        self.bindings.contains_key(&key)
    }
    pub fn key_down(&mut self, key: char){
        if self.bindings.contains_key(&key){
            self.keypad[self.bindings[&key]] = true;
        }
    }
    pub fn key_up(&mut self, key: char){
        if self.bindings.contains_key(&key){
            self.keypad[self.bindings[&key]] = false;
        }
    }
    pub fn draw_screen_winit<'a>(&self, buffer: &mut softbuffer::Buffer<'a, Rc<Window>, Rc<Window>>){
        for i in 0..buffer.len(){
            buffer[i] = match self.gfx[i]{
                true => 0x00FFFFFF,
                false => 0x00000000,
            }
        }
    }
    pub fn toggle_key(&mut self, key: u8){
        if key > 15{
            println!("Key must be between 0 and 15");
            return;
        }
        self.keypad[key as usize] = !self.keypad[key as usize];
    }
    pub fn emulate_cycle(&mut self) -> Result<(), CpuError>{
        if self.draw{
            //This breaks c8db
            // self.draw_screen(false);
            //Until I have real graphics, just let main/c8db handle it
            //Make the draw flag public
            self.draw = false;
        }
        //If blocking, wait for a certain key press
        /*match self.blocking{
            None => operations::perform_op(self)?,
            Some(key) => {
                println!("Waiting for key {key}");
                if self.keypad[key as usize]{
                    self.blocking = None;
                }
            }
        }*/
        //If not blocking, run the next instruction
        if self.blocking.is_none(){
            operations::perform_op(self)?;
        }
        self.decrement_timers();
        Ok(())
    }
    //Assumes the terminal is in raw mode
    //Deprecated
    pub fn set_keys(&mut self){
        while let Ok(true) = event::poll(Duration::from_millis(0)){
            if let Ok(Event::Key(key_event)) = event::read(){
                if let KeyCode::Char(c) = key_event.code{
                    if key_event.modifiers.contains(KeyModifiers::CONTROL){
                        println!("Exiting");
                        println!("Program counter at {:X}", self.pc);
                        //TODO: RAII guard so I don't have to do this
                        disable_raw_mode();
                        std::process::exit(0);
                    }
                    if self.bindings.contains_key(&c){
                        // println!("key {c} pressed\r");
                        // println!("Found {c}, setting {} to true", self.bindings[&c]);
                        if let Some(register) = self.blocking{
                            println!("Setting register {register} to {}\r", self.bindings[&c]);
                            self.registers[register as usize] = self.bindings[&c] as u8;
                            self.blocking = None;
                            println!("Reg: {}\r", self.registers[register as usize]);
                        }
                        self.keypad[self.bindings[&c]] = true;
                        // self.print_keypad(None);
                    }
                }
            }
        }
    }
    pub fn reset_keys(&mut self){
        self.keypad.fill(false);
    }
    //Below are all debug functions meant to be used with c8db
    pub fn print_all_reg(&self){
        for (i, val) in self.registers.iter().enumerate(){
            println!("V{i}: {val} or 0x{:02X}", val);
        }
        println!("I: {} or 0x{:03X}", self.I, self.I);
    }
    pub fn print_i_reg(&self){
        println!("I: {} or 0x{:03X}", self.I, self.I);
    }
    pub fn print_reg(&self, regnum: usize){
        if regnum > 15{
            println!("Register {regnum} doesn't exist. Please specify a number from 0 to 15");
            return;
        }
        println!("V{regnum}: {} or 0x{:02X}", self.registers[regnum], self.registers[regnum]);
    }
    pub fn disassemble(&self, st: Option<usize>, lines: usize){
        let start = match self.pc{
            x if x >= 0x200 => st.unwrap_or(max(0x200, self.pc - 8)),
            x if x < 0x008 => st.unwrap_or(0x000),
            _ => st.unwrap_or(self.pc - 8)
        };
        if start > 4095{
            println!("Starting line too big, memory ends at 4095");
            return;
        }
        if start % 2 != 0{
            println!("Can't start on an odd number");
            return;
        }
        
        let lines = std::cmp::min(lines*2, (4096-start)*2);
        for i in (0..lines).step_by(2){
            if self.pc == start+i{
                print!("=> ");
            }
            else{
                print!("   ");
            }
            println!("{BLUE}0x{:04X}{WHITE}:    {RED}0x{:04X}{RESET}", start+i, operations::decode_op(self.memory[start+i], self.memory[start+i+1]));
        }
    }
    pub fn print_stack(&self){
        self.stack.print_stack();
    }
    pub fn print_keypad(&self, keynum: Option<u8>){
        match keynum{
            None => {
                println!("Current Keypad status:");
                for (i, key) in self.keypad.iter().enumerate(){
                    println!("Key {i}: {key}");
                }
            }
            Some(key) if key > 15 => println!("Please include a number from 0 to 15"),
            Some(key) => println!("Key {key}: {}", self.keypad[key as usize])
        }
    }
}

#[derive(Debug)]
pub enum CpuError {
    /// The Program Counter reached an address outside valid memory bounds.
    ProgramCounterOutOfBounds { pc: usize, max_memory: usize },
    /// The Program Counter reached odd address in memory
    ProgramCounterInvalidLocation(usize),
    /// The opcode fetched from memory is not a recognized Chip-8 instruction.
    UnknownOpcode(u16),
    /// The stack pointer overflowed or underflowed.
    StackUnderflowError,

    StackOverflowError,
    OutofBoundsAccess
}

impl std::fmt::Display for CpuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CpuError::ProgramCounterOutOfBounds { pc, max_memory } => {
                write!(f, "CPU Error: Program Counter (0x{:04X}) went out of bounds (Max: 0x{:04X})", pc, max_memory)
            }
            CpuError::ProgramCounterInvalidLocation(pc) => {
                write!(f, "CPU Error: Program Counter (0x{:04X}) located at the middle of an instruction (odd index in memory)", pc)
            }
            CpuError::UnknownOpcode(op) => write!(f, "CPU Error: Unknown opcode 0x{:04X}", op),
            //CpuError::StackError(msg) => write!(f, "CPU Error: Stack error: {}", msg),
            CpuError::StackOverflowError => write!(f, "Stack overflow error, the Chip 8 stack only allows up to 12 levels of nesting"),
            CpuError::StackUnderflowError => write!(f, "Stack undeflow error, there's nothing currently on the stack"),
            CpuError::OutofBoundsAccess => write!(f, "Out of bounds access attempted")
        }
    }
}

impl std::error::Error for CpuError {}
