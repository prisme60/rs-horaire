extern crate horaire;

use horaire::timelines::TimeLine;
use horaire::timelines::display_time_lines;
use horaire::source::transilien::transilien;
use horaire::source::ratp::ratp;
use horaire::source::sncf::sncf;

fn main() {
    display_time_lines(transilien("PSL").unwrap_or(Vec::<TimeLine>::new()).iter());
    display_time_lines(ratp("A", "Auber").unwrap_or(Vec::<TimeLine>::new()).iter());
    display_time_lines(sncf("PSL", true).unwrap_or(Vec::<TimeLine>::new()).iter());
}
