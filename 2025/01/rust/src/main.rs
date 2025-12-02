use num::BigUint;
use std::io;

fn main() {
    let mut curr = 50;
    let mut silver: u64 = 0;
    let mut gold: BigUint = BigUint::ZERO;
    io::stdin()
        .lines()
        .flatten()
        .map(|line| match line.split_at(1) {
            ("L", n) => -n.parse::<i64>().unwrap(),
            (_, n) => n.parse::<i64>().unwrap(),
        })
        .for_each(|n| {
            let prev = curr;
            curr += n;
            gold += (curr.abs() / 100) as u64 + (prev != 0 && curr <= 0) as u64;
            curr = curr.rem_euclid(100);
            silver += (curr == 0) as u64;
        });

    println!("silver: {silver}");
    println!("gold: {gold}");
}
