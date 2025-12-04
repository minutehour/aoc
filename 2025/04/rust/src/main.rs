use std::io;

fn main() {
    let mut grid: Vec<Vec<Option<usize>>> = io::stdin()
        .lines()
        .flatten()
        .map(|line| line.chars().map(|ch| (ch == '@').then_some(0)).collect())
        .collect();

    for (y, row) in grid.iter().enumerate() {
        for (x, p) in row.iter().enumerate() {
            let Some(_) = p else {
                continue;
            };

            for nx in x.saturating_sub(1)..=(x + 1).min(row.len() - 1) {
                for ny in y.saturating_sub(1)..=(y + 1).min(grid.len() - 1) {
                    // lol quadruple for loop (this makes me sad)
                    if (x, y) == (nx, ny) {
                        continue;
                    }
                    let mut h = grid.get(ny).and_then(|r| r.get(nx)).unwrap();
                }
            }
        }
    }

    let silver: u64 = 0;
    let gold: u64 = 0;
    println!("silver: {silver}");
    println!("gold: {gold}");
}
