use crate::card::Card;
use crate::layers;
use svg::Document;

/// Generate a complete SVG document for a single card
pub fn generate_card(card: &Card) -> Document {
    let mut doc = Document::new()
        .set("viewBox", (0, 0, 1000, 1400))
        .set("width", "1000")
        .set("height", "1400")
        .set("xmlns", "http://www.w3.org/2000/svg");

    // Layer 1: Mucha-style Art Nouveau frame (20%)
    let frame = layers::frame::generate(card);
    doc = doc.add(frame);

    // Layer 2: Rank narrative core (40%)
    let core = layers::core::generate(card);
    doc = doc.add(core);

    // Layer 3: Suit texture overlay (20%)
    let suit_layer = layers::suit::generate(card);
    doc = doc.add(suit_layer);

    // Layer 4: Authority layer for face cards (10%)
    if card.is_face() {
        let authority = layers::authority::generate(card);
        doc = doc.add(authority);
    }

    // Layer 5: Klimt ornament density (10%)
    let ornament = layers::ornament::generate(card);
    doc = doc.add(ornament);

    doc
}
