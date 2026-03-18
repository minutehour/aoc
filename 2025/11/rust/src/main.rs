use std::{cell::RefCell, collections::HashMap, io, rc::Rc};

#[derive(PartialEq, Eq, Hash, Debug)]
struct Node {
    device: String,
    dac: bool,
    fft: bool,
}

fn main() {
    let adj = io::stdin()
        .lines()
        .flatten()
        .map(|line| {
            let (k, v) = line.split_once(": ").unwrap();
            (
                String::from(k),
                String::from(v)
                    .split_whitespace()
                    .map(String::from)
                    .collect(),
            )
        })
        .collect::<HashMap<String, Vec<String>>>();

    let cache = Rc::new(RefCell::new(HashMap::new()));

    let silver = paths_from(
        Node {
            device: String::from("you"),
            dac: true,
            fft: true,
        },
        &adj,
        Rc::clone(&cache),
    );
    let gold = paths_from(
        Node {
            device: String::from("svr"),
            dac: false,
            fft: false,
        },
        &adj,
        Rc::clone(&cache),
    );

    println!("silver: {silver}");
    println!("gold: {gold}");
}

fn paths_from(
    curr: Node,
    adj: &HashMap<String, Vec<String>>,
    cache: Rc<RefCell<HashMap<Node, usize>>>,
) -> usize {
    if curr.device == "out" {
        if curr.dac && curr.fft {
            return 1;
        }
        return 0;
    }

    let Some(&v) = cache.borrow().get(&curr) else {
        let paths = adj
            .get(&curr.device)
            .unwrap()
            .iter()
            .map(|n| {
                paths_from(
                    Node {
                        device: n.clone(),
                        dac: curr.dac || curr.device == "dac",
                        fft: curr.fft || curr.device == "fft",
                    },
                    adj,
                    Rc::clone(&cache),
                )
            })
            .sum();
        cache.borrow_mut().insert(curr, paths);
        return paths;
    };
    return v;
}
