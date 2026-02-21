use crate::card::{Card, Suit};
use crate::geometry::*;
use crate::palette::palette_for;
use svg::node::element::{Circle, Group, Path};

/// Generate Klimt-style ornament density layer (10% visual weight)
pub fn generate(card: &Card) -> Group {
    let pal = palette_for(card);
    let seed = card.seed() as u32 + 4000;
    let density = card.complexity();
    let suit = card.suit().unwrap_or(Suit::Spade);
    let mut g = Group::new().set("id", "ornament").set("opacity", 0.4);

    match suit {
        Suit::Spade => g = g.add(ring_halo(density, pal.highlight, seed)),
        Suit::Heart => g = g.add(spiral_dots(density, pal.highlight, pal.secondary, seed)),
        Suit::Club => g = g.add(lattice(density, pal.highlight, seed)),
        Suit::Diamond => g = g.add(micro_rhombus(density, pal.highlight, pal.secondary, seed)),
    }

    g
}

/// Spade ornament: concentric ring halos
fn ring_halo(density: f64, color: &str, seed: u32) -> Group {
    let mut g = Group::new();
    let count = (density * 20.0) as usize + 2;

    for i in 0..count {
        let r = 40.0 + i as f64 * (350.0 / count as f64);
        let (dx, dy) = noise_offset(CX + i as f64 * 10.0, CY, 8.0, seed + i as u32);
        let d = ellipse_path(CX + dx, CY + dy, r, r * 1.1);
        let opacity = 0.15 + density * 0.2;
        g = g.add(
            Path::new()
                .set("d", d)
                .set("fill", "none")
                .set("stroke", color)
                .set("stroke-width", 0.5)
                .set("opacity", opacity),
        );
    }

    // Small decorative dots between rings
    let dot_count = (density * 30.0) as usize;
    for i in 0..dot_count {
        let angle = (i as f64 / dot_count as f64) * std::f64::consts::TAU;
        let r = 80.0 + (i as f64 * 17.3) % 250.0;
        let (dx, dy) = noise_offset(i as f64 * 20.0, r, 5.0, seed + 100 + i as u32);
        let x = CX + angle.cos() * r + dx;
        let y = CY + angle.sin() * r * 1.1 + dy;
        if x > 70.0 && x < 930.0 && y > 70.0 && y < 1330.0 {
            g = g.add(
                Circle::new()
                    .set("cx", x)
                    .set("cy", y)
                    .set("r", 1.5)
                    .set("fill", color)
                    .set("opacity", 0.3),
            );
        }
    }

    g
}

/// Heart ornament: spiral dot patterns
fn spiral_dots(density: f64, color1: &str, color2: &str, seed: u32) -> Group {
    let mut g = Group::new();
    let turns = 2.0 + density * 3.0;
    let dot_count = (density * 60.0) as usize + 10;

    let pts = spiral(CX, CY, 30.0, 350.0, turns, dot_count);
    for (i, &(x, y)) in pts.iter().enumerate() {
        let (dx, dy) = noise_offset(x, y, 10.0, seed + i as u32);
        let px = x + dx;
        let py = y + dy;
        if px > 70.0 && px < 930.0 && py > 70.0 && py < 1330.0 {
            let r = 1.0 + (i as f64 * 0.05) % 3.0;
            let c = if i % 3 == 0 { color1 } else { color2 };
            g = g.add(
                Circle::new()
                    .set("cx", px)
                    .set("cy", py)
                    .set("r", r)
                    .set("fill", c)
                    .set("opacity", 0.35),
            );
        }
    }

    // Secondary spiral (offset)
    let pts2 = spiral(CX, CY, 50.0, 300.0, turns * 0.7, dot_count / 2);
    for (i, &(x, y)) in pts2.iter().enumerate() {
        let (dx, dy) = noise_offset(x + 50.0, y + 50.0, 8.0, seed + 200 + i as u32);
        let px = x + dx;
        let py = y + dy;
        if px > 70.0 && px < 930.0 && py > 70.0 && py < 1330.0 {
            g = g.add(
                Circle::new()
                    .set("cx", px)
                    .set("cy", py)
                    .set("r", 1.0)
                    .set("fill", color1)
                    .set("opacity", 0.2),
            );
        }
    }

    g
}

/// Club ornament: lattice grid pattern
fn lattice(density: f64, color: &str, seed: u32) -> Group {
    let mut g = Group::new();
    let spacing = 80.0 - density * 40.0;
    let spacing = spacing.max(25.0);

    let mut y = 100.0;
    while y < 1300.0 {
        let mut x = 100.0;
        while x < 900.0 {
            let (dx, dy) = noise_offset(x, y, 4.0, seed);
            let px = x + dx;
            let py = y + dy;
            // Small cross at each lattice point
            let size = 3.0 + density * 4.0;
            let d = format!(
                "M{:.1},{:.1} L{:.1},{:.1} M{:.1},{:.1} L{:.1},{:.1}",
                px - size,
                py,
                px + size,
                py,
                px,
                py - size,
                px,
                py + size
            );
            g = g.add(
                Path::new()
                    .set("d", d)
                    .set("fill", "none")
                    .set("stroke", color)
                    .set("stroke-width", 0.4)
                    .set("opacity", 0.25),
            );
            x += spacing;
        }
        y += spacing;
    }

    // Connecting lines between lattice points (sparse)
    let line_count = (density * 15.0) as usize;
    for i in 0..line_count {
        let x1 = 100.0 + (i as f64 * 73.7) % 800.0;
        let y1 = 100.0 + (i as f64 * 97.3) % 1200.0;
        let x2 = x1 + spacing;
        let y2 = y1 + spacing;
        let d = format!("M{:.1},{:.1} L{:.1},{:.1}", x1, y1, x2, y2);
        g = g.add(
            Path::new()
                .set("d", d)
                .set("fill", "none")
                .set("stroke", color)
                .set("stroke-width", 0.3)
                .set("opacity", 0.15),
        );
    }

    g
}

/// Diamond ornament: micro rhombus array
fn micro_rhombus(density: f64, color1: &str, color2: &str, seed: u32) -> Group {
    let mut g = Group::new();
    let count = (density * 40.0) as usize + 5;

    for i in 0..count {
        let angle = (i as f64 / count as f64) * std::f64::consts::TAU * 3.0;
        let r = 50.0 + (i as f64 * 13.7) % 350.0;
        let (dx, dy) = noise_offset(i as f64 * 30.0, r, 10.0, seed + i as u32);
        let x = CX + angle.cos() * r + dx;
        let y = CY + angle.sin() * r * 1.2 + dy;

        if x > 70.0 && x < 930.0 && y > 70.0 && y < 1330.0 {
            let s = 4.0 + density * 6.0;
            let d = format!(
                "M{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} Z",
                x,
                y - s,
                x + s * 0.6,
                y,
                x,
                y + s,
                x - s * 0.6,
                y
            );
            let c = if i % 2 == 0 { color1 } else { color2 };
            g = g.add(
                Path::new()
                    .set("d", d)
                    .set("fill", "none")
                    .set("stroke", c)
                    .set("stroke-width", 0.5)
                    .set("opacity", 0.3),
            );
        }
    }

    g
}
