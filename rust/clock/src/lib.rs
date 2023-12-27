use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut minutes = hours * 60 + minutes;
        let mut hours = minutes / 60;
        minutes %= 60;

        if minutes < 0 {
            minutes += 60;
            hours -= 1;
        }

        hours = (hours % 24 + 24) % 24;

        Clock { hours, minutes }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        let minutes = self.minutes + minutes;
        Clock::new(self.hours, minutes)
    }
}


