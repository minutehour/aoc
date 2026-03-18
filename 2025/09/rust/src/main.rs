use std::io;

#[derive(Debug)]
struct Floor {
    x_comp: Vec<usize>,
    y_comp: Vec<usize>,
}

impl Floor {
    fn new(pts: Vec<(usize, usize)>) -> (Self, Vec<(usize, usize)>) {
        let (mut xs, mut ys): (Vec<_>, Vec<_>) = pts.clone().into_iter().unzip();
        xs.sort_unstable();
        xs.dedup();
        ys.sort_unstable();
        ys.dedup();

        let pts = pts
            .into_iter()
            .map(|(x, y)| (xs.binary_search(&x).unwrap(), ys.binary_search(&y).unwrap()))
            .collect();

        (
            Self {
                x_comp: xs,
                y_comp: ys,
            },
            pts,
        )
    }

    fn uncomp(&self, (x, y): (usize, usize)) -> (usize, usize) {
        (self.x_comp[x], self.y_comp[y])
    }

    fn area(&self, (p1, p2): ((usize, usize), (usize, usize))) -> usize {
        let (x1, y1) = self.uncomp(p1);
        let (x2, y2) = self.uncomp(p2);
        (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1)
    }
}

fn main() {
    let pts = io::stdin()
        .lines()
        .flatten()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect::<Vec<(usize, usize)>>();

    let (floor, pts) = Floor::new(pts);

    let xmax = *pts.iter().map(|(x, _)| x).max().unwrap();
    let ymax = *pts.iter().map(|(_, y)| y).max().unwrap();

    // some padding so that flood fill works correctly
    let mut grid: Vec<Vec<Option<bool>>> = vec![vec![None; xmax + 3]; ymax + 3];

    // draw green/red tiles
    for pair in [vec![*pts.last().unwrap()], pts.clone()]
        .concat()
        .windows(2)
    {
        let (x1, y1) = pair[0];
        let (x2, y2) = pair[1];
        (x1.min(x2)..=x1.max(x2)).for_each(|x| {
            (y1.min(y2)..=(y1.max(y2))).for_each(|y| grid[y + 1][x + 1] = Some(true))
        });
    }

    // flood fill from top left
    let mut q = vec![(0, 0)];
    while let Some((x, y)) = q.pop() {
        grid[y][x] = Some(false);
        let ns = vec![
            (x.saturating_sub(1), y),
            ((x + 1).min(xmax + 2), y),
            (x, y.saturating_sub(1)),
            (x, (y + 1).min(ymax + 2)),
        ];
        q.extend(ns.into_iter().filter(|&(x, y)| grid[y][x].is_none()));
    }

    // replace inside cells with Some(true) and undo the padding
    let grid: Vec<Vec<bool>> = grid
        .into_iter()
        .skip(1)
        .map(|r| r.into_iter().skip(1).map(|c| c.unwrap_or(true)).collect())
        .collect();

    let mut rectangles = pts
        .iter()
        .enumerate()
        .flat_map(|(idx, p1)| pts.iter().take(idx).map(move |p2| (*p1, *p2)))
        .collect::<Vec<_>>();
    rectangles.sort_unstable_by_key(|&r| std::cmp::Reverse(floor.area(r)));

    let silver: usize = floor.area(*rectangles.first().unwrap());
    let gold: usize = floor.area(
        *rectangles
            .iter()
            .filter(|&&((x1, y1), (x2, y2))| {
                (x1.min(x2)..=x1.max(x2))
                    .flat_map(|x| (y1.min(y2)..=y1.max(y2)).map(move |y| (x, y)))
                    .all(|(x, y)| grid[y][x])
            })
            .next()
            .unwrap(),
    );
    println!("silver: {silver}");
    println!("gold: {gold}");
}
