use std::rc::Rc;
use std::time::Instant;

use crate::cpu::CPU;
use softbuffer::Surface;
use winit::application::ApplicationHandler;
use winit::event::{WindowEvent, KeyEvent};
use winit::event_loop::{self, ActiveEventLoop, EventLoop};
use winit::platform::modifier_supplement::KeyEventExtModifierSupplement;
use winit::window::{Window, WindowId};
use std::num::NonZeroU32;

// #[derive(Default)]
pub struct Chip8Frontend{
    window: Option<Rc<Window>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    cpu: CPU,
    //Time each instruction should take
    time_per_instruction: f64,
    last_instruction_time: Instant,
}

impl Chip8Frontend{
    pub fn new(cpu: CPU, time_per_inst: f64) -> Chip8Frontend{
        Chip8Frontend{
            window: None,
            surface: None,
            cpu: cpu,
            time_per_instruction: time_per_inst,
            last_instruction_time: Instant::now(),
        }
    }
}

impl ApplicationHandler for Chip8Frontend{
    fn resumed(&mut self, event_loop: &ActiveEventLoop){
        if self.window.is_none(){
            let window_attributes = Window::default_attributes().with_title("Chip 8 Emulator");
            let window = event_loop.create_window(window_attributes).expect("Window creation failed");
            self.window = Some(Rc::new(window));

            // let Ok(ctx) = softbuffer::Context::new(self.window.clone()) else{
            // unwrap safe here since I just set it to Some()
            let Ok(ctx) = softbuffer::Context::new(self.window.as_ref().unwrap().clone()) else{
                println!("Softbuffer context creation fail");
                std::process::exit(1);
            };
            let Ok(mut surface) = softbuffer::Surface::new(&ctx, self.window.as_ref().unwrap().clone()) else{
                println!("Softbuffer surface creation fail");
                std::process::exit(1);
            };
            surface.resize(NonZeroU32::new(64).unwrap(), NonZeroU32::new(32).unwrap()).unwrap();
            // surface.resize(NonZeroU32::new(640).unwrap(), NonZeroU32::new(320).unwrap()).unwrap();
            self.surface = Some(surface);
        }
    }

    fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            window_id: WindowId,
            event: WindowEvent,
        ) {
        match event{
            WindowEvent::RedrawRequested => {
                // println!("Redraw logic");
                if let Some(surface) = &mut self.surface{
                    let Ok(mut buffer) = surface.buffer_mut() else{
                        println!("surface.buffer_mut() fail");
                        return;
                    };
                    self.cpu.draw_screen_softbuffer(&mut buffer);

                    if let Err(e) = buffer.present(){
                        println!("Softbuffer buffer.present failed with error {e}");
                    }
                }
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
                if let winit::keyboard::Key::Character(s) = event.key_without_modifiers(){
                    if let Some(c) = s.chars().next(){
                        //My key functions check, but I assume this saves some overhead
                        if self.cpu.valid_character(c){
                            match event.state{
                                winit::event::ElementState::Pressed => self.cpu.key_down(c),
                                winit::event::ElementState::Released => self.cpu.key_up(c),
                            }
                        }
                    }
                }
            }
            _ => {
                // println!("Event: {:?}", event);
            }
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.last_instruction_time.elapsed().as_secs_f64() >= self.time_per_instruction{
            if let Err(e) = self.cpu.emulate_cycle(){
                eprintln!("Emulate cycle threw error {e}");
                println!("Program counter at line: {}", self.cpu.pc);
                std::process::exit(1);
            }
            if self.cpu.draw{
                if let Some(window) = &self.window && let Some(_) = &self.surface{
                    window.request_redraw();
                }
            }
            self.last_instruction_time = Instant::now();
        }
    }
}
