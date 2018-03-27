#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate select;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

pub mod timelines;
pub mod source;
pub mod errors;

use timelines::TimeLine;
use timelines::display_time_lines;
use source::transilien::transilien;
use source::ratp::ratp;
use source::sncf::sncf;

//pub use errors::*;


fn main() {
    display_time_lines(transilien("PSL").unwrap_or( Vec::<TimeLine>::new()).iter());
    display_time_lines(ratp("A", "Auber").unwrap_or(Vec::<TimeLine>::new()).iter());
    display_time_lines(sncf("PSL", true).unwrap_or(Vec::<TimeLine>::new()).iter());
}

