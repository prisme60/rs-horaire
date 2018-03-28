#[macro_use]
extern crate error_chain;
extern crate horaire;

use horaire::timelines::display_time_lines;
use horaire::source::transilien::transilien;
use horaire::errors::*;

fn run() -> Result<()> {
    display_time_lines(transilien("PSL")?.iter());
    Ok(())
}

quick_main!(run);
