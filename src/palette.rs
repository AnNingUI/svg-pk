use crate::card::{Card, Suit};

#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub primary: &'static str,
    pub secondary: &'static str,
    pub highlight: &'static str,
    pub background: &'static str,
}

pub fn palette_for(card: &Card) -> Palette {
    match card {
        Card::Joker(1) => Palette {
            primary: "#3A0A0A",
            secondary: "#C41E3A",
            highlight: "#FFB6C1",
            background: "#1A0808",
        },
        Card::Joker(_) => Palette {
            primary: "#2A2A2A",
            secondary: "#8C8C8C",
            highlight: "#C8C8C8",
            background: "#151515",
        },
        Card::Standard(suit, _) => match suit {
            Suit::Spade => Palette {
                primary: "#2B2B2B",
                secondary: "#D4C5A9",
                highlight: "#F5F0E1",
                background: "#1A1A1A",
            },
            Suit::Heart => Palette {
                primary: "#8B0000",
                secondary: "#C41E3A",
                highlight: "#FFB6C1",
                background: "#1A0A0A",
            },
            Suit::Club => Palette {
                primary: "#1B4332",
                secondary: "#40916C",
                highlight: "#D4AF37",
                background: "#0A1A0A",
            },
            Suit::Diamond => Palette {
                primary: "#1B1B4B",
                secondary: "#4169E1",
                highlight: "#C0C0C0",
                background: "#0A0A1A",
            },
        },
    }
}

/// Joker uses a multi-color gradient; return all four suit palettes
pub fn joker_palettes() -> [Palette; 4] {
    [
        palette_for(&Card::Standard(Suit::Spade, crate::card::Rank::Ace)),
        palette_for(&Card::Standard(Suit::Heart, crate::card::Rank::Ace)),
        palette_for(&Card::Standard(Suit::Club, crate::card::Rank::Ace)),
        palette_for(&Card::Standard(Suit::Diamond, crate::card::Rank::Ace)),
    ]
}
