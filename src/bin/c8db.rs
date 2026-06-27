use Chip_8::cpu::CPU;
use std::io::{self, Write};
use std::collections::HashSet;

fn main(){
    let mut c = CPU::new();
    //println!("Hello world");
    let mut input = String::new();
    let mut break_points:HashSet<usize> = HashSet::new();
    loop{
        input.clear();
        print!("(c8db) ");
        if let Err(e) = io::stdout().flush(){
            eprintln!("stdout error: {e}");
        }
        io::stdin().read_line(&mut input).expect("Stdin failure");
        let tokens: Vec<&str> = input.split_whitespace().collect();
        parse_input(&tokens, &mut c, &mut break_points);
    }
}

//Write a function for most of them
fn parse_input(tokens: &[&str], c: &mut CPU, b: &mut HashSet<usize>){
    if tokens.len() < 1 {
        return;
    }
    match tokens[0]{
       "load"|"l" => load(tokens, c),
       "disassemble"|"dis" => disassemble(tokens, c),
       "print"|"p" => print_register(tokens, c),
       "step"|"s" => step(c),
       "screen"|"sc" => c.draw_screen(),
       "jump"|"j" => jump(tokens, c),
       "break"|"b" => set_breakpoint(tokens, c, b),
       "continue"|"c" => continue_till_breakpoint(c, b),
       "delete"|"d" => delete_breakpoint(tokens, c, b),
       "help"|"h" => help(),
       "quit" => std::process::exit(0),
        _ => println!("Undefined command: {}, type help or h for help", tokens[0])
    }
}

fn help(){
    println!("Chip 8 Debugger (C8DB), inspired by GNU Debugger (GDB)\n\n");
    println!("Below are a list of all possible commands\n\n");
    println!("load/l [FILEPATH]                              Loads the binary file located at the selected path.\n");
    println!("print/p [NUM]/all                              Prints a register from 0-15, or type all to print them all\n");
    println!("disassemble/dis [NUM LINES=10] [ADDRESS=PC]    Prints the assembly instructions starting at the selected memory address.\n");
    println!("step/s                                         Steps one instruction.\n");
    println!("screen/sc                                      Displays the Chip 8 screen.\n");
    println!("jump/j [ADDRESS]                               Jumps to the selected memory address.\n");
    println!("break/b [ADDRESS]                              Sets a break point at the selected memory address.\n");
    println!("delete/d [ADDRESS]                             Deletes the break point at the selected memory address.\n");
    println!("continue/c                                     Runs the program until it encounters a break point, or ends.\n");
    println!("help/h                                         You're already here.\n");
    println!("Any memory address can be input as decimal or hexadecimal. If using hex, prefix with 0x.\n");
}

fn delete_breakpoint(tokens: &[&str], c: &mut CPU, b: &mut HashSet<usize>){
    if tokens.len() < 2{
        println!("Please include a line number, in decimal or hex. If it's in hex, prefix it with 0x");
        return;
    }
    let bp;
    if tokens[1].starts_with("0x"){
        bp = usize::from_str_radix(tokens[1].strip_prefix("0x").unwrap_or("How did this happen?"), 16).ok();
    }
    else{
        bp = tokens[1].parse::<usize>().ok();
    }
    if let Some(addr) = bp{
        if !b.contains(&addr){
            println!("Break point not found at address 0x{:04X}", addr);
            return;
        }
        b.remove(&addr);
        println!("Removed break point at addres 0x{:04X}", addr);
    }
}

fn set_breakpoint(tokens: &[&str], c: &mut CPU, b: &mut HashSet<usize>){
    if tokens.len() < 2{
        println!("Please include a line number, in decimal or hex. If it's in hex, prefix it with 0x");
        return;
    }
    let bp;
    if tokens[1].starts_with("0x"){
        bp = usize::from_str_radix(tokens[1].strip_prefix("0x").unwrap_or("How did this happen?"), 16).ok();
    }
    else{
        bp = tokens[1].parse::<usize>().ok();
    }
    if let Some(addr) = bp{
        if addr > 0xFFF{
            println!("Address to big, pick an address between 0 and 4096");
            return;
        }
        if addr % 2 != 0{
            println!("Address must be an even number");
            return;
        }
        b.insert(addr);
        println!("Set breakpoint at line 0x{:04X}", addr);
    }
}

fn continue_till_breakpoint(c: &mut CPU, b: &mut HashSet<usize>){
    while {
        let _ = c.emulate_cycle();
        !b.contains(&c.pc) && c.pc != 4094
    }{}
    if c.pc == 4094{
        println!("Reached end of program");
        return;
    }
    println!("Encountered break point at 0x{:04X}", c.pc);
}

fn jump(tokens: &[&str], c: &mut CPU){
    if tokens.len() < 2{
        println!("Please provide an address to jump to. If it's in hex, prefix it with 0x");
        return;
    }
    let start;
    if tokens[1].starts_with("0x"){
        start = usize::from_str_radix(tokens[1].strip_prefix("0x").unwrap_or("How did this happen?"), 16).ok();
    }
    else{
        start = tokens[1].parse::<usize>().ok();
    }
    if let Some(addr) = start{
        if addr > 0xFFF{
            println!("Address to big, pick an address between 0 and 4096");
            return;
        }
        if addr % 2 != 0{
            println!("Address must be an even number");
            return;
        }
        c.pc = addr;
        return;
    }
    println!("Invalid address. Please input an address between 0 and 4096. If it's in hex, prefix it with 0x.");
}

fn load(tokens: &[&str], c: &mut CPU){
    if tokens.len() < 2{
        println!("Not enough arguments");
        return;
    }
    if let Err(_) = c.load_game(tokens[1]){
        println!("File not found");
        return;
    }
    println!("Loaded {}", tokens[1]);
}

fn disassemble(tokens: &[&str], c: &mut CPU){
    println!("0x200 and 201: {:X}{:X}", c.memory[0x200], c.memory[0x201]);
    if tokens.len() < 2{
        c.disassemble(None, 10);
        return;
    }
    let Ok(lines) = tokens[1].parse::<usize>() else{
        println!("Please input a positive integer in decimal");
        return;
    };
    let mut start = None;
    if tokens.len() > 2{
        if tokens[2].starts_with("0x"){
            start = usize::from_str_radix(tokens[2].strip_prefix("0x").unwrap_or("How did this happen?"), 16).ok();
        }
        else{
            start = tokens[2].parse::<usize>().ok();
        }
    }
    c.disassemble(start, lines);
}

fn print_register(tokens: &[&str], c: &mut CPU){
    if tokens.len() < 2{
        println!("Not enough arguments");
        return;
    }
    if tokens[1] == "all"{
        c.print_all_reg();
        return;
    }
    let Ok(regnum) = tokens[1].parse::<usize>() else {
        println!("Not an integer");
        return;
    };
    c.print_reg(regnum);
}

fn step(c: &mut CPU){
    if let Err(e) = c.emulate_cycle(){
        println!("Program received error: {}", e);
    }
}

