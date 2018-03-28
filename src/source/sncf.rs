extern crate reqwest;

use timelines::TimeLine;
use errors::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonSncf {
    trains: Vec<TimeLineSncf>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeLineSncf {
    origdest: String,                            //	"LE HAVRE"
    num: String,                                 //	"3132"
    #[serde(rename = "type")] type_name: String, //	"INTERCITES"
    picto: String, //	"/sites/default/files/styles/picto_train_board/public/field_taxo_trans_picto_horaires/2015-03/30_sncf_nb.png"
    voie: String,  //	"22"
    voie_attr: String, //	""
    heure: String, //	"21:41"
    etat: String,  //	""
    retard: String, //	""
    infos: String, //	""
}

pub fn sncf(train_station: &str, departure: bool) -> Result<Vec<TimeLine>> {
    let mut vec = Vec::<TimeLine>::new();
    // http://www.gares-sncf.com/fr/train-times/PSL/departure
    // http://www.gares-sncf.com/fr/train-times/PSL/arrival
    let sens = if departure { "/departure" } else { "/arrival" };
    let mut url: String = String::from("http://www.gares-sncf.com/fr/train-times/");
    url.push_str(&train_station);
    url.push_str(&sens);
    let mut resp = reqwest::get(&url)?;

    //assert!(resp.status().is_success());
    if resp.status().is_success() {
        let json: JsonSncf = resp.json()?;

        //println!("{:?}", json);

        // finding all instances of our class of interest
        for train in json.trains {
            let mission = train.num;
            let heure = train.heure;
            let destination = train.origdest;
            let voie = train.voie;

            vec.push(TimeLine::new(&mission, &heure, &destination, &voie));
        }
        Ok(vec)
    } else {
        Err(ErrorKind::InvalidAnswerError.into())
    }
}
