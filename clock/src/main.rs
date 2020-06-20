use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minutes_left = (((minutes + 60) % 60) + 60) % 60;

        let hours_from_minutes = if minutes as f32 / 60 as f32 >= 0.0 || minutes % 60 == 0{
            println!("bigger than zero: {}", minutes as f32 / 60 as f32);
            minutes / 60
        } else {
            println!("less than zero: {}", minutes as f32 / 60 as f32);
            (minutes / 60) - 1
        };
        println!("hours_from_minutes: {}", hours_from_minutes);

        let total_hours = ((((hours + hours_from_minutes) + 24) % 24) + 24) % 24;
        // Clock {hours: total_h, minutes: total_m}
        Clock {hours: total_hours, minutes: minutes_left}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let Clock { hours: h, minutes: m} = self;
        let total_hours = (h + ((m + minutes) / 60)) % 24;
        println!("total_hours: {}", total_hours);

        let minutes_left = (((minutes + m + 60) % 60) + 60) % 60;

        Clock {hours: total_hours, minutes: minutes_left}
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = match self.hours {
            0..=9 => format!("0{}", self.hours),
            _ => format!("{}", self.hours), 
        }; 

        let minutes = match self.minutes {
            0..=9 => format!("0{}", self.minutes),
            _ => format!("{}", self.minutes),
        };
        write!(f, "{}:{}", hours, minutes)
    }
}


fn main() {
    // let clock = Clock::new(-25, -160);
    // let clock = Clock::new(24, 160);
    // println!("test add minutes: {}", clock);
    // assert_eq!(Clock::new(-25, -160).to_string(), "20:20");
    let clock = Clock::new(0, 3).add_minutes(-4);
    println!("test : {}", clock);
    assert_eq!(clock.to_string(), "23:59");
}