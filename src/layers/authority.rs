use crate::card::{Card, Rank};
use crate::geometry::*;
use crate::palette::palette_for;
use svg::node::element::{Group, Path};

/// Generate authority layer for face cards only (10% visual weight)
pub fn generate(card: &Card) -> Group {
    let pal = palette_for(card);
    let seed = card.seed() as u32 + 3000;
    let rank = card.rank().unwrap_or(Rank::Jack);
    let mut g = Group::new().set("id", "authority").set("opacity", 0.6);

    match rank {
        Rank::Jack => g = g.add(open_spine(pal.secondary, pal.highlight, seed)),
        Rank::Queen => g = g.add(enclosing_shell(pal.secondary, pal.highlight, seed)),
        Rank::King => g = g.add(spire_axis(pal.secondary, pal.highlight, seed)),
        _ => {}
    }

    g
}

/// Jack: open spine — unfinished boundaries, potential (no center vertical line)
fn open_spine(color: &str, highlight: &str, seed: u32) -> Group {
    let mut g = Group::new();

    // Lateral ribs extending outward from center area (open-ended, no spine)
    for i in 0..6 {
        let y = 280.0 + i as f64 * 150.0;
        let w = 100.0 + (i as f64 * 0.8).sin().abs() * 80.0;
        // Left rib
        let pts_l = vec![(CX - 20.0, y), (CX - w * 0.5, y - 10.0), (CX - w, y + 5.0)];
        let d_l = organic_path(&pts_l, 4.0, seed + 10 + i);
        g = g.add(
            Path::new()
                .set("d", d_l)
                .set("fill", "none")
                .set("stroke", highlight)
                .set("stroke-width", 1.2)
                .set("opacity", 0.7),
        );
        // Right rib
        let pts_r = vec![(CX + 20.0, y), (CX + w * 0.5, y - 10.0), (CX + w, y + 5.0)];
        let d_r = organic_path(&pts_r, 4.0, seed + 20 + i);
        g = g.add(
            Path::new()
                .set("d", d_r)
                .set("fill", "none")
                .set("stroke", highlight)
                .set("stroke-width", 1.2)
                .set("opacity", 0.7),
        );
    }

    // Small disconnected nodes along the center axis (dots, not lines)
    for i in 0..5 {
        let y = 300.0 + i as f64 * 160.0;
        let (dx, dy) = noise_offset(CX, y, 5.0, seed + 30 + i as u32);
        let d = ellipse_path(CX + dx, y + dy, 4.0, 4.0);
        g = g.add(
            Path::new()
                .set("d", d)
                .set("fill", "none")
                .set("stroke", color)
                .set("stroke-width", 1.5)
                .set("opacity", 0.6),
        );
    }

    g
}

/// Queen: enclosing shell — protective, cyclical
fn enclosing_shell(color: &str, highlight: &str, seed: u32) -> Group {
    let mut g = Group::new();

    // Concentric organic ellipses
    for i in 0..5 {
        let rx = 180.0 + i as f64 * 40.0;
        let ry = 260.0 + i as f64 * 50.0;
        let d = ellipse_path(CX, CY, rx, ry);
        let opacity = 0.5 - i as f64 * 0.08;
        g = g.add(
            Path::new()
                .set("d", d)
                .set("fill", "none")
                .set("stroke", color)
                .set("stroke-width", 1.5)
                .set("opacity", opacity),
        );
    }

    // Internal circulation paths
    for i in 0..8 {
        let angle = (i as f64 / 8.0) * std::f64::consts::TAU;
        let r1 = 120.0;
        let r2 = 200.0;
        let x1 = CX + angle.cos() * r1;
        let y1 = CY + angle.sin() * r1 * 1.4;
        let next_angle = ((i + 1) as f64 / 8.0) * std::f64::consts::TAU;
        let x2 = CX + next_angle.cos() * r2;
        let y2 = CY + next_angle.sin() * r2 * 1.4;
        let pts = vec![
            (x1, y1),
            ((x1 + x2) / 2.0 + 20.0, (y1 + y2) / 2.0),
            (x2, y2),
        ];
        let d = organic_path(&pts, 6.0, seed + i);
        g = g.add(
            Path::new()
                .set("d", d)
                .set("fill", "none")
                .set("stroke", highlight)
                .set("stroke-width", 0.8)
                .set("opacity", 0.4),
        );
    }

    g
}

/// King: spire axis — authority expressed through internal geometry, no outer elements
fn spire_axis(color: &str, highlight: &str, seed: u32) -> Group {
    let mut g = Group::new();

    // Horizontal authority bars across the center region
    for i in 0..7 {
        let y = 300.0 + i as f64 * 120.0;
        let half_w = 80.0 + (3.0 - (i as f64 - 3.0).abs()) * 15.0;
        let pts = vec![(CX - half_w, y), (CX, y + 3.0), (CX + half_w, y)];
        let d = organic_path(&pts, 2.0, seed + 10 + i);
        g = g.add(
            Path::new()
                .set("d", d)
                .set("fill", "none")
                .set("stroke", color)
                .set("stroke-width", 1.2)
                .set("opacity", 0.5),
        );
    }

    // Diamond nodes at bar intersections — Klimt gold accents
    for i in 0..5 {
        let y = 360.0 + i as f64 * 120.0;
        let (dx, dy) = noise_offset(CX, y, 4.0, seed + 20 + i as u32);
        let size = 6.0;
        let d = format!(
            "M{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} Z",
            CX + dx, y + dy - size,
            CX + dx + size, y + dy,
            CX + dx, y + dy + size,
            CX + dx - size, y + dy,
        );
        g = g.add(
            Path::new()
                .set("d", d)
                .set("fill", "none")
                .set("stroke", highlight)
                .set("stroke-width", 1.2)
                .set("opacity", 0.6),
        );
    }

    g
}
