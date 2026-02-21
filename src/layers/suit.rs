use crate::card::{Card, Suit};
use crate::geometry::*;
use crate::palette::palette_for;
use svg::node::element::{Group, Path};

/// Generate suit texture overlay (20% visual weight)
pub fn generate(card: &Card) -> Group {
    let pal = palette_for(card);
    let seed = card.seed() as u32 + 2000;
    let suit = card.suit().unwrap_or(Suit::Spade);
    let mut g = Group::new().set("id", "suit-texture").set("opacity", 0.5);

    match suit {
        Suit::Spade => g = g.add(rib_lines(pal.secondary, pal.highlight, seed)),
        Suit::Heart => g = g.add(flowing_veins(pal.secondary, pal.highlight, seed)),
        Suit::Club => g = g.add(branching_nodes(pal.secondary, pal.highlight, seed)),
        Suit::Diamond => g = g.add(faceted_lines(pal.secondary, pal.highlight, seed)),
    }

    g
}

/// Spade: parallel rib lines + hollow channels (bone/mechanical)
fn rib_lines(color: &str, highlight: &str, seed: u32) -> Group {
    let mut g = Group::new();
    // Horizontal rib lines across the card
    for i in 0..18 {
        let y = 120.0 + i as f64 * 70.0;
        let x_start = 100.0 + (i as f64 * 0.7).sin().abs() * 40.0;
        let x_end = 900.0 - (i as f64 * 0.7).sin().abs() * 40.0;
        let pts = vec![
            (x_start, y),
            (x_start + 100.0, y + 3.0),
            (CX, y - 2.0),
            (x_end - 100.0, y + 3.0),
            (x_end, y),
        ];
        let d = organic_path(&pts, 2.0, seed + i);
        g = g.add(
            Path::new()
                .set("d", d)
                .set("fill", "none")
                .set("stroke", color)
                .set("stroke-width", 0.6)
                .set("opacity", 0.4),
        );
    }
    // Hollow channels (vertical gaps)
    for i in 0..5 {
        let x = 200.0 + i as f64 * 150.0;
        let pts = vec![
            (x, 150.0),
            (x + 5.0, 400.0),
            (x - 5.0, 700.0),
            (x + 5.0, 1000.0),
            (x, 1250.0),
        ];
        let d = organic_path(&pts, 3.0, seed + 20 + i);
        g = g.add(
            Path::new()
                .set("d", d)
                .set("fill", "none")
                .set("stroke", highlight)
                .set("stroke-width", 0.4)
                .set("opacity", 0.3),
        );
    }
    g
}

/// Heart: flowing vein curves (organic, pulsing)
fn flowing_veins(color: &str, highlight: &str, seed: u32) -> Group {
    let mut g = Group::new();
    // Main veins radiating from center
    for i in 0..8 {
        let angle = (i as f64 / 8.0) * std::f64::consts::TAU;
        let x_end = CX + angle.cos() * 400.0;
        let y_end = CY + angle.sin() * 550.0;
        let mid_x = CX + angle.cos() * 200.0 + (i as f64 * 1.3).sin() * 40.0;
        let mid_y = CY + angle.sin() * 275.0 + (i as f64 * 1.7).cos() * 40.0;
        let pts = vec![(CX, CY), (mid_x, mid_y), (x_end, y_end)];
        let d = organic_path(&pts, 8.0, seed + i);
        g = g.add(
            Path::new()
                .set("d", d)
                .set("fill", "none")
                .set("stroke", color)
                .set("stroke-width", 1.2)
                .set("opacity", 0.5),
        );

        // Branch veins
        for j in 0..3 {
            let t = 0.3 + j as f64 * 0.25;
            let bx = CX + (x_end - CX) * t;
            let by = CY + (y_end - CY) * t;
            let branch_angle = angle + if j % 2 == 0 { 0.5 } else { -0.5 };
            let bx2 = bx + branch_angle.cos() * 60.0;
            let by2 = by + branch_angle.sin() * 60.0;
            let pts = vec![(bx, by), (bx2, by2)];
            let d = organic_path(&pts, 4.0, seed + 10 + i * 3 + j);
            g = g.add(
                Path::new()
                    .set("d", d)
                    .set("fill", "none")
                    .set("stroke", highlight)
                    .set("stroke-width", 0.5)
                    .set("opacity", 0.3),
            );
        }
    }
    g
}

/// Club: branching dendrite nodes (neural network)
fn branching_nodes(color: &str, highlight: &str, seed: u32) -> Group {
    let mut g = Group::new();
    // Node positions
    let nodes = radial_distribute(12, CX, CY, 280.0);
    let inner_nodes = radial_distribute(6, CX, CY, 140.0);

    // Draw connections between nodes
    for (i, &(x1, y1)) in inner_nodes.iter().enumerate() {
        // Connect to center
        let d = format!("M{:.1},{:.1} L{:.1},{:.1}", CX, CY, x1, y1);
        g = g.add(
            Path::new()
                .set("d", d)
                .set("fill", "none")
                .set("stroke", color)
                .set("stroke-width", 0.8)
                .set("opacity", 0.4),
        );

        // Connect to outer nodes
        let o1 = &nodes[i * 2];
        let o2 = &nodes[(i * 2 + 1) % nodes.len()];
        let pts1 = vec![(x1, y1), (o1.0, o1.1)];
        let d1 = organic_path(&pts1, 5.0, seed + i as u32);
        g = g.add(
            Path::new()
                .set("d", d1)
                .set("fill", "none")
                .set("stroke", color)
                .set("stroke-width", 0.6)
                .set("opacity", 0.35),
        );
        let pts2 = vec![(x1, y1), (o2.0, o2.1)];
        let d2 = organic_path(&pts2, 5.0, seed + 20 + i as u32);
        g = g.add(
            Path::new()
                .set("d", d2)
                .set("fill", "none")
                .set("stroke", color)
                .set("stroke-width", 0.6)
                .set("opacity", 0.35),
        );

        // Node circles
        g = g.add(
            svg::node::element::Circle::new()
                .set("cx", x1)
                .set("cy", y1)
                .set("r", 4)
                .set("fill", highlight)
                .set("opacity", 0.5),
        );
    }

    for &(x, y) in &nodes {
        g = g.add(
            svg::node::element::Circle::new()
                .set("cx", x)
                .set("cy", y)
                .set("r", 3)
                .set("fill", color)
                .set("opacity", 0.4),
        );
    }

    g
}

/// Diamond: faceted geometric lines (crystal/refraction)
fn faceted_lines(color: &str, highlight: &str, seed: u32) -> Group {
    let mut g = Group::new();
    // Concentric diamond shapes
    for i in 0..6 {
        let s = 60.0 + i as f64 * 60.0;
        let (dx, dy) = noise_offset(CX, CY + i as f64 * 100.0, 5.0, seed + i);
        let d = format!(
            "M{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} Z",
            CX + dx,
            CY - s * 1.3 + dy,
            CX + s * 0.8 + dx,
            CY + dy,
            CX + dx,
            CY + s * 1.3 + dy,
            CX - s * 0.8 + dx,
            CY + dy
        );
        g = g.add(
            Path::new()
                .set("d", d)
                .set("fill", "none")
                .set("stroke", color)
                .set("stroke-width", 0.8)
                .set("opacity", 0.3),
        );
    }

    // Diagonal refraction lines
    for i in 0..12 {
        let angle = (i as f64 / 12.0) * std::f64::consts::TAU;
        let x1 = CX + angle.cos() * 50.0;
        let y1 = CY + angle.sin() * 70.0;
        let x2 = CX + angle.cos() * 380.0;
        let y2 = CY + angle.sin() * 530.0;
        let d = format!("M{:.1},{:.1} L{:.1},{:.1}", x1, y1, x2, y2);
        g = g.add(
            Path::new()
                .set("d", d)
                .set("fill", "none")
                .set("stroke", highlight)
                .set("stroke-width", 0.4)
                .set("opacity", 0.25),
        );
    }

    g
}
