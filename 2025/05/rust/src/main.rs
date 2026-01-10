use std::io;

fn main() {
    let mut input = io::stdin().lines().flatten().map(|line| line);
    let ranges = input.by_ref().take_while(|s| !s.is_empty()).map(|s| {
        let (lo, hi) = s.split_once('-').unwrap();
        (
            lo.to_owned().parse().unwrap(),
            hi.to_owned().parse().unwrap(),
        )
    });

    let mut merged: Vec<(u64, u64)> = vec![];
    for (mut lo, mut hi) in ranges {
        let ldx = merged.partition_point(|&(_, max)| max < lo);
        let mut rdx = merged.partition_point(|&(_, max)| max < hi);

        let left = merged.get(ldx);
        let right = merged.get(rdx);

        // ldx = merged.len() -> left is none -> insert at end
        let Some(&(llo, lhi)) = left else {
            merged.push((lo, hi));
            continue;
        };

        if (llo..=lhi).contains(&lo) {
            lo = lo.min(llo); // include range start point
        }

        // rdx = merged.len() -> right is none -> merge with all
        let Some(&(rlo, rhi)) = right else {
            merged.drain(ldx..);
            merged.push((lo, hi));
            continue;
        };

        if (rlo..=rhi).contains(&hi) {
            hi = hi.max(rhi);
            rdx += 1;
        }

        merged.drain(ldx..rdx);
        merged.insert(ldx, (lo, hi));
    }

    let silver = input
        .map(|n| n.parse::<u64>().unwrap())
        .filter(|id| merged.iter().any(|&(lo, hi)| (lo..=hi).contains(id)))
        .count();
    let gold: u64 = merged.iter().map(|(lo, hi)| hi - lo + 1).sum();

    println!("silver: {silver}");
    println!("gold: {gold}");
}
