use std::fmt;
use std::str::FromStr;
use chrono::prelude::*;

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

impl TimeLine {
    pub fn to_html_table_line(&self) -> String {
        format!("<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",self.mission, self.time, self.track, self.destination)
    }

    pub fn get_time(&self) -> DateTime<Local> {
        TimeLine::get_time_static(self.time.as_str())
    }

    pub fn get_time_static(time : &str) -> DateTime<Local> {
        let mut hms = [0u32; 3];
        let mut index = 0;
        for val in time.split(':') {
            hms[index] = u32::from_str(val).unwrap();
            index += 1;
            if index > hms.len() {
                break;
            }
        }
        Local::today().and_hms(hms[0], hms[1], hms[2])
    }

    pub fn get_remaining_seconds(&self) -> i64 {
        (self.get_time() - Local::now()).num_seconds()
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
