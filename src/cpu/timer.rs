use std::time::Instant;

pub(super) struct Chip8Timer{
    last_check: Option<Instant>,
    //In seconds
    pub duration: f64
}

impl Chip8Timer{
    pub(super) fn new(start: Option<Instant>, length: f64) -> Self{
        Chip8Timer{
            last_check: start,
            duration: length
        }
    }
    pub(super) fn set_duration(&mut self, new_duration: f64){
        self.last_check = Some(Instant::now());
        //Stored in ticks, convert to seconds
        self.duration = new_duration/60.0;
        // println!("Timer set to duration: {new_duration}");
    }
    //Returns true if timer has elapsed
    pub(super) fn update(&mut self) -> bool{
        if let Some(time) = self.last_check{
            let time_passed = time.elapsed().as_secs_f64();
            if time_passed > 1.0/60.0{
                self.duration -= time_passed;
                self.last_check = Some(Instant::now());
                if self.duration <= 0.0{
                    self.duration = 0.0;
                    self.last_check = None;
                    return true;
                }
            }
            // if time_passed > 1.0/60.0 && time_passed >= self.duration{
            //     self.last_check = None;
            //     return true;
            // }
        }
        false
    }
}
