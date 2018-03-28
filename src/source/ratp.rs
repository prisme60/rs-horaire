extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::Class;
use timelines::TimeLine;
use errors::*;

pub fn ratp(rer_line: &str, train_station: &str) -> Result<Vec<TimeLine>> {
    let mut vec = Vec::<TimeLine>::new();
    let params = [
        ("networks", "rer"),
        ("line_rer", rer_line),
        ("type", "now"),
        ("op", "Rechercher"),
        ("stop_point_rer", train_station),
    ];
    let client = reqwest::Client::new();
    let resp = client
        .get("https://www.ratp.fr/horaires")
        .query(&params)
        .send()?;

    assert!(resp.status().is_success());

    let document = Document::from_read(resp)?;

    // finding all instances of our class of interest
    for node in document.find(Class("body-rer")) {
        let mission = node.find(Class("js-horaire-show-mission")).next().unwrap();
        let heure = node.find(Class("heure-wrap")).next().unwrap();
        let destination = node.find(Class("terminus-wrap")).next().unwrap();
        let voie = "";

        vec.push(TimeLine::new(
            &mission.text(),
            &heure.text(),
            &destination.text(),
            &voie,
        ));
    }
    Ok(vec)
}
