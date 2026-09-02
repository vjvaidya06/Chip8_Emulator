use std::time::Instant;


pub(super) struct Chip8Key{
    value: bool,
    last_pressed: Instant
}

impl Chip8Key{
    pub(super) fn new() -> Chip8Key{
        Chip8Key { value: false, last_pressed: Instant::now() }
    }
    pub(super) fn value(&self) -> bool{
        return self.value;
    }
    //Always sets value to true
    pub(super) fn press(&mut self){
        self.value = true;
        self.last_pressed = Instant::now();
    }
    //Flips the current state
    //Mainly for the debugger
    pub(super) fn toggle(&mut self){
        self.value = !self.value;
    }
    pub(super) fn update(&mut self) {
        if self.last_pressed.elapsed().as_millis() >= super::constants::KEYDELAY{
            self.value = false;
        }
    }
}
