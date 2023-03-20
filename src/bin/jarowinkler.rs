use jaro_winkler_distance::{jaro_winkler_distance, PrefixLength};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: jarowinkler <lhs> <rhs>");
        return;
    }
    let lhs = &args[1];
    let rhs = &args[2];
    let jaro_winkler_distance = jaro_winkler_distance(lhs, rhs, &PrefixLength::Four);
    println!("jaro winkler distance: {jaro_winkler_distance}");
}
