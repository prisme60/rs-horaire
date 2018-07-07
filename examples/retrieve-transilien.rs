#[macro_use]
extern crate error_chain;
extern crate horaire;

use horaire::timelines::display_time_lines;
use horaire::source::transilien::transilien;
use horaire::errors::*;

fn run() -> Result<()> {
    let time_lines = transilien("PSL")?;
    display_time_lines(time_lines.iter());
    if !time_lines.is_empty() {
        let seconds = time_lines[0].get_remaining_seconds();
        let minutes = seconds / 60;
        println!("Next train in {} minutes and {} seconds", seconds / 60, seconds - minutes * 60);
    }
    Ok(())
}

quick_main!(run);
