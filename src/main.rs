mod cpu;
mod keybindings;
mod winit_handler;
use std::fs;
use std::num::NonZeroU32;
use std::process::exit;
use std::process::ExitCode;
use std::rc::Rc;
use std::time::Duration;
use std::time::Instant;
use cpu::{CPU,CpuError};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use winit::event_loop;
use winit::raw_window_handle::HasRawDisplayHandle;
use std::{thread, time};
use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::KeyCode,
    window::Window
};
use softbuffer::{Context, Surface};
use winit_handler::Chip8Frontend;

fn older_main() -> ExitCode {
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

fn old_main() -> ExitCode{
    //Just testing, remove all these unwraps later
    let event_loop = EventLoop::new().expect("Winit event loop creation fail");
    event_loop.set_control_flow(ControlFlow::Poll);
    let window = event_loop.create_window(Window::default_attributes()).expect("OS error when creating winit window");
    let context = Context::new(event_loop.owned_display_handle()).expect("Context creation fail");
    let mut surface = Surface::new(&context, window).expect("Surface creation fail");
    surface.resize(NonZeroU32::new(64).unwrap(), NonZeroU32::new(32).unwrap()).unwrap();
    let mut buffer = surface.buffer_mut().unwrap();

    for index in 0..2048{
        buffer[index] = 0x00000000;
    }
    buffer[1000] = 0x00FFFFFF;
    buffer.present().unwrap();
    
    std::thread::sleep(Duration::from_secs(10));
    
    // let mut surface = Surface::new(&context, )
    ExitCode::from(0)
}

fn main() -> ExitCode{
    let bindings = keybindings::get_keybindings();

    let mut cpu = CPU::new(bindings);

    //TODO: Make more efficient
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|s| s == "-l") || args.iter().any(|s| s == "--legacy"){
        cpu.toggle_legacy_mode();
    }
    if args.iter().any(|s| s == "-w") || args.iter().any(|s| s == "--wrapping"){
        cpu.toggle_wrapping();
    }
    if args.iter().any(|s| s == "-o") || args.iter().any(|s| s == "--odds"){
        cpu.toggle_odds();
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

    let Ok(event_loop) = EventLoop::new() else{
        println!("Error creating event loop");
        return ExitCode::from(1);
    };

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut winit_app = Chip8Frontend::new(cpu, 0.005);

    event_loop.run_app(&mut winit_app);

    ExitCode::from(0)
}
