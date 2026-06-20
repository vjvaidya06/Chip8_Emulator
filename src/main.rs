mod cpu;
use cpu::{CPU,CpuError};

//temp 
fn main() {
    //let i = operations::decode_op(0x1A, 0xE8);
    //println!("{:X}", i);
    //println!("{}", 0b101);
    let mut cpu = CPU::new();
    //TODO: We really need a tester for this
    /*cpu.memory[0x200] = 0x63;
    cpu.memory[0x201] = 0x12;
    cpu.memory[0x202] = 0x62;
    cpu.memory[0x203] = 0xF0;
    cpu.memory[0x204] = 0x82;
    cpu.memory[0x205] = 0x31;
    cpu.memory[0x206] = 0x82;
    cpu.memory[0x207] = 0x32;
    cpu.memory[0x208] = 0x82;
    cpu.memory[0x209] = 0x33;
    cpu.memory[0x20A] = 0x82;
    cpu.memory[0x20B] = 0x34;
    for i in 0..6{
        cpu.emulate_cycle();
        println!("Register 2: {}, Register 3: {}", cpu.registers[2], cpu.registers[3]);
        println!("---------------------------------------------------------------------");
    }*/
    println!("{}", cpu.registers[2]);
    println!("{}", cpu.registers[3]);
    //cpu.emulate_cycle();
    // cpu.gfx[0] = true;
    // cpu.gfx[32] = true;
    // cpu.draw_screen();
    //Jump to AAB
    //cpu.memory[0x200] = 0x1A;
    //cpu.memory[0x201] = 0xAB;
    /*cpu.registers[0] = 1;
    for i in 0..16{
        cpu.registers[i] = i as u8;
    }
    //cpu.registers[8] = 3;
    //good
    //cpu.memory[0x200] = 0x58;
    //cpu.memory[0x201] = 0x31;
    cpu.memory[0x200] = 0x68;
    cpu.memory[0x201] = 0x03;
    cpu.memory[0x202] = 0x58;
    cpu.memory[0x203] = 0x30;
    cpu.memory[0x204] = 0x75;
    cpu.memory[0x205] = 0x02;
    cpu.memory[0x206] = 0x78;
    cpu.memory[0x207] = 0x02;
    cpu.memory[0x208] = 0x12;
    cpu.memory[0x209] = 0x04;
    for i in 0..6{
        if let Err(e) = cpu.emulate_cycle(){
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
    cpu.print_reg();*/
    //println!("This should complain: {}", cpu.memory[0x200]);
    //cpu.emulate_cycle();
    //println!("{:?}", cpu.load_game("./utils.rs"));
}

//final
fn real_main() {
    //set_up_graphics()
    //set_up_input()

    let cpu = CPU::new();
    //cpu.load_game();

    loop{
        //cpu.emulate_cycle();
        if cpu.registers[15] != 0{
            //draw_graphics();
        }

        //cpu.set_keys();
    }
}
