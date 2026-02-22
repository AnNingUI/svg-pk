use crate::card::{Card, Rank, Suit};
use crate::geometry::*;
use crate::palette::palette_for;
use svg::node::element::{Circle, Group, Path};

/// Generate rank narrative core (40% visual weight)
pub fn generate(card: &Card) -> Group {
    let pal = palette_for(card);
    let seed = card.seed() as u32 + 1000;

    if card.is_joker() {
        return generate_joker(card, seed);
    }

    let rank = card.rank().unwrap();
    let suit = card.suit().unwrap();
    let mut g = Group::new().set("id", "core");

    match rank {
        Rank::Ace => {
            g = g.add(ace_core(
                suit,
                pal.primary,
                pal.secondary,
                pal.highlight,
                seed,
            ))
        }
        Rank::Two => g = g.add(two_core(suit, pal.primary, pal.secondary, seed)),
        Rank::Three => g = g.add(three_core(suit, pal.primary, pal.secondary, seed)),
        Rank::Four => g = g.add(four_core(suit, pal.primary, pal.secondary, seed)),
        Rank::Five => g = g.add(five_core(suit, pal.primary, pal.secondary, seed)),
        Rank::Six => g = g.add(six_core(suit, pal.primary, pal.secondary, seed)),
        Rank::Seven => g = g.add(seven_core(suit, pal.primary, pal.secondary, seed)),
        Rank::Eight => g = g.add(eight_core(suit, pal.primary, pal.secondary, seed)),
        Rank::Nine => g = g.add(nine_core(suit, pal.primary, pal.secondary, seed)),
        Rank::Ten => g = g.add(ten_core(suit, pal.primary, pal.secondary, seed)),
        Rank::Jack | Rank::Queen | Rank::King => {
            g = g.add(face_core(
                rank,
                suit,
                pal.primary,
                pal.secondary,
                pal.highlight,
                seed,
            ));
        }
    }

    // Rank label top-left and bottom-right (rotated 180)
    g = g.add(rank_label(rank, suit, pal.highlight));

    g
}

fn rank_label(rank: Rank, suit: Suit, color: &str) -> Group {
    let mut g = Group::new();
    // Top-left: rank text on top, suit symbol below
    let label = svg::node::element::Text::new(rank.label())
        .set("x", 55)
        .set("y", 125)
        .set("font-size", 54)
        .set("font-family", "serif")
        .set("fill", color)
        .set("text-anchor", "middle");
    g = g.add(label);

    let sym = svg::node::element::Text::new(suit.symbol())
        .set("x", 55)
        .set("y", 185)
        .set("font-size", 48)
        .set("font-family", "serif")
        .set("fill", color)
        .set("text-anchor", "middle");
    g = g.add(sym);

    // Bottom-right (rotated 180): suit symbol closer to center, rank text closer to edge
    let sym2 = svg::node::element::Text::new(suit.symbol())
        .set("x", 945)
        .set("y", 1345)
        .set("font-size", 48)
        .set("font-family", "serif")
        .set("fill", color)
        .set("text-anchor", "middle")
        .set("transform", "rotate(180,945,1285)");
    g = g.add(sym2);

    let label2 = svg::node::element::Text::new(rank.label())
        .set("x", 945)
        .set("y", 1375)
        .set("font-size", 54)
        .set("font-family", "serif")
        .set("fill", color)
        .set("text-anchor", "middle")
        .set("transform", "rotate(180,945,1330)");
    g = g.add(label2);

    g
}

fn suit_shape(suit: Suit, cx: f64, cy: f64, size: f64, _seed: u32) -> Path {
    match suit {
        Suit::Spade => {
            let d = format!(
                "M{},{} C{},{} {},{} {},{} C{},{} {},{} {},{} L{},{} Z",
                cx,
                cy - size,
                cx - size * 0.6,
                cy - size * 0.6,
                cx - size * 0.7,
                cy + size * 0.2,
                cx - size * 0.3,
                cy + size * 0.5,
                cx - size * 0.1,
                cy + size * 0.7,
                cx + size * 0.1,
                cy + size * 0.7,
                cx + size * 0.3,
                cy + size * 0.5,
                cx,
                cy + size * 0.3
            );
            Path::new().set("d", d)
        }
        Suit::Heart => {
            let d = format!(
                "M{},{} C{},{} {},{} {},{} C{},{} {},{} {},{} Z",
                cx,
                cy + size * 0.7,
                cx - size * 0.1,
                cy + size * 0.5,
                cx - size * 0.7,
                cy + size * 0.2,
                cx - size * 0.7,
                cy - size * 0.1,
                cx - size * 0.7,
                cy - size * 0.6,
                cx,
                cy - size * 0.3,
                cx,
                cy - size * 0.3
            );
            let d2 = format!(
                " M{},{} C{},{} {},{} {},{} C{},{} {},{} {},{} Z",
                cx,
                cy + size * 0.7,
                cx + size * 0.1,
                cy + size * 0.5,
                cx + size * 0.7,
                cy + size * 0.2,
                cx + size * 0.7,
                cy - size * 0.1,
                cx + size * 0.7,
                cy - size * 0.6,
                cx,
                cy - size * 0.3,
                cx,
                cy - size * 0.3
            );
            Path::new().set("d", format!("{}{}", d, d2))
        }
        Suit::Club => {
            let d = format!(
                "M{},{} C{},{} {},{} {},{} M{},{} C{},{} {},{} {},{} M{},{} C{},{} {},{} {},{} M{},{} L{},{}",
                cx,
                cy + size * 0.6,
                cx - size * 0.2,
                cy + size * 0.3,
                cx - size * 0.5,
                cy,
                cx - size * 0.3,
                cy - size * 0.4,
                cx,
                cy + size * 0.6,
                cx + size * 0.2,
                cy + size * 0.3,
                cx + size * 0.5,
                cy,
                cx + size * 0.3,
                cy - size * 0.4,
                cx,
                cy + size * 0.6,
                cx,
                cy + size * 0.2,
                cx,
                cy - size * 0.3,
                cx,
                cy - size * 0.6,
                cx,
                cy + size * 0.6,
                cx,
                cy + size * 0.8
            );
            Path::new().set("d", d)
        }
        Suit::Diamond => {
            let d = format!(
                "M{},{} L{},{} L{},{} L{},{} Z M{},{} L{},{} M{},{} L{},{}",
                cx,
                cy - size * 0.8,
                cx + size * 0.5,
                cy,
                cx,
                cy + size * 0.8,
                cx - size * 0.5,
                cy,
                cx - size * 0.25,
                cy - size * 0.4,
                cx + size * 0.25,
                cy - size * 0.4,
                cx - size * 0.25,
                cy + size * 0.4,
                cx + size * 0.25,
                cy + size * 0.4
            );
            Path::new().set("d", d)
        }
    }
}

fn ace_core(suit: Suit, primary: &str, secondary: &str, highlight: &str, seed: u32) -> Group {
    let mut g = Group::new();
    let shape = suit_shape(suit, CX, CY, 200.0, seed)
        .set("fill", primary)
        .set("stroke", secondary)
        .set("stroke-width", 2.5)
        .set("opacity", 0.9);
    g = g.add(shape);
    let inner = Path::new()
        .set("d", ellipse_path(CX, CY, 60.0, 80.0))
        .set("fill", "none")
        .set("stroke", highlight)
        .set("stroke-width", 1.5)
        .set("opacity", 0.7);
    g = g.add(inner);
    for i in 0..12 {
        let angle = (i as f64 / 12.0) * std::f64::consts::TAU;
        let x1 = CX + angle.cos() * 90.0;
        let y1 = CY + angle.sin() * 90.0;
        let x2 = CX + angle.cos() * 170.0;
        let y2 = CY + angle.sin() * 170.0;
        let d = format!("M{:.1},{:.1} L{:.1},{:.1}", x1, y1, x2, y2);
        g = g.add(
            Path::new()
                .set("d", d)
                .set("stroke", secondary)
                .set("stroke-width", 0.8)
                .set("opacity", 0.4)
                .set("fill", "none"),
        );
    }
    g
}

fn two_core(suit: Suit, primary: &str, secondary: &str, seed: u32) -> Group {
    let mut g = Group::new();
    let y_off = 200.0;
    g = g.add(
        suit_shape(suit, CX, CY - y_off, 100.0, seed)
            .set("fill", primary)
            .set("stroke", secondary)
            .set("stroke-width", 2),
    );
    g = g.add(
        suit_shape(suit, CX, CY + y_off, 100.0, seed + 1)
            .set("fill", primary)
            .set("stroke", secondary)
            .set("stroke-width", 2),
    );
    let pts = vec![
        (CX, CY - y_off + 80.0),
        (CX + 60.0, CY),
        (CX, CY + y_off - 80.0),
    ];
    let d = organic_path(&pts, 5.0, seed + 2);
    g = g.add(
        Path::new()
            .set("d", d)
            .set("fill", "none")
            .set("stroke", secondary)
            .set("stroke-width", 1.5)
            .set("opacity", 0.6),
    );
    let pts2 = vec![
        (CX, CY - y_off + 80.0),
        (CX - 60.0, CY),
        (CX, CY + y_off - 80.0),
    ];
    let d2 = organic_path(&pts2, 5.0, seed + 3);
    g = g.add(
        Path::new()
            .set("d", d2)
            .set("fill", "none")
            .set("stroke", secondary)
            .set("stroke-width", 1.5)
            .set("opacity", 0.6),
    );
    g
}

fn three_core(suit: Suit, primary: &str, secondary: &str, seed: u32) -> Group {
    let mut g = Group::new();
    let points = radial_distribute(3, CX, CY, 180.0);
    for (i, &(px, py)) in points.iter().enumerate() {
        g = g.add(
            suit_shape(suit, px, py, 80.0, seed + i as u32)
                .set("fill", primary)
                .set("stroke", secondary)
                .set("stroke-width", 2),
        );
        let d = format!("M{:.1},{:.1} L{:.1},{:.1}", CX, CY, px, py);
        g = g.add(
            Path::new()
                .set("d", d)
                .set("fill", "none")
                .set("stroke", secondary)
                .set("stroke-width", 1.0)
                .set("opacity", 0.5),
        );
    }
    g = g.add(
        Circle::new()
            .set("cx", CX)
            .set("cy", CY)
            .set("r", 15)
            .set("fill", secondary)
            .set("opacity", 0.6),
    );
    g
}

fn four_core(suit: Suit, primary: &str, secondary: &str, seed: u32) -> Group {
    let mut g = Group::new();
    let offsets = [
        (-120.0, -160.0),
        (120.0, -160.0),
        (-120.0, 160.0),
        (120.0, 160.0),
    ];
    for (i, &(dx, dy)) in offsets.iter().enumerate() {
        g = g.add(
            suit_shape(suit, CX + dx, CY + dy, 80.0, seed + i as u32)
                .set("fill", primary)
                .set("stroke", secondary)
                .set("stroke-width", 2),
        );
    }
    let d = format!(
        "M{},{} L{},{} M{},{} L{},{} M{},{} L{},{} M{},{} L{},{}",
        CX - 120.0,
        CY - 160.0,
        CX + 120.0,
        CY - 160.0,
        CX - 120.0,
        CY + 160.0,
        CX + 120.0,
        CY + 160.0,
        CX - 120.0,
        CY - 160.0,
        CX - 120.0,
        CY + 160.0,
        CX + 120.0,
        CY - 160.0,
        CX + 120.0,
        CY + 160.0
    );
    g = g.add(
        Path::new()
            .set("d", d)
            .set("fill", "none")
            .set("stroke", secondary)
            .set("stroke-width", 1.0)
            .set("opacity", 0.4),
    );
    g
}

fn five_core(suit: Suit, primary: &str, secondary: &str, seed: u32) -> Group {
    let mut g = four_core(suit, primary, secondary, seed);
    g = g.add(
        suit_shape(suit, CX, CY, 70.0, seed + 10)
            .set("fill", "none")
            .set("stroke", secondary)
            .set("stroke-width", 1.5)
            .set("stroke-dasharray", "8,4")
            .set("opacity", 0.7),
    );
    for i in 0..3 {
        let angle = (i as f64 / 3.0) * std::f64::consts::TAU + 0.5;
        let x = CX + angle.cos() * 60.0;
        let y = CY + angle.sin() * 60.0;
        let d = ellipse_path(x, y, 15.0, 10.0);
        g = g.add(
            Path::new()
                .set("d", d)
                .set("fill", "none")
                .set("stroke", secondary)
                .set("stroke-width", 0.8)
                .set("stroke-dasharray", "3,3")
                .set("opacity", 0.5),
        );
    }
    g
}

fn six_core(suit: Suit, primary: &str, secondary: &str, seed: u32) -> Group {
    let mut g = Group::new();
    let points = radial_distribute(6, CX, CY, 170.0);
    for (i, &(px, py)) in points.iter().enumerate() {
        g = g.add(
            suit_shape(suit, px, py, 65.0, seed + i as u32)
                .set("fill", primary)
                .set("stroke", secondary)
                .set("stroke-width", 1.8),
        );
        let d = format!("M{:.1},{:.1} L{:.1},{:.1}", px, py, CX, CY);
        g = g.add(
            Path::new()
                .set("d", d)
                .set("fill", "none")
                .set("stroke", secondary)
                .set("stroke-width", 0.8)
                .set("opacity", 0.4),
        );
    }
    g = g.add(
        Path::new()
            .set("d", ellipse_path(CX, CY, 170.0, 170.0))
            .set("fill", "none")
            .set("stroke", secondary)
            .set("stroke-width", 1.0)
            .set("opacity", 0.3),
    );
    g
}

fn seven_core(suit: Suit, primary: &str, secondary: &str, seed: u32) -> Group {
    let mut g = Group::new();
    let points = radial_distribute(7, CX, CY, 160.0);
    for (i, &(px, py)) in points.iter().enumerate() {
        let (dx, dy) = noise_offset(px, py, 15.0, seed + i as u32);
        g = g.add(
            suit_shape(suit, px + dx, py + dy, 55.0, seed + i as u32)
                .set("fill", primary)
                .set("stroke", secondary)
                .set("stroke-width", 1.5),
        );
    }
    g
}

fn eight_core(suit: Suit, primary: &str, secondary: &str, seed: u32) -> Group {
    let mut g = Group::new();
    // Two rings separated further apart, smaller elements to avoid overlap
    let ring1 = radial_distribute(4, CX, CY - 180.0, 120.0);
    let ring2 = radial_distribute(4, CX, CY + 180.0, 120.0);
    for (i, &(px, py)) in ring1.iter().enumerate() {
        g = g.add(
            suit_shape(suit, px, py, 42.0, seed + i as u32)
                .set("fill", primary)
                .set("stroke", secondary)
                .set("stroke-width", 1.5),
        );
    }
    for (i, &(px, py)) in ring2.iter().enumerate() {
        g = g.add(
            suit_shape(suit, px, py, 42.0, seed + 4 + i as u32)
                .set("fill", primary)
                .set("stroke", secondary)
                .set("stroke-width", 1.5),
        );
    }
    g = g.add(
        Path::new()
            .set("d", ellipse_path(CX, CY - 180.0, 120.0, 120.0))
            .set("fill", "none")
            .set("stroke", secondary)
            .set("stroke-width", 0.8)
            .set("opacity", 0.3),
    );
    g = g.add(
        Path::new()
            .set("d", ellipse_path(CX, CY + 180.0, 120.0, 120.0))
            .set("fill", "none")
            .set("stroke", secondary)
            .set("stroke-width", 0.8)
            .set("opacity", 0.3),
    );
    // Connection between the two rings
    let pts = vec![(CX, CY - 60.0), (CX + 15.0, CY), (CX, CY + 60.0)];
    let d = organic_path(&pts, 3.0, seed + 8);
    g = g.add(
        Path::new()
            .set("d", d)
            .set("fill", "none")
            .set("stroke", secondary)
            .set("stroke-width", 1.0)
            .set("opacity", 0.4),
    );
    g
}

fn nine_core(suit: Suit, primary: &str, secondary: &str, seed: u32) -> Group {
    let mut g = Group::new();
    let positions = [
        (CX, CY),
        (CX - 130.0, CY - 180.0),
        (CX + 130.0, CY - 180.0),
        (CX - 180.0, CY - 60.0),
        (CX + 180.0, CY - 60.0),
        (CX - 180.0, CY + 60.0),
        (CX + 180.0, CY + 60.0),
        (CX - 130.0, CY + 180.0),
        (CX + 130.0, CY + 180.0),
    ];
    for (i, &(px, py)) in positions.iter().enumerate() {
        let size = if i == 0 { 60.0 } else { 45.0 };
        g = g.add(
            suit_shape(suit, px, py, size, seed + i as u32)
                .set("fill", primary)
                .set("stroke", secondary)
                .set("stroke-width", 1.5),
        );
    }
    for &(px, py) in &positions[1..] {
        let d = format!("M{:.1},{:.1} L{:.1},{:.1}", px, py, CX, CY);
        g = g.add(
            Path::new()
                .set("d", d)
                .set("fill", "none")
                .set("stroke", secondary)
                .set("stroke-width", 0.5)
                .set("opacity", 0.3),
        );
    }
    g
}

fn ten_core(suit: Suit, primary: &str, secondary: &str, seed: u32) -> Group {
    let mut g = Group::new();
    let positions = [
        (CX - 100.0, CY - 250.0),
        (CX + 100.0, CY - 250.0),
        (CX - 150.0, CY - 100.0),
        (CX, CY - 100.0),
        (CX + 150.0, CY - 100.0),
        (CX - 150.0, CY + 100.0),
        (CX, CY + 100.0),
        (CX + 150.0, CY + 100.0),
        (CX - 100.0, CY + 250.0),
        (CX + 100.0, CY + 250.0),
    ];
    for (i, &(px, py)) in positions.iter().enumerate() {
        g = g.add(
            suit_shape(suit, px, py, 45.0, seed + i as u32)
                .set("fill", primary)
                .set("stroke", secondary)
                .set("stroke-width", 1.5),
        );
    }
    for row in &[
        &positions[0..2],
        &positions[2..5],
        &positions[5..8],
        &positions[8..10],
    ] {
        for w in row.windows(2) {
            let d = format!("M{:.1},{:.1} L{:.1},{:.1}", w[0].0, w[0].1, w[1].0, w[1].1);
            g = g.add(
                Path::new()
                    .set("d", d)
                    .set("fill", "none")
                    .set("stroke", secondary)
                    .set("stroke-width", 0.6)
                    .set("opacity", 0.3),
            );
        }
    }
    g
}

fn face_core(
    rank: Rank,
    suit: Suit,
    primary: &str,
    secondary: &str,
    highlight: &str,
    seed: u32,
) -> Group {
    let mut g = Group::new();
    // Central figure frame
    let frame_d = rounded_rect(200.0, 200.0, 600.0, 1000.0, 40.0);
    g = g.add(
        Path::new()
            .set("d", frame_d)
            .set("fill", "none")
            .set("stroke", secondary)
            .set("stroke-width", 2.0)
            .set("opacity", 0.5),
    );

    // Central suit emblem (smaller, behind figure)
    g = g.add(
        suit_shape(suit, CX, CY, 80.0, seed)
            .set("fill", primary)
            .set("stroke", highlight)
            .set("stroke-width", 2.0)
            .set("opacity", 0.4),
    );

    // --- Humanoid figure (top half, anti-symmetric with bottom) ---
    // Top half figure occupies y=220..700, bottom half is 180° rotated copy
    let top_figure = match rank {
        Rank::Jack => jack_figure(suit, primary, secondary, highlight, seed),
        Rank::Queen => queen_figure(suit, primary, secondary, highlight, seed),
        Rank::King => king_figure(suit, primary, secondary, highlight, seed),
        _ => Group::new(),
    };
    g = g.add(top_figure);

    // Bottom half: 180° rotated mirror of the figure
    let bottom_figure = match rank {
        Rank::Jack => jack_figure(suit, primary, secondary, highlight, seed + 50),
        Rank::Queen => queen_figure(suit, primary, secondary, highlight, seed + 50),
        Rank::King => king_figure(suit, primary, secondary, highlight, seed + 50),
        _ => Group::new(),
    };
    g = g.add(bottom_figure.set("transform", format!("rotate(180,{},{})", CX, CY)));

    g
}

/// Jack — "The Unfolding Seed": an opening spiral-blade form
/// Symbolizes potential, youth, emergence — purely abstract, no human form
fn jack_figure(_suit: Suit, _primary: &str, secondary: &str, highlight: &str, seed: u32) -> Group {
    let mut g = Group::new();
    let cy = 430.0; // center of top half
    let j = 8.0;

    // Central seed pod — organic closed form, the dormant core
    let pod = organic_path(
        &[
            (CX, cy - 60.0),
            (CX + 35.0, cy - 30.0),
            (CX + 28.0, cy + 20.0),
            (CX, cy + 45.0),
            (CX - 28.0, cy + 20.0),
            (CX - 35.0, cy - 30.0),
            (CX, cy - 60.0),
        ],
        j,
        seed,
    );
    g = g.add(
        Path::new()
            .set("d", pod)
            .set("fill", "none")
            .set("stroke", highlight)
            .set("stroke-width", 2.2),
    );

    // Inner void — smaller echo of the pod shape
    let inner = organic_path(
        &[
            (CX, cy - 30.0),
            (CX + 16.0, cy - 12.0),
            (CX + 12.0, cy + 10.0),
            (CX, cy + 22.0),
            (CX - 12.0, cy + 10.0),
            (CX - 16.0, cy - 12.0),
            (CX, cy - 30.0),
        ],
        j * 0.5,
        seed + 1,
    );
    g = g.add(
        Path::new()
            .set("d", inner)
            .set("fill", "none")
            .set("stroke", secondary)
            .set("stroke-width", 1.2)
            .set("opacity", 0.6),
    );

    // Unfolding tendrils — spiraling outward from the seed, Mucha curves
    let tendril_angles: [(f64, f64, f64); 5] = [
        (-0.8, 1.0, 120.0),
        (-0.3, 0.7, 140.0),
        (0.2, 1.3, 130.0),
        (0.7, 0.5, 110.0),
        (1.2, 1.1, 125.0),
    ];
    for (i, &(angle_start, curve_bias, length)) in tendril_angles.iter().enumerate() {
        let base_angle = angle_start + std::f64::consts::FRAC_PI_4;
        let pts: Vec<(f64, f64)> = (0..6)
            .map(|s| {
                let t = s as f64 / 5.0;
                let a = base_angle + t * curve_bias * 1.5;
                let r = 40.0 + t * length;
                (CX + a.cos() * r, cy + a.sin() * r)
            })
            .collect();
        let d = organic_path(&pts, j * 1.3, seed + 2 + i as u32);
        g = g.add(
            Path::new()
                .set("d", d)
                .set("fill", "none")
                .set("stroke", secondary)
                .set("stroke-width", 1.5)
                .set("opacity", 0.5 + t_opacity(i, 5)),
        );
    }

    // Klimt dot clusters along tendrils — golden accents
    for i in 0..7 {
        let angle = (i as f64 / 7.0) * std::f64::consts::TAU;
        let r = 90.0 + (i as f64 * 13.0) % 40.0;
        let (dx, dy) = noise_offset(
            CX + angle.cos() * r,
            cy + angle.sin() * r,
            8.0,
            seed + 20 + i,
        );
        g = g.add(
            Circle::new()
                .set("cx", CX + angle.cos() * r + dx)
                .set("cy", cy + angle.sin() * r + dy)
                .set("r", 2.5 + (i as f64 % 3.0))
                .set("fill", highlight)
                .set("opacity", 0.4),
        );
    }

    // Giger biomech rings — concentric broken arcs around the seed
    for i in 0..3 {
        let r = 55.0 + i as f64 * 25.0;
        let arc_pts: Vec<(f64, f64)> = (0..8)
            .map(|s| {
                let a = (s as f64 / 8.0) * std::f64::consts::PI * 1.4 - 0.7;
                (CX + a.cos() * r, cy + a.sin() * r)
            })
            .collect();
        let d = organic_path(&arc_pts, j * 0.6, seed + 10 + i as u32);
        g = g.add(
            Path::new()
                .set("d", d)
                .set("fill", "none")
                .set("stroke", secondary)
                .set("stroke-width", 0.7)
                .set("opacity", 0.3),
        );
    }

    g
}

fn t_opacity(i: usize, total: usize) -> f64 {
    (i as f64 / total as f64) * 0.3
}

/// Queen — "The Enclosing Shell": concentric organic membranes
/// Symbolizes protection, containment, cyclical power — purely abstract
fn queen_figure(_suit: Suit, _primary: &str, secondary: &str, highlight: &str, seed: u32) -> Group {
    let mut g = Group::new();
    let cy = 430.0;
    let j = 7.0;

    // Concentric organic shells — each slightly different, breathing outward
    let shell_radii = [35.0, 65.0, 100.0, 140.0, 175.0];
    for (i, &r) in shell_radii.iter().enumerate() {
        let steps = 12 + i * 2;
        let pts: Vec<(f64, f64)> = (0..=steps)
            .map(|s| {
                let a = (s as f64 / steps as f64) * std::f64::consts::TAU;
                let rx = r * (1.0 + 0.15 * ((a * 3.0 + i as f64).sin()));
                let ry = r * 1.2 * (1.0 + 0.1 * ((a * 2.0 + i as f64 * 0.7).cos()));
                (CX + a.cos() * rx, cy + a.sin() * ry)
            })
            .collect();
        let d = organic_path(&pts, j * (0.5 + i as f64 * 0.2), seed + i as u32);
        let opacity = 0.7 - i as f64 * 0.1;
        let sw = 2.0 - i as f64 * 0.2;
        let color = if i % 2 == 0 { highlight } else { secondary };
        g = g.add(
            Path::new()
                .set("d", d)
                .set("fill", "none")
                .set("stroke", color)
                .set("stroke-width", sw)
                .set("opacity", opacity),
        );
    }

    // Inner sanctum — a small dense form at the very center
    let core_pts: Vec<(f64, f64)> = (0..=8)
        .map(|s| {
            let a = (s as f64 / 8.0) * std::f64::consts::TAU;
            let r = 15.0 + 5.0 * (a * 2.0).sin();
            (CX + a.cos() * r, cy + a.sin() * r * 1.3)
        })
        .collect();
    let core_d = organic_path(&core_pts, j * 0.3, seed + 10);
    g = g.add(
        Path::new()
            .set("d", core_d)
            .set("fill", "none")
            .set("stroke", highlight)
            .set("stroke-width", 2.5),
    );

    // Connecting filaments — radial threads between shells, Giger membrane feel
    for i in 0..8 {
        let angle = (i as f64 / 8.0) * std::f64::consts::TAU + 0.2;
        let pts: Vec<(f64, f64)> = (0..5)
            .map(|s| {
                let r = 20.0 + s as f64 * 38.0;
                let wobble = ((s as f64 + i as f64) * 1.3).sin() * 8.0;
                (CX + angle.cos() * r + wobble, cy + angle.sin() * r * 1.2)
            })
            .collect();
        let d = organic_path(&pts, j * 0.8, seed + 20 + i as u32);
        g = g.add(
            Path::new()
                .set("d", d)
                .set("fill", "none")
                .set("stroke", secondary)
                .set("stroke-width", 0.6)
                .set("opacity", 0.3),
        );
    }

    // Klimt spiral accents — at cardinal points of the outermost shell
    for i in 0..4 {
        let angle = (i as f64 / 4.0) * std::f64::consts::TAU;
        let sx = CX + angle.cos() * 160.0;
        let sy = cy + angle.sin() * 190.0;
        let sp = spiral(sx, sy, 3.0, 15.0, 1.5, 14);
        let sp_d = organic_path(&sp, 3.0, seed + 30 + i as u32);
        g = g.add(
            Path::new()
                .set("d", sp_d)
                .set("fill", "none")
                .set("stroke", highlight)
                .set("stroke-width", 0.8)
                .set("opacity", 0.45),
        );
    }

    g
}

/// King — "The Axis Spire": a vertical crystalline tower / biomechanical obelisk
/// Symbolizes dominance, authority, immovable power — purely abstract
fn king_figure(_suit: Suit, _primary: &str, secondary: &str, highlight: &str, seed: u32) -> Group {
    let mut g = Group::new();
    let cy = 430.0;
    let j = 6.0;

    // Central spire — tall narrow diamond/obelisk form
    for i in 0..8 {
        let spire = organic_path(
            &[
                (CX, cy - 180.0),
                (CX + 40.0, cy - 80.0),
                (CX + 55.0, cy),
                (CX + 40.0, cy + 80.0),
                (CX, cy + 180.0),
                (CX - 40.0, cy + 80.0),
                (CX - 55.0, cy),
                (CX - 40.0, cy - 80.0),
                (CX, cy - 180.0),
            ],
            j,
            seed,
        );
        g = g.add(
            Path::new()
                .set("d", spire)
                .set("fill", "none")
                .set("stroke", highlight)
                .set("stroke-width", 2.5)
                .set("transform", format!("rotate({} {} {})", i * 45, CX, cy)), // subtle rotation for a dynamic, crystalline feel
        );

        // Inner spire echo — smaller, tighter
        let inner_spire = organic_path(
            &[
                (CX, cy - 120.0),
                (CX + 22.0, cy - 50.0),
                (CX + 30.0, cy),
                (CX + 22.0, cy + 50.0),
                (CX, cy + 120.0),
                (CX - 22.0, cy + 50.0),
                (CX - 30.0, cy),
                (CX - 22.0, cy - 50.0),
                (CX, cy - 120.0),
            ],
            j * 0.6,
            seed + 1,
        );
        g = g.add(
            Path::new()
                .set("d", inner_spire)
                .set("fill", "none")
                .set("stroke", secondary)
                .set("stroke-width", 1.8)
                .set("opacity", 0.7)
                .set("transform", format!("rotate({} {} {})", i * 45, CX, cy)), // subtle rotation for a dynamic, crystalline feel
        );
    }

    // Horizontal strata — layered cross-bars, Giger vertebrae feel
    for i in 0..9 {
        let y = cy - 160.0 + i as f64 * 40.0;
        let t = (y - (cy - 180.0)) / 360.0; // 0..1 along spire height
        let half_w = 15.0 + (0.5 - (t - 0.5).abs()) * 80.0; // widest at center
        let bar = organic_path(
            &[
                (CX - half_w, y),
                (CX - half_w * 0.3, y - 3.0),
                (CX + half_w * 0.3, y - 3.0),
                (CX + half_w, y),
            ],
            j * 0.5,
            seed + 10 + i as u32,
        );
        g = g.add(
            Path::new()
                .set("d", bar)
                .set("fill", "none")
                .set("stroke", secondary)
                .set("stroke-width", 0.8)
                .set("opacity", 0.4),
        );
    }

    // Flanking buttresses — angular supports on each side
    let buttress_l = organic_path(
        &[
            (CX - 55.0, cy),
            (CX - 100.0, cy - 40.0),
            (CX - 120.0, cy - 100.0),
            (CX - 100.0, cy - 140.0),
        ],
        j * 1.2,
        seed + 2,
    );
    let buttress_r = organic_path(
        &[
            (CX + 55.0, cy),
            (CX + 100.0, cy - 40.0),
            (CX + 120.0, cy - 100.0),
            (CX + 100.0, cy - 140.0),
        ],
        j * 1.2,
        seed + 3,
    );
    g = g.add(
        Path::new()
            .set("d", buttress_l)
            .set("fill", "none")
            .set("stroke", secondary)
            .set("stroke-width", 1.4)
            .set("opacity", 0.5),
    );
    g = g.add(
        Path::new()
            .set("d", buttress_r)
            .set("fill", "none")
            .set("stroke", secondary)
            .set("stroke-width", 1.4)
            .set("opacity", 0.5),
    );

    // Lower buttresses — mirrored downward
    let buttress_l2 = organic_path(
        &[
            (CX - 55.0, cy),
            (CX - 100.0, cy + 40.0),
            (CX - 120.0, cy + 100.0),
            (CX - 100.0, cy + 140.0),
        ],
        j * 1.2,
        seed + 4,
    );
    let buttress_r2 = organic_path(
        &[
            (CX + 55.0, cy),
            (CX + 100.0, cy + 40.0),
            (CX + 120.0, cy + 100.0),
            (CX + 100.0, cy + 140.0),
        ],
        j * 1.2,
        seed + 5,
    );
    g = g.add(
        Path::new()
            .set("d", buttress_l2)
            .set("fill", "none")
            .set("stroke", secondary)
            .set("stroke-width", 1.4)
            .set("opacity", 0.5),
    );
    g = g.add(
        Path::new()
            .set("d", buttress_r2)
            .set("fill", "none")
            .set("stroke", secondary)
            .set("stroke-width", 1.4)
            .set("opacity", 0.5),
    );

    // Klimt gold dots — scattered along the spire axis
    for i in 0..10 {
        let y = cy - 150.0 + i as f64 * 30.0;
        let (dx, dy) = noise_offset(CX, y, 12.0, seed + 40 + i);
        g = g.add(
            Circle::new()
                .set("cx", CX + dx)
                .set("cy", y + dy)
                .set("r", 2.0 + (i as f64 % 3.0) * 0.8)
                .set("fill", highlight)
                .set("opacity", 0.5),
        );
    }

    g
}

fn generate_joker(card: &Card, seed: u32) -> Group {
    let mut g = Group::new().set("id", "core");
    let pals = crate::palette::joker_palettes();
    let n = match card {
        Card::Joker(n) => *n,
        _ => 1,
    };
    let is_big = n == 1; // 大王

    // --- Shared structure: central spiral (different rotation) ---
    let turns = if is_big { 5.0 } else { 3.0 };
    let spiral_pts = spiral(CX, CY, if is_big { 15.0 } else { 30.0 }, 220.0, turns, 80);
    let d = organic_path(&spiral_pts, if is_big { 12.0 } else { 6.0 }, seed);
    g = g.add(
        Path::new()
            .set("d", d)
            .set("fill", "none")
            .set(
                "stroke",
                if is_big {
                    pals[1].secondary
                } else {
                    pals[0].secondary
                },
            )
            .set("stroke-width", if is_big { 2.5 } else { 1.5 })
            .set("opacity", 0.7),
    );

    // --- Four suit symbols (shared, but different arrangement) ---
    if is_big {
        // 大王: diamond arrangement, larger
        let offsets = [(0.0, -150.0), (130.0, 0.0), (0.0, 150.0), (-130.0, 0.0)];
        let suits = [Suit::Spade, Suit::Heart, Suit::Club, Suit::Diamond];
        for (i, (&(dx, dy), &suit)) in offsets.iter().zip(suits.iter()).enumerate() {
            g = g.add(
                suit_shape(suit, CX + dx, CY + dy, 55.0, seed + i as u32)
                    .set("fill", pals[i].primary)
                    .set("stroke", pals[i].secondary)
                    .set("stroke-width", 2.0),
            );
        }
    } else {
        // 小王: square arrangement, smaller
        let offsets = [
            (-90.0, -100.0),
            (90.0, -100.0),
            (-90.0, 100.0),
            (90.0, 100.0),
        ];
        let suits = [Suit::Spade, Suit::Heart, Suit::Club, Suit::Diamond];
        for (i, (&(dx, dy), &suit)) in offsets.iter().zip(suits.iter()).enumerate() {
            g = g.add(
                suit_shape(suit, CX + dx, CY + dy, 40.0, seed + i as u32)
                    .set("fill", pals[i].primary)
                    .set("stroke", pals[i].secondary)
                    .set("stroke-width", 1.5),
            );
        }
    }

    // --- Central star/eye motif ---
    if is_big {
        // 大王: radiant star
        for i in 0..8 {
            let angle = (i as f64 / 8.0) * std::f64::consts::TAU;
            let r1 = 40.0;
            let r2 = 80.0;
            let d = format!(
                "M{:.1},{:.1} L{:.1},{:.1}",
                CX + angle.cos() * r1,
                CY + angle.sin() * r1,
                CX + angle.cos() * r2,
                CY + angle.sin() * r2
            );
            g = g.add(
                Path::new()
                    .set("d", d)
                    .set("fill", "none")
                    .set("stroke", pals[1].highlight)
                    .set("stroke-width", 1.5)
                    .set("opacity", 0.6),
            );
        }
        g = g.add(
            Path::new()
                .set("d", ellipse_path(CX, CY, 35.0, 35.0))
                .set("fill", "none")
                .set("stroke", pals[1].highlight)
                .set("stroke-width", 2.0)
                .set("opacity", 0.5),
        );
    } else {
        // 小王: crescent moon
        let d = format!(
            "M{},{} A30,30 0 1,1 {},{} A22,22 0 1,0 {},{}",
            CX - 20.0,
            CY - 25.0,
            CX - 20.0,
            CY + 25.0,
            CX - 20.0,
            CY - 25.0
        );
        g = g.add(
            Path::new()
                .set("d", d)
                .set("fill", "none")
                .set("stroke", pals[0].highlight)
                .set("stroke-width", 1.8)
                .set("opacity", 0.6),
        );
    }

    // --- Joker label ---
    let text_color = if is_big { "#C41E3A" } else { "#D4C5A9" };
    let font_size = if is_big { 52 } else { 44 };
    let label = svg::node::element::Text::new("JOKER")
        .set("x", CX)
        .set("y", 130)
        .set("font-size", font_size)
        .set("font-family", "serif")
        .set("fill", text_color)
        .set("text-anchor", "middle")
        .set("letter-spacing", if is_big { "6" } else { "2" });
    g = g.add(label);

    let label2 = svg::node::element::Text::new("JOKER")
        .set("x", CX)
        .set("y", 1375)
        .set("font-size", font_size)
        .set("font-family", "serif")
        .set("fill", text_color)
        .set("text-anchor", "middle")
        .set("letter-spacing", if is_big { "6" } else { "2" })
        .set("transform", format!("rotate(180,{},1320)", CX));
    g = g.add(label2);

    // --- Decorative border difference ---
    if is_big {
        // 大王: double border accent
        g = g.add(
            Path::new()
                .set("d", ellipse_path(CX, CY, 300.0, 420.0))
                .set("fill", "none")
                .set("stroke", pals[1].secondary)
                .set("stroke-width", 1.5)
                .set("opacity", 0.25),
        );
        g = g.add(
            Path::new()
                .set("d", ellipse_path(CX, CY, 310.0, 430.0))
                .set("fill", "none")
                .set("stroke", pals[1].secondary)
                .set("stroke-width", 0.8)
                .set("opacity", 0.15),
        );
    } else {
        // 小王: single dashed border
        g = g.add(
            Path::new()
                .set("d", ellipse_path(CX, CY, 280.0, 400.0))
                .set("fill", "none")
                .set("stroke", pals[0].secondary)
                .set("stroke-width", 1.2)
                .set("stroke-dasharray", "12,6")
                .set("opacity", 0.25),
        );
    }

    g
}
