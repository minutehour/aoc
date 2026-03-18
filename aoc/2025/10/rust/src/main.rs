use good_lp::*;
use itertools::Itertools;
use std::io;

fn main() {
    let ms = io::stdin()
        .lines()
        .flatten()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let p1 = iter
                .next()
                .unwrap()
                .chars()
                .filter_map(|ch| match ch {
                    '.' => Some(0),
                    '#' => Some(1),
                    _ => None,
                })
                .collect::<Vec<u32>>();

            let mut iter = iter.map(|s| {
                s.split(|ch: char| ch.is_ascii_punctuation())
                    .filter_map(|n| n.parse().ok())
                    .collect::<Vec<u32>>()
            });
            let p2 = iter.next_back().unwrap();
            let bs = iter.collect::<Vec<_>>();
            (p1, p2, bs)
        })
        .collect::<Vec<_>>();

    let silver: usize = ms.iter().map(|(want, _, bs)| solve_p1(&want, &bs)).sum();
    let gold: usize = ms.iter().map(|(_, want, bs)| solve_p2(&want, &bs)).sum();
    println!("silver: {silver}");
    println!("gold: {gold}");
}

fn solve_p1(want: &Vec<u32>, bs: &Vec<Vec<u32>>) -> usize {
    bs.into_iter()
        .powerset()
        .map(|sub| (sub.len(), sub.into_iter().flatten().counts()))
        .filter(|(_, counts)| {
            want.iter()
                .enumerate()
                .all(|(n, w)| counts.get(&(n as u32)).unwrap_or(&0) % 2 == *w as usize)
        })
        .next()
        .unwrap()
        .0
}

fn solve_p2(want: &Vec<u32>, bs: &Vec<Vec<u32>>) -> usize {
    variables! {vars: 0 <= xs[bs.len()] (integer);}
    let obj: Expression = xs.iter().sum();
    let mut model = vars.minimise(&obj).using(default_solver);
    for (n, w) in want.iter().enumerate() {
        let xsum: Expression = bs
            .iter()
            .enumerate()
            .filter(|&(_, b)| b.contains(&(n as u32)))
            .map(|(x, _)| xs[x])
            .sum();
        model = model.with(constraint!(xsum == *w));
    }

    model.solve().unwrap().eval(&obj).round() as usize
}
