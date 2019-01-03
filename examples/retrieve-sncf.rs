use error_chain::quick_main;
use horaire::errors::*;
use horaire::source::sncf::sncf;
use horaire::timelines::display_time_lines;

fn run() -> Result<()> {
    display_time_lines(sncf("PSL", true)?.iter());
    Ok(())
}

quick_main!(run);
