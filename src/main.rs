mod cpu;
mod keybindings;
use std::fs;
use std::process::exit;
use cpu::{CPU,CpuError};

fn main() {
    //set_up_graphics()
    //set_up_input()
    

    let bindings = keybindings::get_keybindings();
    println!("bindings: {:?}", bindings);
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
    if args.len() < 2{
        println!("Please include the program in the args");
        exit(0);
    }
    println!("Loading game {}", &args[1]);
    if let Err(e) = cpu.load_game(&args[1]){
        eprintln!("Error loading file, threw error {e}");
        exit(1);
    }

    loop{
        if let Err(e) = cpu.emulate_cycle(){
            eprintln!("Emulate cycle threw error {e}");
        }
        if cpu.draw{
            cpu.draw_screen(false);
        }

        //cpu.set_keys();
    }
}
