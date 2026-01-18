use std::io;

fn main() {
    let pts = io::stdin()
        .lines()
        .flatten()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect::<Vec<(u64, u64)>>();

    let silver: u64 = pts
        .iter()
        .enumerate()
        .flat_map(|(idx, p1)| pts.iter().take(idx).map(move |p2| area(*p1, *p2)))
        .max()
        .unwrap();

    let gold: u64 = 0;
    println!("silver: {silver}");
    println!("gold: {gold}");
}

fn area((x1, y1): (u64, u64), (x2, y2): (u64, u64)) -> u64 {
    (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1)
}
