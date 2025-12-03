use std::io;

fn max(bs: &Vec<u8>, start: usize, end: usize) -> (u8, usize) {
    let mut bests = [None; 10];
    for idx in start..=end {
        if bs[idx] == 9 {
            return (9, idx);
        }
        bests[bs[idx] as usize].get_or_insert((bs[idx], idx));
    }
    bests.into_iter().flatten().last().unwrap()
}

fn jolt(bs: &Vec<u8>, n: usize) -> u64 {
    let mut j: u64 = 0;
    let mut start = 0;
    for n in (1..=n).rev() {
        j *= 10;
        let (b, idx) = max(bs, start, bs.len() - n);
        j += b as u64;
        start = idx + 1;
    }
    j
}

fn main() {
    let (silver, gold) = io::stdin()
        .lines()
        .flatten()
        .map(|line| {
            line.into_bytes()
                .iter()
                .map(|&x| x - b'0')
                .collect::<Vec<u8>>()
        })
        .map(|bs| (jolt(&bs, 2), jolt(&bs, 12)))
        .reduce(|(a0, a1), (x0, x1)| (a0 + x0, a1 + x1))
        .unwrap();

    println!("silver: {silver}");
    println!("gold: {gold}");
}
