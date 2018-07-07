extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::Class;
use timelines::TimeLine;
use errors::*;

pub fn transilien(train_station: &str) -> Result<Vec<TimeLine>> {
    let mut vec = Vec::<TimeLine>::new();
    let params = [("idOrigin", train_station)];
    let client = reqwest::Client::new();
    let resp = client
        .get("https://transilien.mobi/train/result")
        .query(&params)
        .send()?;

    //assert!(resp.status().is_success());
    if resp.status().is_success() {
        let document = Document::from_read(resp).unwrap();

        // finding all instances of our class of interest
        for node in document.find(Class("resultat_gare")) {
            let mission = node.find(Class("train_mission"))
                .next()
                .ok_or(ErrorKind::MissingField("mission".to_string()))?;
            let heure = node.find(Class("heure_train"))
                .next()
                .ok_or(ErrorKind::MissingField("heure".to_string()))?;
            let destination = node.find(Class("garearrivee"))
                .next()
                .ok_or(ErrorKind::MissingField("destination".to_string()))?;
            let voie = node.find(Class("voie"))
                .next()
                .ok_or(ErrorKind::MissingField("voie".to_string()))?;

            // Remove "Dir :" in the destination
            let destination_with_dir = destination.text();
            let destination_no_dir = destination_with_dir.splitn(2,"Dir : ").last().unwrap();

            vec.push(TimeLine::new(
                &mission.text(),
                &heure.text(),
                &destination_no_dir,
                &voie.text()
            ));
        }
        Ok(vec)
    } else {
        Err(ErrorKind::InvalidAnswerError.into())
    }
}
