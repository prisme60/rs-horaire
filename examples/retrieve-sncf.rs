#[macro_use]
extern crate error_chain;
extern crate horaire;

use horaire::timelines::display_time_lines;
use horaire::source::sncf::sncf;
use horaire::errors::*;

fn run() -> Result<()> {
    display_time_lines(sncf("PSL", true)?.iter());
    Ok(())
}

quick_main!(run);
