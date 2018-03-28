use std::fmt;

pub struct TimeLine {
    mission: String,
    time: String,
    track: String,
    destination: String,
}

impl TimeLine {
    pub fn new(mission: &str, time: &str, track: &str, destination: &str) -> Self {
        TimeLine {
            mission: String::from(mission.trim()),
            time: String::from(time.trim()),
            track: String::from(track.trim()),
            destination: String::from(destination.trim()),
        }
    }
}

impl fmt::Display for TimeLine {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "{} | {} | {} | {}",
            self.mission, self.time, self.track, self.destination
        )
    }
}

pub fn display_time_lines<'a, I>(time_lines: I)
where
    I: Iterator<Item = &'a TimeLine>,
{
    for time_line in time_lines {
        println!("{}", time_line);
    }
}
