use std::time::{Instant,Duration};

pub struct FramesCtrl {
    // pub target: u32, // target frames per seconds
    frame_duration: Duration, // duration of one frame
    last_frame: Instant, // end of the last frame
    frame_start: Instant,
    pub dtime: f64,
}

impl FramesCtrl {
    pub fn init(target: u32) -> Self {
        FramesCtrl { 
            // target: target,
            frame_duration: Duration::from_secs_f64(1.0 / target as f64),
            last_frame: Instant::now(), frame_start: Instant::now(), dtime: 0.0 
        }
    }
    pub fn start_frame(&mut self) {
        self.frame_start = Instant::now();
        self.dtime = self.frame_start.duration_since(self.last_frame).as_secs_f64();
        self.last_frame = self.frame_start;

    }

    pub fn end_frame(&self) {
        let elapsed = self.frame_start.elapsed();
        if elapsed < self.frame_duration {
            std::thread::sleep(self.frame_duration - elapsed);
        }  
    }

    pub fn fps(&self) -> f64 {
        let dt = self.dtime;
        if dt > 0.0 {
            1.0 / dt
        } else {
            0.0
        }
    }
}