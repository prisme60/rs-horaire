#[macro_use]
extern crate error_chain;
extern crate horaire;

use horaire::timelines::{display_time_lines, first_time_line_for_destination};
use horaire::source::transilien::transilien;
use horaire::errors::*;

fn run() -> Result<()> {
    let time_lines = transilien("GARE DE PARIS SAINT-LAZARE")?;
    display_time_lines(time_lines.iter());
    if !time_lines.is_empty() {
        let seconds = time_lines[0].get_remaining_seconds();
        let minutes = seconds / 60;
        println!("Next train in {} minutes and {} seconds", seconds / 60, seconds - minutes * 60);
    }
    match first_time_line_for_destination(time_lines.iter(), "Pontoise") {
        Some(time_line_pontoise) => {
            let seconds = time_line_pontoise.get_remaining_seconds();
            let minutes = seconds / 60;
            println!("Pontoise's train in {} minutes and {} seconds", seconds / 60, seconds - minutes * 60);
        },
        None => println!("No train to Pontoise for the moment.")
    }
    Ok(())
}

quick_main!(run);
