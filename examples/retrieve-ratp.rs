use error_chain::quick_main;
use horaire::errors::*;
use horaire::source::ratp::ratp;
use horaire::timelines::display_time_lines;

fn run() -> Result<()> {
    display_time_lines(ratp("A", "Auber")?.iter());
    Ok(())
}

quick_main!(run);
