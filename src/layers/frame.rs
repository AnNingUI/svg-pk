use crate::card::{Card, Suit};
use crate::geometry::*;
use crate::palette::palette_for;
use svg::node::element::{Group, Path};

/// Generate Mucha-style Art Nouveau frame (20% visual weight)
pub fn generate(card: &Card) -> Group {
    let pal = palette_for(card);
    let seed = card.seed() as u32;
    let mut g = Group::new().set("id", "frame");

    // Background fill
    let bg = Path::new()
        .set("d", rounded_rect(0.0, 0.0, WIDTH, HEIGHT, 20.0))
        .set("fill", pal.background);
    g = g.add(bg);

    // Outer border - organic rounded rect
    let outer = Path::new()
        .set(
            "d",
            rounded_rect(15.0, 15.0, WIDTH - 30.0, HEIGHT - 30.0, 30.0),
        )
        .set("fill", "none")
        .set("stroke", pal.secondary)
        .set("stroke-width", 3);
    g = g.add(outer);

    // Inner border
    let inner = Path::new()
        .set(
            "d",
            rounded_rect(30.0, 30.0, WIDTH - 60.0, HEIGHT - 60.0, 25.0),
        )
        .set("fill", "none")
        .set("stroke", pal.primary)
        .set("stroke-width", 2)
        .set("opacity", 0.6);
    g = g.add(inner);

    // Corner tendrils - spiral bezier terminals at four corners
    let corners = [(50.0, 50.0), (950.0, 50.0), (50.0, 1350.0), (950.0, 1350.0)];
    for (i, &(cx, cy)) in corners.iter().enumerate() {
        let spiral_pts = spiral(cx, cy, 5.0, 35.0, 1.5, 20);
        let d = organic_path(&spiral_pts, 3.0, seed + i as u32);
        let tendril = Path::new()
            .set("d", d)
            .set("fill", "none")
            .set("stroke", pal.highlight)
            .set("stroke-width", 1.5)
            .set("opacity", 0.8);
        g = g.add(tendril);
    }

    // Top arch decoration
    g = g.add(arch_decoration(true, pal.secondary, pal.highlight, seed));
    // Bottom arch decoration (mirrored)
    g = g.add(arch_decoration(
        false,
        pal.secondary,
        pal.highlight,
        seed + 10,
    ));

    // Side flowing curves connecting top and bottom
    g = g.add(side_curves(true, pal.primary, pal.secondary, seed + 20));
    g = g.add(side_curves(false, pal.primary, pal.secondary, seed + 30));

    // Suit-specific texture on frame
    g = g.add(frame_texture(card, seed + 40));

    g
}

fn arch_decoration(top: bool, color1: &str, color2: &str, seed: u32) -> Group {
    let mut g = Group::new();
    let y_base = if top { 25.0 } else { HEIGHT - 25.0 };
    let y_peak = if top { 60.0 } else { HEIGHT - 60.0 };
    let y_inner = if top { 45.0 } else { HEIGHT - 45.0 };

    // Main arch curve
    let pts = vec![
        (150.0, y_base),
        (250.0, y_peak),
        (400.0, y_inner),
        (500.0, y_peak + if top { 10.0 } else { -10.0 }),
        (600.0, y_inner),
        (750.0, y_peak),
        (850.0, y_base),
    ];
    let d = organic_path(&pts, 2.0, seed);
    let arch = Path::new()
        .set("d", d)
        .set("fill", "none")
        .set("stroke", color1)
        .set("stroke-width", 2.0);
    g = g.add(arch);

    // Inner decorative line
    let pts2 = vec![
        (200.0, y_base),
        (300.0, y_inner),
        (500.0, y_peak),
        (700.0, y_inner),
        (800.0, y_base),
    ];
    let d2 = organic_path(&pts2, 1.5, seed + 1);
    let inner_arch = Path::new()
        .set("d", d2)
        .set("fill", "none")
        .set("stroke", color2)
        .set("stroke-width", 1.0)
        .set("opacity", 0.6);
    g = g.add(inner_arch);

    g
}

fn side_curves(left: bool, color1: &str, color2: &str, seed: u32) -> Group {
    let mut g = Group::new();
    let x = if left { 25.0 } else { WIDTH - 25.0 };
    let x_wave = if left { 45.0 } else { WIDTH - 45.0 };

    let pts = vec![
        (x, 150.0),
        (x_wave, 350.0),
        (x, 500.0),
        (x_wave, 700.0),
        (x, 900.0),
        (x_wave, 1050.0),
        (x, 1250.0),
    ];
    let d = organic_path(&pts, 3.0, seed);
    let curve = Path::new()
        .set("d", d)
        .set("fill", "none")
        .set("stroke", color1)
        .set("stroke-width", 1.5)
        .set("opacity", 0.7);
    g = g.add(curve);

    // Parallel inner curve
    let x2 = if left { 35.0 } else { WIDTH - 35.0 };
    let x2_wave = if left { 50.0 } else { WIDTH - 50.0 };
    let pts2 = vec![
        (x2, 200.0),
        (x2_wave, 400.0),
        (x2, 550.0),
        (x2_wave, 700.0),
        (x2, 850.0),
        (x2_wave, 1000.0),
        (x2, 1200.0),
    ];
    let d2 = organic_path(&pts2, 2.0, seed + 1);
    let curve2 = Path::new()
        .set("d", d2)
        .set("fill", "none")
        .set("stroke", color2)
        .set("stroke-width", 1.0)
        .set("opacity", 0.4);
    g = g.add(curve2);

    g
}

fn frame_texture(card: &Card, seed: u32) -> Group {
    let mut g = Group::new().set("opacity", 0.3);
    let pal = palette_for(card);
    let suit = card.suit().unwrap_or(Suit::Spade);

    match suit {
        Suit::Spade => {
            // Bone texture: parallel rib lines along frame edges
            for i in 0..8 {
                let y = 80.0 + i as f64 * 160.0;
                let pts = vec![(40.0, y), (60.0, y + 5.0), (40.0, y + 10.0)];
                let d = organic_path(&pts, 1.0, seed + i);
                g = g.add(
                    Path::new()
                        .set("d", d)
                        .set("fill", "none")
                        .set("stroke", pal.secondary)
                        .set("stroke-width", 0.8),
                );
                // Mirror on right side
                let pts_r = vec![(960.0, y), (940.0, y + 5.0), (960.0, y + 10.0)];
                let d_r = organic_path(&pts_r, 1.0, seed + i + 100);
                g = g.add(
                    Path::new()
                        .set("d", d_r)
                        .set("fill", "none")
                        .set("stroke", pal.secondary)
                        .set("stroke-width", 0.8),
                );
            }
        }
        Suit::Heart => {
            // Vein texture: flowing curves along frame
            for i in 0..6 {
                let y = 100.0 + i as f64 * 200.0;
                let pts = vec![
                    (35.0, y),
                    (50.0, y + 40.0),
                    (35.0, y + 80.0),
                    (50.0, y + 120.0),
                ];
                let d = organic_path(&pts, 4.0, seed + i);
                g = g.add(
                    Path::new()
                        .set("d", d)
                        .set("fill", "none")
                        .set("stroke", pal.secondary)
                        .set("stroke-width", 0.6),
                );
            }
        }
        Suit::Club => {
            // Branch texture: small branching nodes
            for i in 0..10 {
                let y = 60.0 + i as f64 * 130.0;
                let pts = vec![(42.0, y), (55.0, y - 8.0)];
                let d = organic_path(&pts, 1.5, seed + i);
                g = g.add(
                    Path::new()
                        .set("d", d)
                        .set("fill", "none")
                        .set("stroke", pal.secondary)
                        .set("stroke-width", 0.7),
                );
                let pts2 = vec![(42.0, y), (55.0, y + 8.0)];
                let d2 = organic_path(&pts2, 1.5, seed + i + 50);
                g = g.add(
                    Path::new()
                        .set("d", d2)
                        .set("fill", "none")
                        .set("stroke", pal.secondary)
                        .set("stroke-width", 0.7),
                );
            }
        }
        Suit::Diamond => {
            // Crystal texture: small angular facets
            for i in 0..8 {
                let y = 80.0 + i as f64 * 160.0;
                let d = format!("M40,{} L50,{} L40,{} Z", y, y + 8.0, y + 16.0);
                g = g.add(
                    Path::new()
                        .set("d", d)
                        .set("fill", "none")
                        .set("stroke", pal.secondary)
                        .set("stroke-width", 0.6),
                );
                let d2 = format!("M960,{} L950,{} L960,{} Z", y, y + 8.0, y + 16.0);
                g = g.add(
                    Path::new()
                        .set("d", d2)
                        .set("fill", "none")
                        .set("stroke", pal.secondary)
                        .set("stroke-width", 0.6),
                );
            }
        }
    }

    g
}
