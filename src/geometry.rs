use noise::{NoiseFn, Perlin};

pub const WIDTH: f64 = 1000.0;
pub const HEIGHT: f64 = 1400.0;
pub const CX: f64 = 500.0;
pub const CY: f64 = 700.0;

/// Generate a cubic bezier path data string with optional noise perturbation
pub fn bezier_curve(points: &[(f64, f64)], noise_amt: f64, seed: u32) -> String {
    if points.len() < 2 {
        return String::new();
    }
    let perlin = Perlin::new(seed);
    let mut d = String::new();
    let (x0, y0) = apply_noise(points[0].0, points[0].1, noise_amt, &perlin, 0.0);
    d.push_str(&format!("M{:.1},{:.1}", x0, y0));

    let mut i = 1;
    while i + 2 < points.len() {
        let (x1, y1) = apply_noise(points[i].0, points[i].1, noise_amt, &perlin, i as f64);
        let (x2, y2) = apply_noise(
            points[i + 1].0,
            points[i + 1].1,
            noise_amt,
            &perlin,
            (i + 1) as f64,
        );
        let (x3, y3) = apply_noise(
            points[i + 2].0,
            points[i + 2].1,
            noise_amt,
            &perlin,
            (i + 2) as f64,
        );
        d.push_str(&format!(
            " C{:.1},{:.1} {:.1},{:.1} {:.1},{:.1}",
            x1, y1, x2, y2, x3, y3
        ));
        i += 3;
    }
    // remaining points as line-to
    while i < points.len() {
        let (x, y) = apply_noise(points[i].0, points[i].1, noise_amt, &perlin, i as f64);
        d.push_str(&format!(" L{:.1},{:.1}", x, y));
        i += 1;
    }
    d
}

fn apply_noise(x: f64, y: f64, amt: f64, perlin: &Perlin, t: f64) -> (f64, f64) {
    if amt < 0.001 {
        return (x, y);
    }
    let nx = perlin.get([x * 0.01, y * 0.01, t * 0.5]) * amt;
    let ny = perlin.get([x * 0.01 + 100.0, y * 0.01 + 100.0, t * 0.5]) * amt;
    (x + nx, y + ny)
}

// /// Mirror a path data string vertically around center_y
// pub fn mirror_y_path(path: &str, center_y: f64) -> String {
//     format!("{} {}", path, transform_path_flip_y(path, center_y))
// }

// fn transform_path_flip_y(_path: &str, cy: f64) -> String {
//     // Simple approach: wrap in a group with transform
//     // For path data, we return a transform attribute value
//     format!("translate(0,{}) scale(1,-1) translate(0,{})", cy, -cy)
// }

// /// Generate transform string for 180° rotation around center
// pub fn rotate_180() -> String {
//     format!("rotate(180,{},{})", CX, CY)
// }

/// Generate a spiral path
pub fn spiral(
    cx: f64,
    cy: f64,
    r_start: f64,
    r_end: f64,
    turns: f64,
    steps: usize,
) -> Vec<(f64, f64)> {
    let mut pts = Vec::with_capacity(steps);
    for i in 0..steps {
        let t = i as f64 / (steps - 1) as f64;
        let angle = t * turns * std::f64::consts::TAU;
        let r = r_start + (r_end - r_start) * t;
        pts.push((cx + angle.cos() * r, cy + angle.sin() * r));
    }
    pts
}

/// Distribute n points radially around center
pub fn radial_distribute(n: usize, cx: f64, cy: f64, radius: f64) -> Vec<(f64, f64)> {
    (0..n)
        .map(|i| {
            let angle = (i as f64 / n as f64) * std::f64::consts::TAU - std::f64::consts::FRAC_PI_2;
            (cx + angle.cos() * radius, cy + angle.sin() * radius)
        })
        .collect()
}

/// Perlin noise offset for a point
pub fn noise_offset(x: f64, y: f64, scale: f64, seed: u32) -> (f64, f64) {
    let perlin = Perlin::new(seed);
    let dx = perlin.get([x * 0.01, y * 0.01]) * scale;
    let dy = perlin.get([x * 0.01 + 50.0, y * 0.01 + 50.0]) * scale;
    (dx, dy)
}

/// Generate an organic path with jitter applied to control points
pub fn organic_path(points: &[(f64, f64)], jitter: f64, seed: u32) -> String {
    bezier_curve(points, jitter, seed)
}

/// Create an ellipse path data string
pub fn ellipse_path(cx: f64, cy: f64, rx: f64, ry: f64) -> String {
    format!(
        "M{:.1},{:.1} A{:.1},{:.1} 0 1,1 {:.1},{:.1} A{:.1},{:.1} 0 1,1 {:.1},{:.1} Z",
        cx - rx,
        cy,
        rx,
        ry,
        cx + rx,
        cy,
        rx,
        ry,
        cx - rx,
        cy
    )
}

/// Create a rounded rectangle path
pub fn rounded_rect(x: f64, y: f64, w: f64, h: f64, r: f64) -> String {
    format!(
        "M{:.1},{:.1} L{:.1},{:.1} Q{:.1},{:.1} {:.1},{:.1} L{:.1},{:.1} Q{:.1},{:.1} {:.1},{:.1} L{:.1},{:.1} Q{:.1},{:.1} {:.1},{:.1} L{:.1},{:.1} Q{:.1},{:.1} {:.1},{:.1} Z",
        x + r,
        y,
        x + w - r,
        y,
        x + w,
        y,
        x + w,
        y + r,
        x + w,
        y + h - r,
        x + w,
        y + h,
        x + w - r,
        y + h,
        x + r,
        y + h,
        x,
        y + h,
        x,
        y + h - r,
        x,
        y + r,
        x,
        y,
        x + r,
        y
    )
}
