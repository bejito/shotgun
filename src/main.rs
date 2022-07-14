mod sncf;

fn main() {
    let origin = sncf::Gare::Marseille;
    let destination = sncf::Gare::Paris;

    let (total, available) = sncf::get_available_tgvmax_ratio(origin, destination);
    println!("For the journey {} => {},", origin, destination);
    println!(
        "there are {} out of {} MAX JEUNE trains are available in the next 30 days.",
        available, total
    );

    if available > 0 {
        println!("!! QUICK !!");
        println!("HURRY UP TO BOOK!");
    } else {
        println!("No train for you today.")
    }
}
