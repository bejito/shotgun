use error_chain::error_chain;
use serde_json::Value;
use std::io::Read;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() -> Result<()> {
    // The MAX JEUNE URL with query string Paris => Marseille.
    // It is a public endpoint so no token needed.
    let url = "https://data.sncf.com/api/records/1.0/search/?dataset=tgvmax&q=&facet=date&facet=origine&facet=od_happy_card&facet=destination&refine.origine=PARIS+(intramuros)&refine.destination=MARSEILLE+ST+CHARLES";
    let mut res = reqwest::blocking::get(url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    process(body);

    Ok(())
}

/// Processes the response body.
fn process(s: String) -> serde_json::Result<()> {
    let v: Value = serde_json::from_str(s.as_str())?;
    println!("Number of MAX JEUNE from PARIS (intramuros) to MARSEILLE SAINT CHARLES in the next 30 days: {}.", v["nhits"]);
    Ok(())
}
