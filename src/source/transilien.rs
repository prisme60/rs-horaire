use crate::{errors::*, timelines::TimeLine};
use reqwest::{self, header};
use std::{time::Duration};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonRequestTransilien<'a> {
    departure: &'a str,
    uic_departure : &'a str,
    uic_arrival : &'a str,
    pmr: bool
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsonResponseTransilien {
    //platformAvailable: bool,
    //disruptionsAvailable: bool,
    next_trains_list : Vec<NextTrain>,
    //departureStopArea
    //arrivalStopArea
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct NextTrain {
    mode_transport_enum: String, // RER
    line_transport_enum: String, // RER_C
    code_mission: String, // MONA
    cancelled:bool,
    delayed: bool,
    departure_time:String, //	16:28
    destination_mission:String, // MASSY PALAISEAU
    platform: String, //	16
    //deservedStations	[…]
    //hasTraficDisruption	false
    //hasTravauxDisruption	true
    //disruptions	[…]
}

pub fn transilien(train_station: &str, train_station_uic: u32) -> Result<Vec<TimeLine>> {
    let uic = train_station_uic.to_string();
    let json_transilien_req = JsonRequestTransilien {departure: train_station, uic_departure: uic.as_str(), uic_arrival: "", pmr: false};
    transilien_params(&json_transilien_req)
}

fn transilien_params(params: &JsonRequestTransilien) -> Result<Vec<TimeLine>> {
    let mut vec = Vec::<TimeLine>::new();

    // HTTP/1.1 200 OK
    // Server: KCDN
    // Date: Tue, 05 Feb 2019 10:06:51 GMT
    // Content-Type: application/json;charset=UTF-8
    // Content-Length: 2639
    // Cache-Control: no-cache, no-store, max-age=0, must-revalidate
    // Pragma: no-cache
    // Expires: 0
    // X-XSS-Protection: 1; mode=block
    // X-Frame-Options: DENY
    // X-Content-Type-Options: nosniff
    // Vary: Accept-Encoding
    // Content-Encoding: gzip
    // Age: 0
    // Accept-Ranges: bytes
    // X-Cdn-Srv: ovh-rbx-cache-1
    let mut headers = header::HeaderMap::new();
    headers.insert(header::HOST, header::HeaderValue::from_static("www.transilien.com"));
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:67.0) Gecko/20100101 Firefox/67.0"));
    headers.insert(header::ACCEPT, header::HeaderValue::from_static("application/json, text/plain, */*"));
    headers.insert(header::ACCEPT_LANGUAGE, header::HeaderValue::from_static("fr"));
    headers.insert(header::ACCEPT_ENCODING, header::HeaderValue::from_static("gzip, deflate, br"));
    //headers.insert(header::SERVER, header::HeaderValue::from_static("KCDN"));
    headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json;charset=utf-8"));
    headers.insert(header::CONNECTION, header::HeaderValue::from_static("keep-alive"));
    headers.insert(header::CACHE_CONTROL, header::HeaderValue::from_static("no-cache"));
    headers.insert(header::PRAGMA, header::HeaderValue::from_static("no-cache"));
    //headers.insert(header::EXPIRES, header::HeaderValue::from_static("0"));
    //headers.insert(header::X_XSS_PROTECTION, header::HeaderValue::from_static("1; mode=block"));
    //headers.insert(header::X_FRAME_OPTIONS, header::HeaderValue::from_static("DENY"));
    //headers.insert(header::X_CONTENT_TYPE_OPTIONS, header::HeaderValue::from_static("nosniff"));
    //headers.insert(header::VARY, header::HeaderValue::from_static("Accept-Encoding"));
    //headers.insert(header::AGE, header::HeaderValue::from_static("0"));
    //headers.insert(header::ACCEPT_RANGES, header::HeaderValue::from_static("bytes"));

    let client = reqwest::Client::builder().gzip(true).timeout(Duration::from_secs(10)).build()?;

    // https://www.transilien.com/api/nextDeparture/search
    // {"departure":"PONTOISE","uicDeparture":"8727613","pmr":false}
    //let json_transilien_req = JsonRequestTransilien {departure: "PONTOISE", /*uic_departure: "8727613",*/ pmr: false};
    //println!("json : {}", params);

    let mut resp = client
        .post("https://www.transilien.com/api/nextDeparture/search")
        .headers(headers)
        .json(&params)
        .send()?;

    //println!("Response :\n{:?}", resp);
    //assert!(resp.status().is_success());
    if resp.status().is_success() {
        //println!("Text :\n{:?}", resp.text()?);
        let json_resp : JsonResponseTransilien = resp.json()?;

        // finding all instances of our class of interest
        for next_train in json_resp.next_trains_list {
            vec.push(TimeLine::new(
                &next_train.code_mission,
                &next_train.departure_time,
                &next_train.destination_mission,
                &next_train.platform,
            ));
        }
        Ok(vec)
    } else {
        Err(ErrorKind::InvalidAnswerError.into())
    }
}
