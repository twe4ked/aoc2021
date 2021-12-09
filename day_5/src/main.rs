use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

fn main() {
    let hydrothermal_vents_positions = parse(include_str!("../input"));

    let mut map_1 = HashMap::new();
    let mut map_2 = HashMap::new();

    for (p1, p2) in hydrothermal_vents_positions {
        line(&mut map_1, false, p1, p2);
        line(&mut map_2, true, p1, p2);
    }

    let part_1 = map_1.into_iter().filter(|(_k, v)| *v >= 2).count();
    let part_2 = map_2.into_iter().filter(|(_k, v)| *v >= 2).count();

    println!("Part 1: {}", part_1);
    assert_eq!(7436, part_1);

    println!("Part 2: {}", part_2);
    assert_eq!(21104, part_2);
}

fn parse(input: &str) -> Vec<(Point, Point)> {
    input
        .lines()
        .map(|line| {
            // x1,y1 -> x2,y2
            let mut parts = line.split(" -> ");
            let mut x1y1 = parts.next().unwrap().split(',');
            let mut x2y2 = parts.next().unwrap().split(',');
            (
                Point::new(
                    x1y1.next().unwrap().parse().unwrap(),
                    x1y1.next().unwrap().parse().unwrap(),
                ),
                Point::new(
                    x2y2.next().unwrap().parse().unwrap(),
                    x2y2.next().unwrap().parse().unwrap(),
                ),
            )
        })
        .collect()
}

fn line(map: &mut HashMap<Point, u16>, draw_diagonals: bool, p1: Point, p2: Point) {
    if p1.x == p2.x {
        for y in p1.y.min(p2.y)..=p1.y.max(p2.y) {
            *map.entry(Point::new(p1.x, y)).or_insert(0) += 1;
        }
    } else if p1.y == p2.y {
        for x in p1.x.min(p2.x)..=p1.x.max(p2.x) {
            *map.entry(Point::new(x, p1.y)).or_insert(0) += 1;
        }
    } else if draw_diagonals {
        let mut x = p1.x;
        let mut y = p1.y;

        loop {
            *map.entry(Point::new(x, y)).or_insert(0) += 1;

            // Increment or decrement x and y to move towards the other point until we reach the
            // other point (when they're equal).

            match x.cmp(&p2.x) {
                Ordering::Greater => x -= 1,
                Ordering::Less => x += 1,
                Ordering::Equal => break,
            }

            match y.cmp(&p2.y) {
                Ordering::Greater => y -= 1,
                Ordering::Less => y += 1,
                Ordering::Equal => break,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let example = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        let hydrothermal_vents_positions = parse(example);

        let mut map_1 = HashMap::new();
        let mut map_2 = HashMap::new();

        for (p1, p2) in hydrothermal_vents_positions {
            line(&mut map_1, false, p1, p2);
            line(&mut map_2, true, p1, p2);
        }

        print_map(&map_1);
        println!("----");
        print_map(&map_2);
        println!("----");

        let part_1 = map_1.into_iter().filter(|(_k, v)| *v >= 2).count();
        let part_2 = map_2.into_iter().filter(|(_k, v)| *v >= 2).count();

        assert_eq!(5, part_1);
        assert_eq!(12, part_2);
    }

    fn print_map(map: &HashMap<Point, u16>) {
        for y in 0..=9 {
            for x in 0..=9 {
                if let Some(n) = map.get(&Point::new(x, y)) {
                    print!("{}", n);
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}
