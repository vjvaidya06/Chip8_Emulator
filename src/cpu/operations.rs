//1-7, 9-D are simple 
//8, E, F will need their own blocks
//I should be able to use Hashmap<uxy, fn(u16)
//Just include wrappers for 8, E and F.
//eight(u16), etc.
//They will read the instruction and do whatever
//Maybe add this hashmap to the cpu init step?
//Wait
//just use [fn(u16):16]
//
//REMEMBER: This can access the CPU struct
//When it comes to incrementing pc, let the instruction function handle it

//const lookup: [fn(u16);2] = [op1, op2];
//Change these all to return a result
//Halt on PC out of bounds
//Bitwise and I with 0x0FFF every time
use super::{CPU, CpuError};
use super::constants;
use rand::RngExt;


pub fn decode_op(m1: u8, m2: u8) -> u16{
    let mut ret: u16 = (m1 as u16) << 8;
    ret |= m2 as u16;
    ret
}

pub fn perform_op(c: &mut CPU) -> Result<(), CpuError>{
    if c.pc >= 0x0FFF {
        return Err(CpuError::ProgramCounterOutOfBounds { pc: c.pc, max_memory: (4096) });
    }
    if c.pc % 2 != 0 {
        return Err(CpuError::ProgramCounterInvalidLocation(c.pc));
    }
//    println!("Program Counter: {:X} Performing: {:X}", c.pc, decode_op(c.memory[c.pc], c.memory[c.pc+1]));
    let leading_num: usize = (c.memory[c.pc] >> 4).into();
    //println!("{leading_num}");
    constants::LOOKUP[leading_num](c)?;
    //These are instructions that directly modify the program counter 
    //For other jump instructions we jump by 2 instead of 4
    //We can't underflow on these instructions because jmp 0x0 will be a corner case
    //usize can't be less than 0
    if leading_num != 0x1 && leading_num != 0x2 && leading_num != 0xB{
        c.pc += 2;
    }
    Ok(())
}

//0???
pub(super) fn op_screen(c: &mut CPU) -> Result<(), CpuError>{
    //00E0
    //I thought the whole point of a nested function was that I didn't have to do this
    fn clear(c: &mut CPU){
        //println!("Clearing screen");
        c.gfx.fill(false);
    }
    //00EE
    fn op_return(c: &mut CPU) -> Result<(), CpuError>{
        let top = c.stack.pop()?;
        c.pc = top as usize;
        Ok(())
    }
    let full_op = decode_op(c.memory[c.pc], c.memory[c.pc+1]);
    match full_op {
        0x00E0 => clear(c),
        0x00EE => op_return(c)?,
        //0NNN is unsupported by this emulator
        _ => {}
    };
    Ok(())
}

//1NNN
pub(super) fn jmp_addr(c: &mut CPU) -> Result<(), CpuError>{
    let location = decode_op(c.memory[c.pc], c.memory[c.pc+1]) & 0x0FFF;
    c.pc = location as usize;
    Ok(())
}

//2NNN
pub(super) fn call(c: &mut CPU) -> Result<(), CpuError>{
    let location = decode_op(c.memory[c.pc], c.memory[c.pc+1]) & 0x0FFF;
    c.stack.push(c.pc as u16)?;
    c.pc = location as usize;
    Ok(())
}

//3XNN
pub(super) fn jmp_if_eq(c: &mut CPU) -> Result<(), CpuError>{
    let regnum = c.memory[c.pc] & 0x0F;
    //println!("Checking register {regnum} against {}", c.memory[c.pc+1]);
    if c.registers[regnum as usize] == c.memory[c.pc+1]{
        c.pc += 2;
    }
    Ok(())
}

//4XNN
pub(super) fn jmp_if_neq(c: &mut CPU) -> Result<(), CpuError>{
    let regnum = c.memory[c.pc] & 0x0F;
    //println!("Checking register {regnum} against {}", c.memory[c.pc+1]);
    if c.registers[regnum as usize] != c.memory[c.pc+1]{
        c.pc += 2;
    }
    Ok(())
}

//5XY0
pub(super) fn jmp_if_reg_eq(c: &mut CPU) -> Result<(), CpuError>{
    //Invalid Operation
    if c.memory[c.pc+1] & 0x0F != 0{
        return Err(CpuError::UnknownOpcode(decode_op(c.memory[c.pc], c.memory[c.pc+1])));
    }
    let regnum1 = c.memory[c.pc] & 0x0F;
    let regnum2 = c.memory[c.pc+1] > 4;
    if c.registers[regnum1 as usize] == c.registers[regnum2 as usize]{
        c.pc += 2;
    }
    Ok(())
}

//6XNN
pub(super) fn set_reg(c: &mut CPU) -> Result<(), CpuError>{
    let regnum = c.memory[c.pc] & 0x0F;
    c.registers[regnum as usize] = c.memory[c.pc+1];
    Ok(())
}

//7XNN
pub(super) fn add_reg(c: &mut CPU) -> Result<(), CpuError>{
    let regnum = c.memory[c.pc] & 0x0F;
    c.registers[regnum as usize] += c.memory[c.pc+1];
    Ok(())
}
//8XY?
//Big match case
//For all these operations
//We need a case for when we're touching vf directly
//The answer is that the flag wins
//Always set VF last
pub(super) fn reg_op(c: &mut CPU) -> Result<(), CpuError>{
    let regnum1: u8 = c.memory[c.pc] & 0x0F;
    let regnum2: u8 = c.memory[c.pc+1] >> 4;
    let opnum = c.memory[c.pc+1] & 0x0F;
    //println!("opnum: {:X}, regnum1: {regnum1}, regnum2: {regnum2}", opnum);
    match opnum{
        0x0 => c.registers[regnum1 as usize] = c.registers[regnum2 as usize],
        0x1 => c.registers[regnum1 as usize] |= c.registers[regnum2 as usize],
        0x2 => c.registers[regnum1 as usize] &= c.registers[regnum2 as usize],
        0x3 => c.registers[regnum1 as usize] ^= c.registers[regnum2 as usize],
        0x4 => {
            let (res, overflow) = c.registers[regnum1 as usize].overflowing_add(c.registers[regnum2 as usize]);
            c.registers[regnum1 as usize] = res;
            if overflow {c.registers[0xF] = 1;} else {c.registers[0xF] = 0;}
        }
        0x5 => {
            let (res, overflow) = c.registers[regnum1 as usize].overflowing_sub(c.registers[regnum2 as usize]);
            c.registers[regnum1 as usize] = res;
            if overflow {c.registers[0xF] = 1;} else {c.registers[0xF] = 0;}
        }
        0x6 => {
            let mut temp = 0;
            if c.legacy_mode{
                temp = c.registers[regnum2 as usize] & 1;
                c.registers[regnum1 as usize] = c.registers[regnum2 as usize] >> 1;
            }else{
                temp = c.registers[regnum1 as usize] & 1;
                c.registers[regnum1 as usize] >>= 1;
            }
            c.registers[0xF] = temp;
        }
        0x7 => {
            let (res, overflow) = c.registers[regnum2 as usize].overflowing_sub(c.registers[regnum1 as usize]);
            c.registers[regnum1 as usize] = res;
            if overflow {c.registers[0xF] = 1;} else {c.registers[0xF] = 0;}
        }
        0xE => {
            let temp;
            if c.legacy_mode{
                temp = (c.registers[regnum2 as usize] & 0x80) >> 7;
                c.registers[regnum1 as usize] = c.registers[regnum2 as usize] << 1;
            }else{
                temp = (c.registers[regnum1 as usize] & 0x080) >> 7;
                c.registers[regnum1 as usize] <<= 1;
            }
            c.registers[0xF] = temp;
        }
        _ => return Err(CpuError::UnknownOpcode(decode_op(c.memory[c.pc], c.memory[c.pc+1])))
    }
    Ok(())
}


//9XY0
pub(super) fn jmp_if_reg_neq(c: &mut CPU) -> Result<(), CpuError>{
    //Invalid Operation
    if c.memory[c.pc+1] & 0xF0 != 0{
        return Err(CpuError::UnknownOpcode(decode_op(c.memory[c.pc], c.memory[c.pc+1])));
    }
    let regnum1 = c.memory[c.pc] & 0x0F;
    let regnum2 = c.memory[c.pc+1] > 4;
    if c.registers[regnum1 as usize] != c.registers[regnum2 as usize]{
        c.pc += 2;
    }
    Ok(())
}

//ANNN
pub(super) fn set_i_to_addr(c: &mut CPU) -> Result<(), CpuError>{
    let location = decode_op(c.memory[c.pc], c.memory[c.pc+1]) & 0x0FFF;
    c.I = location;
    Ok(())
}

//BNN
pub(super) fn jmp_plus(c: &mut CPU) -> Result<(), CpuError>{
    c.pc = (c.registers[0] as usize) + (decode_op(c.memory[c.pc], c.memory[c.pc+1]) & 0x0FFF) as usize;
    Ok(())
}

//CXNN 
pub(super) fn rand_and(c: &mut CPU) -> Result<(), CpuError>{
    let randint: u8 = rand::random_range(0..=255);
    let regnum = c.memory[c.pc] & 0x0F;
    //println!("Setting V{regnum} to {randint} & {}", c.memory[c.pc+1]);
    c.registers[regnum as usize] = randint & c.memory[c.pc + 1];
    //println!("V{regnum}: {:X}", c.registers[regnum as usize]);
    Ok(())
}

//DXYN
pub(super) fn draw_sprite(c: &mut CPU) -> Result<(), CpuError>{
    let regnum1 = c.memory[c.pc] & 0x0F;
    let regnum2 = (c.memory[c.pc+1] & 0xF0) >> 4;
    let n = c.memory[c.pc+1] & 0x0F;
    let mut location = (64*((c.registers[regnum2 as usize] as usize))) + c.registers[regnum1 as usize] as usize - 2;
    for i in 0..n{
        let bitmap: u8 = c.memory[(c.I + i as u16) as usize];
        //Fetch the top bit, xor it with the equivalent screen bit, 
        //update the collision flag if needed
        for j in 0..8{
            let bit = if ((bitmap << j) & 0b10000000) >> 7 == 1 {true} else {false};
            if bit && c.gfx[location]{
                //Collision detected
                c.registers[0x0F] = 1;
            }
            c.gfx[location] = c.gfx[location] ^ bit;
            location += 1;
        }
        //64 - 8
        location += 56;
    }
    Ok(())
}

//EX??
pub(super) fn e_key_op(c: &mut CPU) -> Result<(), CpuError>{
    let regnum = c.memory[c.pc] & 0x0F;
    let keynum = c.registers[regnum as usize];
    let cmp = match c.memory[c.pc+1]{
        0x9E => c.keypad[keynum as usize],
        0xA1 => !c.keypad[keynum as usize],
        _ => return Err(CpuError::UnknownOpcode(decode_op(c.memory[c.pc], c.memory[c.pc+1])))

    };
    if cmp{
        c.pc += 2;
    }
    Ok(())
}

//FX??
pub(super) fn f_op(c: &mut CPU) -> Result<(), CpuError>{
    let regnum = c.memory[c.pc] & 0x0F;
    match c.memory[c.pc+1]{
        0x07 => get_delay(c, regnum),
        0x0A => wait_for_key(c, regnum),
        0x15 => println!("timer"),
        0x18 => println!("sound"),
        0x1E => add_vx_i(c, regnum),
        0x29 => get_sprite_addr(c, regnum),
        0x33 => println!("bcd"),
        0x55 => println!("memory"),
        0x65 => println!("memory"),
        _ => return Err(CpuError::UnknownOpcode(decode_op(c.memory[c.pc], c.memory[c.pc+1])))
    }
    Ok(())
}

fn get_delay(c: &mut CPU, regnum: u8){
    println!("unimplemented");
    // c.registers[regnum as usize] = c.delay_timer;
}

fn add_vx_i(c: &mut CPU, regnum: u8){
    c.I += c.registers[regnum as usize] as u16;
}

fn get_sprite_addr(c: &mut CPU, regnum: u8){
    c.I = ((c.registers[regnum as usize] & 0x0F) * 5) as u16;
}

fn wait_for_key(c: &mut CPU, regnum: u8){
    if c.keypad[regnum as usize]{
        c.blocking = None;
    }
    else{
        c.blocking = Some(c.registers[regnum as usize]);
    }
}
