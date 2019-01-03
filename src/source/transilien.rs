extern crate reqwest;
extern crate select;

use crate::errors::*;
use crate::timelines::TimeLine;
use select::document::Document;
use select::predicate::Class;

pub fn transilien(train_station: &str) -> Result<Vec<TimeLine>> {
    let params = [("departure", train_station)];
    transilien_params(&params)
}

pub fn transilien_uic(train_station_uic: u32) -> Result<Vec<TimeLine>> {
    let train_station_uic_str = train_station_uic.to_string();
    let params = [("departure", train_station_uic_str.as_str())];
    transilien_params(&params)
}

pub fn transilien_params(params: &[(&str, &str)]) -> Result<Vec<TimeLine>> {
    let mut vec = Vec::<TimeLine>::new();
    let client = reqwest::Client::new();
    let resp = client
        .get("https://www.transilien.com/fr/horaires/prochains-departs")
        .query(&params)
        .send()?;

    //assert!(resp.status().is_success());
    if resp.status().is_success() {
        let document = Document::from_read(resp).unwrap();

        // finding all instances of our class of interest
        for node in document.find(Class("result-main-line")) {
            let mission = node
                .find(Class("code"))
                .next()
                .ok_or_else(|| ErrorKind::MissingField("mission".to_string()))?;
            let heure = node
                .find(Class("hour"))
                .next()
                .ok_or_else(|| ErrorKind::MissingField("heure".to_string()))?;
            let destination = node
                .find(Class("destination-col"))
                .next()
                .ok_or_else(|| ErrorKind::MissingField("destination".to_string()))?;
            let voie_node = node
                .find(Class("pathway"))
                .next()
                .ok_or_else(|| ErrorKind::MissingField("voie".to_string()))?;
            let voie = match voie_node.find(Class("hidden-xs")).next() {
                Some(v) => v.text(),
                None => "".to_owned(),
            };
            // Remove "Dir :" in the destination
            let destination_with_dir = destination.text();
            let destination_no_dir = destination_with_dir
                .splitn(2, "Destination")
                .last()
                .unwrap();

            vec.push(TimeLine::new(
                &mission.text(),
                &heure.text(),
                &destination_no_dir,
                &voie,
            ));
        }
        Ok(vec)
    } else {
        Err(ErrorKind::InvalidAnswerError.into())
    }
}
