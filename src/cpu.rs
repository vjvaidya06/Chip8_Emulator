mod constants;
mod operations;
mod stack;
use std::cmp::max;
use std::fs::File;
use std::io::Read;
use std::io::{Write, stdout};
use crossterm::QueueableCommand;
use crossterm::{
    execute, queue,
    style::{self, Stylize}, cursor, terminal
};
use simply_colored::*;
//Refactor later if needed
//Set things back to private at the end
#[allow(non_snake_case)]
pub struct CPU{
    pub memory: [u8;4096],
    pub registers: [u8;16],
    I: u16,
    pub pc: usize,
    //May want a wrapper class for this later
    stack: stack::Stack,
    key: [u8;16],
    pub gfx: [bool;64*32],
    legacy_mode: bool,
    on_char: u8,
    off_char: u8
}
impl CPU{
    pub fn new() -> CPU {
        let mut c = CPU {
            memory: [0;4096],
            registers: [0;16],
            I: 0,
            pc: 0x200,
            stack: stack::Stack::new(),
            key: [0;16],
            gfx: [false;64*32],
            legacy_mode: false,
            on_char: 16,
            off_char: 1

        };
        for i in 0..80{
            c.memory[i] = constants::FONTSET[i];
        }
        return c;
    }
    //8XY6 and 8XYE can behave differently depending on the implementation
    pub fn toggle_legacy_mode(&mut self){
        self.legacy_mode = !self.legacy_mode;
    }
    //Fix this
    //How should the emulator behave when open fails?
    pub fn load_game(&mut self, path: &str) -> Result<(), std::io::Error>{
        let mut file = File::open(path)?;
        file.read(&mut self.memory[512..])?;
        Ok(())
    }
    pub fn draw_screen(&self){
        let mut stdout = stdout();
        if let Err(_) = execute!(stdout, terminal::Clear(terminal::ClearType::All), cursor::MoveTo(5, 5)){
            println!("Terminal clear or cursor move failed");
            std::process::exit(1);
        }
        println!();
        println!("{}", "-".repeat(64));
        for i in 0..32{
            print!("|");
            for j in 0..64{
                let wrchar = match self.gfx[(64*i)+j]{
                    true => constants::COLORS[self.on_char as usize],
                    false => constants::COLORS[self.off_char as usize],
                };
                if let Err(_) = stdout.queue(style::PrintStyledContent( "█".with(wrchar))){
                    println!("Terminal print failed");
                    std::process::exit(1);
                }
            }
            print!("|");
            println!();
        }
        println!("{}", "-".repeat(64));
    }
    //Implement blocking by pausing this call while a certain flag is true
    pub fn emulate_cycle(&mut self) -> Result<(), CpuError>{
        operations::perform_op(self)?;
        Ok(())
    }
    pub fn print_all_reg(&self){
        for (i, val) in self.registers.iter().enumerate(){
            println!("V{i}: {val}");
        }
    }
    pub fn print_reg(&self, regnum: usize){
        if regnum > 15{
            println!("Register {regnum} doesn't exist. Please specify a number from 0 to 15");
            return;
        }
        println!("V{regnum}: {}", self.registers[regnum]);
    }
    pub fn disassemble(&self, start: Option<usize>, lines: usize){
        let start = start.unwrap_or(self.pc);
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
        }
    }
}

impl std::error::Error for CpuError {}
