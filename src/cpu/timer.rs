use std::time::Instant;

pub(super) struct Chip8Timer{
    start_time: Option<Instant>,
    //In seconds
    pub duration: f64
}

impl Chip8Timer{
    pub(super) fn new(start: Option<Instant>, length: f64) -> Self{
        Chip8Timer{
            start_time: start,
            duration: length
        }
    }
    pub(super) fn set_duration(&mut self, new_duration: f64){
        self.start_time = Some(Instant::now());
        self.duration = new_duration;
        // println!("Timer set to duration: {new_duration}");
    }
    //Returns true if timer has elapsed
    pub(super) fn update(&mut self) -> bool{
        if let Some(time) = self.start_time{
            let time_passed = time.elapsed().as_secs_f64();
            if time_passed > 1.0/60.0 && time_passed >= self.duration{
                self.start_time = None;
                return true;
            }
        }
        false
    }
}
