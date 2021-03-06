use error_chain::quick_main;
use horaire::errors::*;
use horaire::source::transilien::transilien;
use horaire::timelines::{display_time_lines, first_time_line_for_destination};

fn run() -> Result<()> {
    let time_lines = transilien("GARE DE PARIS SAINT-LAZARE", 8738400)?;
    let destination = "PONTOISE";
    display_time_lines(time_lines.iter());
    if !time_lines.is_empty() {
        let seconds = time_lines[0].get_remaining_seconds();
        let minutes = seconds / 60;
        println!("Next train in {} minutes and {} seconds", seconds / 60, seconds - minutes * 60);
    }
    match first_time_line_for_destination(time_lines.iter(), destination) {
        Some(time_line_pontoise) => {
            let seconds = time_line_pontoise.get_remaining_seconds();
            let minutes = seconds / 60;
            println!("{}'s train in {} minutes and {} seconds", destination, seconds / 60, seconds - minutes * 60);
        }
        None => println!("No train to {} for the moment.", destination),
    }
    Ok(())
}

quick_main!(run);
