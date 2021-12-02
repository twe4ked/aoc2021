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
            let mut parts = l.split(' ');
            let dir = parts.next().unwrap();
            let num = parts.next().unwrap().parse::<i32>().unwrap();
            match dir {
                "up" => Direction::Up(num),
                "down" => Direction::Down(num),
                "forward" => Direction::Forward(num),
                _ => panic!(),
            }
        })
        .collect();

    let mut x = 0;
    let mut y = 0;
    for direction in &input {
        match direction {
            Direction::Up(n) => y -= n,
            Direction::Down(n) => y += n,
            Direction::Forward(n) => x += n,
        }
    }
    let part_1 = x * y;

    println!("Part 1: {}", part_1);
    assert_eq!(2147104, part_1);

    let mut a = 0;
    let mut x = 0;
    let mut y = 0;
    for direction in input {
        match direction {
            Direction::Up(n) => a -= n,
            Direction::Down(n) => a += n,
            Direction::Forward(n) => {
                x += n;
                y += a * n;
            }
        }
    }
    let part_2 = x * y;

    println!("Part 2: {}", part_2);
    assert_eq!(2044620088, part_2);
}
