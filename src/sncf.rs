use anyhow::{Context, Result};
use reqwest::Url;
use serde_derive::Deserialize;
use serde_json::Value;

const BASE_SNCF_API: &str = "https://data.sncf.com/api/records/1.0/search/";
const _PARIS_MARSEILLE_HAPPY_CARD_EXAMPLE_URL: &str = "https://data.sncf.com/api/records/1.0/search/?dataset=tgvmax&q=&facet=date&facet=origine&facet=destination&facet=od_happy_card&refine.origine=PARIS+(intramuros)&refine.destination=MARSEILLE+ST+CHARLES&refine.od_happy_card=OUI";

/// Gare is an train station.
/// Just giving a nice french vibe here. Next step: strikes.
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum Gare {
    Paris,
    Marseille,
    LaRochelle,
    Lyon,
}

impl Gare {
    fn value(&self) -> &str {
        match *self {
            Gare::Paris => "PARIS (intramuros)",
            Gare::Marseille => "MARSEILLE ST CHARLES",
            Gare::LaRochelle => "LA ROCHELLE VILLE",
            Gare::Lyon => "LYON (intramuros)",
        }
    }
}

impl std::fmt::Display for Gare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

pub struct Travel {
    origin: String,
    destination: String,
    date: String,
    departure_time: String,
    arrival_time: String,
    train: String,
}

impl std::fmt::Display for Travel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} {} {} => {} {}",
            self.date,
            self.train,
            self.origin,
            self.departure_time,
            self.arrival_time,
            self.destination
        )
    }
}

#[derive(Deserialize)]
struct Response {
    records: Vec<Record>,
}

#[derive(Deserialize)]
struct Record {
    fields: Fields,
}

#[derive(Deserialize)]
struct Fields {
    date: String,
    origine: String,
    destination: String,
    #[serde(rename = "heure_depart")]
    departure_time: String,
    #[serde(rename = "heure_arrivee")]
    arrival_time: String,
    #[serde(rename = "train_no")]
    train: String,
}

pub fn get_travels(url: &str) -> Result<Vec<Travel>> {
    let body = reqwest::blocking::get(url)?.text()?;
    let resp: Response = serde_json::from_str(&body)?;

    let mut travels: Vec<Travel> = vec![];
    for r in resp.records {
        let f = r.fields;
        travels.push(Travel {
            origin: f.origine,
            destination: f.destination,
            date: f.date,
            departure_time: f.departure_time,
            arrival_time: f.arrival_time,
            train: f.train,
        });
    }
    travels.sort_by(|t1, t2| t1.date.cmp(&t2.date));
    Ok(travels)
}

fn get_number_from_url(url: &str) -> Result<u64> {
    let body = reqwest::blocking::get(url)?.text()?;
    let v: Value = serde_json::from_str(&body)?;
    let n = v["nhits"].as_u64().context("cannot convert to u64")?;
    Ok(n)
}

/// Construct a tgvmax query url using the provided details.
pub fn construct_tgvmax_query_url(origin: Gare, destination: Gare, happy_card: bool) -> String {
    let mut params = vec![
        ("dataset", "tgvmax"),
        ("facet", "date"),
        ("facet", "origin"),
        ("facet", "destination"),
        ("refine.origine", origin.value()),
        ("refine.destination", destination.value()),
    ];
    if happy_card {
        params.push(("refine.od_happy_card", "OUI"));
    }

    let url = Url::parse_with_params(BASE_SNCF_API, params).unwrap();
    url.to_string()
}

pub fn get_available_tgvmax_ratio(origin: Gare, destination: Gare) -> (u64, u64) {
    let url_total = construct_tgvmax_query_url(origin, destination, false);
    let url_available = construct_tgvmax_query_url(origin, destination, true);

    let total = get_number_from_url(&url_total).expect("http error");
    let available = get_number_from_url(&url_available).expect("http error");

    (total, available)
}
