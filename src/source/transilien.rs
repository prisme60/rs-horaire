extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::{Class};
use timelines::TimeLine;
use errors::*;

pub fn transilien(train_station: &str) -> Result<Vec<TimeLine>> {
    let mut vec = Vec::<TimeLine>::new();
    let params = [("idOrigin", train_station)];
    let client = reqwest::Client::new();
    let resp = client.get("https://transilien.mobi/train/result")
        .query(&params)
        .send()?;

    assert!(resp.status().is_success());

    let document = Document::from_read(resp).unwrap();

    // finding all instances of our class of interest
    for node in document.find(Class("resultat_gare")) {
        let mission = node.find(Class("train_mission")).next().unwrap();
        let heure = node.find(Class("heure_train")).next().unwrap();
        let destination = node.find(Class("garearrivee")).next().unwrap();
        let voie = node.find(Class("voie")).next().unwrap();

        vec.push(TimeLine::new(&mission.text(),&heure.text(), &destination.text(), &voie.text()));
    }
    Ok(vec)
}
