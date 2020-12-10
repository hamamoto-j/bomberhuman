use std::time::Instant;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Timer {
    left_time: i32,
    count_time: i32,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            left_time: 180,
            count_time: 60,
        }
    }

    //    fn elapsed(&self) -> u64 {
    //       let elapsed = self.start_time.elapsed();
    //       elapsed.as_secs()
    //  }

    //   pub fn left(&self) -> u64 {
    //       180 - self.elapsed()
    //   }

    pub fn update(&mut self) {
        self.count_time -= 1;
        if self.count_time < 0 {
            self.left_time -= 1;
            self.count_time = 60;
        }
    }

    pub fn left(&self) -> i32 {
        self.left_time
    }
}
