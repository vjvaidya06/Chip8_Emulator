mod cpu;
mod keybindings;
use std::fs;
use std::process::exit;
use std::process::ExitCode;
use std::time::Instant;
use cpu::{CPU,CpuError};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use std::{thread, time};

fn main() -> ExitCode {
    //set_up_graphics()
    //set_up_input()
    const TIME_PER_INSTRUCTION: f64 = 0.01;
    

    let bindings = keybindings::get_keybindings();
    // println!("bindings: {:?}", bindings);
    //exit(0);
    // println!("Data: {data}");
    // exit(0);
    //Read into array with serde, log error
    //Maybe choose defaults on err
    //Make the CPU object accept bindings
    //Write a function to detect key presses
    // let bindings: []

    let mut cpu = CPU::new(bindings);
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|s| s == "-l") || args.iter().any(|s| s == "--legacy"){
        cpu.toggle_legacy_mode();
    }
    if args.len() < 2{
        println!("Please include the program in the args");
        exit(0);
    }
    println!("Loading game {}", &args[args.len()-1]);
    if let Err(e) = cpu.load_game(&args[args.len()-1]){
        eprintln!("Error loading file, threw error {e}");
        exit(1);
    }
    if let Err(e) = enable_raw_mode(){
        println!("Failed to enable raw mode, threw error {e}");
        exit(1);
    }

    //The terminal can't detect key up moments, re check key presses every cycle
    // cpu.memory[511] = 1;
    // cpu.keypad[1] = true;
    let mut last_instruction_time = Instant::now();
    loop{
        if last_instruction_time.elapsed().as_secs_f64() >= TIME_PER_INSTRUCTION{
            cpu.set_keys();
            // cpu.keypad[14] = true;
            // println!("Emulating cycle\r");
            if let Err(e) = cpu.emulate_cycle(){
                eprintln!("Emulate cycle threw error {e}");
                println!("Program counter at line: {}\r", cpu.pc);
                return ExitCode::from(1);
            }
            // cpu.print_keypad(Some(1));
            if cpu.draw{
                cpu.draw_screen(true);
            }
            cpu.reset_keys();
            //Wait till delay passes, then reset
            last_instruction_time = Instant::now();
            //Find the right number
            //Use real time
        }
    }
    disable_raw_mode();
}
