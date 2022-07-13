use anyhow::{Context, Result};
use reqwest::Url;
use serde_json::Value;

const BASE_SNCF_API: &str = "https://data.sncf.com/api/records/1.0/search/";
const _PARIS_MARSEILLE_HAPPY_CARD_EXAMPLE_URL: &str = "https://data.sncf.com/api/records/1.0/search/?dataset=tgvmax&q=&facet=date&facet=origine&facet=destination&facet=od_happy_card&refine.origine=PARIS+(intramuros)&refine.destination=MARSEILLE+ST+CHARLES&refine.od_happy_card=OUI";

#[derive(Debug)]
enum Gare {
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

fn main() {
    let url_paris_marseille = construct_tgvmax_query_url(Gare::Paris, Gare::Marseille, false);
    let url_paris_marseille_happy_card =
        construct_tgvmax_query_url(Gare::Paris, Gare::Marseille, true);

    if let Ok(n_max_jeune) = get_number_from_url(&url_paris_marseille) {
        println!("Number of MAX JEUNE: {}.", n_max_jeune)
    }

    if let Ok(n_max_jeune_available) = get_number_from_url(&url_paris_marseille_happy_card) {
        println!("Number of available MAX JEUNE: {}.", n_max_jeune_available)
    }
}

fn get_number_from_url(url: &str) -> Result<u64> {
    let body = reqwest::blocking::get(url)?.text()?;
    let v: Value = serde_json::from_str(&body)?;
    let n = v["nhits"].as_u64().context("cannot convert to u64")?;
    Ok(n)
}

/// Construct a tgvmax query url using the provided details.
fn construct_tgvmax_query_url(origin: Gare, destination: Gare, happy_card: bool) -> String {
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
