use std::fmt;
use std::str::FromStr;
use chrono::prelude::*;
use std::time;


pub struct TimeLine {
    mission: String,
    time: String,
    destination: String,
    track: String,
}

impl TimeLine {
    pub fn new(mission: &str, time: &str, destination: &str, track: &str) -> Self {
        TimeLine {
            mission: String::from(mission.trim()),
            time: String::from(time.trim()),
            destination: String::from(destination.trim()),
            track: String::from(track.trim())
        }
    }

    pub fn get_mission_string(&self) -> &String     { &self.mission }
    pub fn get_time_string(&self) -> &String        { &self.time }
    pub fn get_destination_string(&self) -> &String { &self.destination }
    pub fn get_track_string(&self) -> &String { &self.track }

    pub fn to_html_table_line(&self) -> String {
        format!("<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",self.mission, self.time, self.track, self.destination)
    }

    pub fn get_time(&self) -> Result<DateTime<Local>,&str> {
        TimeLine::get_time_static(self.time.as_str())
    }

    pub fn get_time_static(time : &str) -> Result<DateTime<Local>, &str> {
        let mut hms = [0u32; 3];
        let mut index = 0;
        for val in time.split(':') {
            match u32::from_str(val) {
                Ok(val) => {hms[index] = val;},
                Err(_) => {return Err("No given time");}
            };
            index += 1;
            if index > hms.len() {
                break;
            }
        }
        Ok(Local::today().and_hms(hms[0], hms[1], hms[2]))
    }

    pub fn get_remaining_seconds(&self) -> i64 {
        self.get_seconds_difference_from_reference(&Local::now())
    }

    pub fn get_seconds_difference_from_reference(&self, reference_date_time : &DateTime<Local>) -> i64 {
        match self.get_time() {
            Ok(time_val) => (time_val - *reference_date_time).num_seconds(),
            Err(_) => 24*60*60 // 1 day
        }
    }

    pub fn get_difference_from_reference(&self, reference_date_time : &DateTime<Local>) -> time::Duration {
        match self.get_time() {
            Ok(time_val) => (time_val - *reference_date_time).to_std().unwrap(),
            Err(_) => time::Duration::new(24*60*60, 0)
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
            self.mission, self.time, self.destination, self.track
        )
    }
}

pub fn get_time_lines_html<'a, I>(time_lines: I) -> String
    where
        I: Iterator<Item = &'a TimeLine>,
{
    let mut strings = time_lines.fold(String::from("<html><header/><body><table>"), |acc, ref mut time_line| {
        acc + &format!("{}", time_line.to_html_table_line())
    });
    strings.pop();
    strings.push_str("</table></body></html>");
    strings
    // time_lines.map(|time_line| format!("{}", time_line)).collect::<Vec<_>>().join("<p>\n")
}

pub fn display_time_lines<'a, I>(time_lines: I)
where
    I: Iterator<Item = &'a TimeLine>,
{
    for time_line in time_lines {
        println!("{}", time_line);
    }
}

pub fn first_time_line_for_destination<'a, I>(time_lines: I, destination : &str) -> Option<&'a TimeLine>
    where
        I: Iterator<Item = &'a TimeLine>,
{
    for time_line in time_lines {
        if time_line.destination == destination {
            return Some(&time_line);
        }
    }
    None
}