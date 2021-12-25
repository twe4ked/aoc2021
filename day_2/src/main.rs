#[derive(Debug)]
enum Direction {
    Up(i32),
    Down(i32),
    Forward(i32),
}

fn main() {
    let input: Vec<_> = include_str!("../input")
        .lines()
        .map(|l| {
            let (dir, num) = l.split_once(' ').unwrap();
            let num = num.parse::<i32>().unwrap();
            match dir {
                "up" => Direction::Up(num),
                "down" => Direction::Down(num),
                "forward" => Direction::Forward(num),
                _ => panic!(),
            }
        })
        .collect();

    let (x, y) = input.iter().fold((0, 0), |(x, y), d| match d {
        Direction::Up(n) => (x, y - n),
        Direction::Down(n) => (x, y + n),
        Direction::Forward(n) => (x + n, y),
    });
    let part_1 = x * y;

    println!("Part 1: {}", part_1);
    assert_eq!(2147104, part_1);

    let (_a, x, y) = input.iter().fold((0, 0, 0), |(a, x, y), d| match d {
        Direction::Up(n) => (a - n, x, y),
        Direction::Down(n) => (a + n, x, y),
        Direction::Forward(n) => (a, x + n, y + a * n),
    });
    let part_2 = x * y;

    println!("Part 2: {}", part_2);
    assert_eq!(2044620088, part_2);
}
