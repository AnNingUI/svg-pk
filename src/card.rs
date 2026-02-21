use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    Spade,
    Heart,
    Club,
    Diamond,
}

impl Suit {
    pub fn all() -> [Suit; 4] {
        [Suit::Spade, Suit::Heart, Suit::Club, Suit::Diamond]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Suit::Spade => "spade",
            Suit::Heart => "heart",
            Suit::Club => "club",
            Suit::Diamond => "diamond",
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Suit::Spade => "♠",
            Suit::Heart => "♥",
            Suit::Club => "♣",
            Suit::Diamond => "♦",
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Rank {
    pub fn all() -> [Rank; 13] {
        [
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ]
    }

    /// Complexity factor 0.07..0.91 controlling ornament density
    pub fn complexity(&self) -> f64 {
        match self {
            Rank::Ace => 0.07,
            Rank::Two => 0.14,
            Rank::Three => 0.21,
            Rank::Four => 0.28,
            Rank::Five => 0.35,
            Rank::Six => 0.42,
            Rank::Seven => 0.49,
            Rank::Eight => 0.56,
            Rank::Nine => 0.63,
            Rank::Ten => 0.70,
            Rank::Jack => 0.77,
            Rank::Queen => 0.84,
            Rank::King => 0.91,
        }
    }

    pub fn is_face(&self) -> bool {
        matches!(self, Rank::Jack | Rank::Queen | Rank::King)
    }

    pub fn name(&self) -> &'static str {
        match self {
            Rank::Ace => "ace",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "jack",
            Rank::Queen => "queen",
            Rank::King => "king",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Rank::Ace => "A",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
        }
    }

    pub fn number(&self) -> u8 {
        match self {
            Rank::Ace => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Card {
    Standard(Suit, Rank),
    Joker(u8), // 1 = red joker, 2 = black joker
}

impl Card {
    pub fn filename(&self) -> String {
        match self {
            Card::Standard(suit, rank) => format!("{}_{}.svg", suit.name(), rank.name()),
            Card::Joker(n) => format!("joker_{}.svg", n),
        }
    }

    pub fn suit(&self) -> Option<Suit> {
        match self {
            Card::Standard(s, _) => Some(*s),
            Card::Joker(_) => None,
        }
    }

    pub fn rank(&self) -> Option<Rank> {
        match self {
            Card::Standard(_, r) => Some(*r),
            Card::Joker(_) => None,
        }
    }

    pub fn is_face(&self) -> bool {
        self.rank().is_some_and(|r| r.is_face())
    }

    pub fn is_joker(&self) -> bool {
        matches!(self, Card::Joker(_))
    }

    pub fn complexity(&self) -> f64 {
        self.rank().map_or(0.5, |r| r.complexity())
    }

    /// Seed for deterministic randomness per card
    pub fn seed(&self) -> u64 {
        match self {
            Card::Standard(s, r) => (*s as u64) * 100 + r.number() as u64,
            Card::Joker(n) => 500 + *n as u64,
        }
    }

    /// Generate all 54 cards
    pub fn all_cards() -> Vec<Card> {
        let mut cards = Vec::with_capacity(54);
        for suit in Suit::all() {
            for rank in Rank::all() {
                cards.push(Card::Standard(suit, rank));
            }
        }
        cards.push(Card::Joker(1));
        cards.push(Card::Joker(2));
        cards
    }
}
