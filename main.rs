use error_chain::error_chain;
use serde_json::Value;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        JSON(serde_json::Error);
    }
}
const PARIS_MARSEILLE_URL: &str = "https://data.sncf.com/api/records/1.0/search/?dataset=tgvmax&q=&facet=date&facet=origine&facet=od_happy_card&facet=destination&refine.origine=PARIS+(intramuros)&refine.destination=MARSEILLE+ST+CHARLES";
const PARIS_MARSEILLE_HAPPY_CARD_URL: &str = "https://data.sncf.com/api/records/1.0/search/?dataset=tgvmax&q=&facet=date&facet=origine&facet=destination&facet=od_happy_card&refine.origine=PARIS+(intramuros)&refine.destination=MARSEILLE+ST+CHARLES&refine.od_happy_card=OUI";

fn main() {
    // The MAX JEUNE URL with query strings Paris => Marseille.
    // It is a public endpoint so no token needed.

    if let Ok(n_max_jeune) = get_number_from_url(PARIS_MARSEILLE_URL) {
        println!("Number of MAX JEUNE: {}.", n_max_jeune)
    }

    if let Ok(n_max_jeune_available) = get_number_from_url(PARIS_MARSEILLE_HAPPY_CARD_URL) {
        println!("Number of available MAX JEUNE: {}.", n_max_jeune_available)
    }
}

fn get_number_from_url(url: &str) -> Result<u64> {
    let body = reqwest::blocking::get(url)?.text()?;
    let v: Value = serde_json::from_str(&body)?;
    Ok(v["nhits"].as_u64().ok_or("cannot convert")?)
}
