use std::fmt;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock {
    minutes: i32,
}

const MIN_PER_DAY: i32 = 24*60;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let m = (hours*60 + minutes) % MIN_PER_DAY;
        let m = (m + MIN_PER_DAY) % MIN_PER_DAY;
        Self {minutes: m}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
       Clock::new(0, minutes + self.minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes/60, self.minutes%60)
    }
}
