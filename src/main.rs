mod card;
mod generator;
mod geometry;
mod layers;
mod palette;

use std::fs;
use std::path::Path;

fn main() {
    let output_dir = "output";
    if !Path::new(output_dir).exists() {
        fs::create_dir_all(output_dir).expect("Failed to create output directory");
    }

    let cards = card::Card::all_cards();
    let total = cards.len();

    println!("Generating {} SVG poker cards...", total);

    for (i, card) in cards.iter().enumerate() {
        let doc = generator::generate_card(card);
        let filename = format!("{}/{}", output_dir, card.filename());
        svg::save(&filename, &doc).unwrap_or_else(|_| panic!("Failed to save {}", filename));
        println!("[{}/{}] {}", i + 1, total, card.filename());
    }

    println!("Done! {} cards saved to {}/", total, output_dir);
}
