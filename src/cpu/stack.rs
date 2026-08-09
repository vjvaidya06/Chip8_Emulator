//Our stack is a fixed size, why use a slower vector?
//Instead just used a fixed size stack array
//We'll use a wrapper class for push/pop, that'll return
//Result<(), CpuError>;

use super::CpuError;
pub(super) struct Stack{
    stack: [u16;12],
    sp: u8
}

//push, pop
impl Stack{
    pub fn new() -> Stack{
        Stack {
            stack: [0;12],
            sp: 0
        }
    }
    pub fn pop(&mut self) -> Result<u16, CpuError>{
        if self.sp == 0{
            return Err(CpuError::StackUnderflowError);
        }
        self.sp -= 1;
        Ok(self.stack[self.sp as usize])
    }
    pub fn push(&mut self, val: u16) -> Result<(), CpuError>{
        if self.sp == 12{
            return Err(CpuError::StackOverflowError);
        }
        //println!("pushing to stack");
        self.stack[self.sp as usize] = val;
        self.sp += 1;
        Ok(())
    }
    pub fn print_stack(&self){
        if self.sp == 0{
            println!("Stack empty");
        }
        for i in (0..self.sp).rev(){
            println!("At location {i}: {:04X}", self.stack[i as usize]);
        }
    }
}
