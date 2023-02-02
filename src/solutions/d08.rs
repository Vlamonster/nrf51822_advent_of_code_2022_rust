use rtt_target::rprintln;

/// Measured speed: 174,629us.
pub fn p1(_memory: &mut [u8], input: &[u8]) {
    let width = input.iter().position(|&d| d == b'\n').unwrap();
    let height = input.iter().rposition(|&d| d == b'\n').unwrap() / width;

    let mut visible = 0;
    for x in 0..width {
        for y in 0..height {
            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                visible += 1;
                continue;
            }
            let tree = input[y * (width + 1) + x];
            if (0..x).rev().all(|nx| input[y * (width + 1) + nx] < tree) {
                visible += 1;
                continue;
            }
            if (x + 1..width).all(|nx| input[y * (width + 1) + nx] < tree) {
                visible += 1;
                continue;
            }
            if (0..y).rev().all(|ny| input[ny * (width + 1) + x] < tree) {
                visible += 1;
                continue;
            }
            if (y + 1..height).all(|ny| input[ny * (width + 1) + x] < tree) {
                visible += 1;
                continue;
            }
        }
    }

    rprintln!("d08a: {}", visible);
}

/// Measured speed: 166,544us.
pub fn p2(_memory: &mut [u8], input: &[u8]) {
    let width = input.iter().position(|&d| d == b'\n').unwrap();
    let height = input.iter().rposition(|&d| d == b'\n').unwrap() / width;

    let mut best_score = 0;
    for x in 1..width - 1 {
        for y in 1..height - 1 {
            let tree = input[y * (width + 1) + x];
            let mut left = x;
            let mut right = width - 1 - x;
            let mut top = y;
            let mut bottom = height - 1 - y;

            for (pos, nx) in (0..x).rev().enumerate() {
                if input[y * (width + 1) + nx] >= tree {
                    left = pos + 1;
                    break;
                }
            }
            for (pos, nx) in (x + 1..width).enumerate() {
                if input[y * (width + 1) + nx] >= tree {
                    right = pos + 1;
                    break;
                }
            }
            for (pos, ny) in (0..y).rev().enumerate() {
                if input[ny * (width + 1) + x] >= tree {
                    top = pos + 1;
                    break;
                }
            }
            for (pos, ny) in (y + 1..height).enumerate() {
                if input[ny * (width + 1) + x] >= tree {
                    bottom = pos + 1;
                    break;
                }
            }

            best_score = best_score.max(left * right * top * bottom);
        }
    }

    rprintln!("d08b: {}", best_score);
}
