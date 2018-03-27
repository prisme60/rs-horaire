#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate select;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::fmt;
use select::document::Document;
use select::predicate::{Class};

error_chain! {
    foreign_links {
        Req(reqwest::Error);
        Io(std::io::Error);
    }
}

struct TimeLine {
    mission: String,
    time: String,
    track: String,
    destination: String
}

impl TimeLine {
    fn new(mission: &str, time: &str, track: &str, destination: &str) -> Self {
        TimeLine {
            mission: String::from(mission.trim()),
            time: String::from(time.trim()),
            track: String::from(track.trim()),
            destination: String::from(destination.trim())}
    }
}

impl fmt::Display for TimeLine {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{} | {} | {} | {}", self.mission, self.time, self.track, self.destination)
    }
}

fn main() {
    display_time_lines(transilien("PSL").unwrap_or( Vec::<TimeLine>::new()).iter());
    display_time_lines(ratp("A", "Auber").unwrap_or(Vec::<TimeLine>::new()).iter());
    display_time_lines(sncf("PSL", true).unwrap_or(Vec::<TimeLine>::new()).iter());
}

fn display_time_lines<'a, I>(time_lines: I)
where
    I: Iterator<Item = &'a TimeLine>,
{
    for time_line in time_lines {
        println!("{}",time_line);
    }
}

fn transilien(train_station: &str) -> Result<Vec<TimeLine>> {
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

fn ratp(rer_line: &str, train_station: &str) -> Result<Vec<TimeLine>>  {
    let mut vec = Vec::<TimeLine>::new();
    let params = [("networks","rer"),("line_rer",rer_line),("type","now"),
         ("op","Rechercher"),("stop_point_rer",train_station)];
    let client = reqwest::Client::new();
    let resp = client.get("https://www.ratp.fr/horaires")
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

        vec.push(TimeLine::new(&mission.text(), &heure.text(), &destination.text(), &voie));
    }
    Ok(vec)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonSncf {
    trains : Vec<TimeLineSncf>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeLineSncf {
    origdest: String, //	"LE HAVRE"
    num: String, //	"3132"
    //type: String, //	"INTERCITES"
    picto: String, //	"/sites/default/files/styles/picto_train_board/public/field_taxo_trans_picto_horaires/2015-03/30_sncf_nb.png"
    voie: String, //	"22"
    voie_attr: String, //	""
    heure: String, //	"21:41"
    etat: String, //	""
    retard: String, //	""
    infos: String, //	""
}

fn sncf(train_station: &str, departure:bool) -> Result<Vec<TimeLine>> {
    let mut vec = Vec::<TimeLine>::new();
    // http://www.gares-sncf.com/fr/train-times/PSL/departure
    // http://www.gares-sncf.com/fr/train-times/PSL/arrival
    let sens = if departure { "/departure" } else { "/arrival" };
    let mut url:String = String::from("http://www.gares-sncf.com/fr/train-times/");
    url.push_str(&train_station);
    url.push_str(&sens);
    let mut resp = reqwest::get(&url)?;

    assert!(resp.status().is_success());

    let json : JsonSncf = resp.json()?;

    println!("{:?}", json);

    // finding all instances of our class of interest
    for train in json.trains {
        let mission = train.num;
        let heure = train.heure;
        let destination = train.origdest;
        let voie = train.voie;

        vec.push(TimeLine::new(&mission, &heure, &destination, &voie));
    }
    Ok(vec)
}