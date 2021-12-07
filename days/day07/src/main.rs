use std::convert::identity;

static INPUT: &str = include_str!("../input.txt");

fn solve(input: &[i32], costfn: impl Fn(i32) -> i32) -> i32 {
    let highest = input.iter().copied().max().unwrap();
    let mut best_cost = i32::MAX;

    for target in 0..highest {
        let mut cost = 0;

        for pos in input {
            cost += costfn((pos - target).abs());
        }

        best_cost = best_cost.min(cost);
    }

    best_cost
}

fn triangle(n: i32) -> i32 {
    (n.pow(2) + n) / 2
}

fn main() {
    let positions: Vec<_> = INPUT
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    println!("Part one: {}", solve(&positions, identity));
    println!("Part two: {}", solve(&positions, triangle));
}
