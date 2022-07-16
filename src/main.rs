use clap::Parser;

mod sncf;

/// Shotgun SNCF trains. First arrive first served.
#[derive(Parser, Debug)]
#[clap(author="Louis Vignoli", version, about, long_about = None)]
struct Args {
    /// Origin station
    #[clap(value_parser, default_value = "marseille")]
    origin: sncf::Gare,
    /// Destination station
    #[clap(value_parser, default_value = "paris")]
    destination: sncf::Gare,
}

fn main() {
    let args = Args::parse();

    println!("Going from {} to {}.", args.origin, args.destination);

    let origin = args.origin;
    let destination = args.destination;

    let (total, available) = sncf::get_available_tgvmax_ratio(origin, destination);
    println!("For the journey {} => {},", origin, destination);
    println!(
        "there are {} out of {} MAX JEUNE trains available in the next 30 days.",
        available, total
    );

    if available > 0 {
        println!("!! QUICK !!");
        println!("HURRY UP TO BOOK!");

        let url = &sncf::construct_tgvmax_query_url(origin, destination, true);
        let travels = sncf::get_travels(url).unwrap();
        for t in travels {
            println!("{}", t);
        }
    } else {
        println!("No train for you today.")
    }

    println!("Be advised that this ***** API from the SNCF miss a lot of bookable trains, so don't rely on it and construct something more robust on top of their private API or god knows what.")
}
