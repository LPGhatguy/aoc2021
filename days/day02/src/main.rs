static INPUT: &str = include_str!("../input.txt");

fn part1() {
    let pos = INPUT.lines().fold((0, 0), |(x, y), line| {
        let mut parts = line.split(' ');
        let dir = parts.next().unwrap();
        let len = parts.next().unwrap().parse::<i32>().unwrap();

        match dir {
            "forward" => (x + len, y),
            "down" => (x, y + len),
            "up" => (x, y - len),
            _ => unreachable!(),
        }
    });

    println!("Part 1: {}", pos.0 * pos.1)
}

fn part2() {
    let pos = INPUT.lines().fold((0, 0, 0), |(x, y, aim), line| {
        let mut parts = line.split(' ');
        let dir = parts.next().unwrap();
        let len = parts.next().unwrap().parse::<i32>().unwrap();

        match dir {
            "forward" => (x + len, y + (len * aim), aim),
            "down" => (x, y, aim + len),
            "up" => (x, y, aim - len),
            _ => unreachable!(),
        }
    });

    println!("Part 2: {}", pos.0 * pos.1)
}

fn main() {
    part1();
    part2();
}
