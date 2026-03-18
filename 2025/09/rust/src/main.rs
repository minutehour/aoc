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

    fn uncomp(self: &Self, (x, y): (usize, usize)) -> (usize, usize) {
        (self.x_comp[x], self.y_comp[y])
    }

    fn area(self: &Self, p1: (usize, usize), p2: (usize, usize)) -> usize {
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

    let silver: usize = pts
        .iter()
        .enumerate()
        .flat_map(|(idx, p1)| pts.iter().take(idx).map(|p2| floor.area(*p1, *p2)))
        .max()
        .unwrap();

    let gold: u64 = 0;
    println!("silver: {silver}");
    println!("gold: {gold}");
}
