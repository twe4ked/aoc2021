use std::fmt::Write;

fn main() {
    let input = include_str!("../input");
    let (positions, fold_instructions) = input.split_once("\n\n").unwrap();

    let mut grid = Grid::new(positions);

    let fold_instructions: Vec<_> = fold_instructions
        .lines()
        .map(|line| {
            let mut parts = line.split('=');
            match parts.next().unwrap() {
                "fold along x" => FoldInstruction::X(parts.next().unwrap().parse().unwrap()),
                "fold along y" => FoldInstruction::Y(parts.next().unwrap().parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .collect();

    grid.fold(&fold_instructions[0]);

    let part_1 = grid.count_dots();
    println!("Part 1: {}", part_1);
    assert_eq!(610, part_1);

    for fold_instruction in &fold_instructions[1..] {
        grid.fold(fold_instruction);
    }

    println!("Part 2: PZFJHRFZ");
    let mut part_2 = String::new();
    for row in grid.grid {
        for d in row {
            if d {
                write!(part_2, "#").unwrap();
            } else {
                write!(part_2, ".").unwrap();
            }
        }
        writeln!(part_2).unwrap();
    }

    let part_2_expected = "###..####.####...##.#..#.###..####.####.
                           #..#....#.#.......#.#..#.#..#.#.......#.
                           #..#...#..###.....#.####.#..#.###....#..
                           ###...#...#.......#.#..#.###..#.....#...
                           #....#....#....#..#.#..#.#.#..#....#....
                           #....####.#.....##..#..#.#..#.#....####."
        .lines()
        .map(|line| line.trim_start())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(&part_2_expected, &part_2.trim());
    print!("\n{}", part_2);
}

enum FoldInstruction {
    X(usize),
    Y(usize),
}

struct Grid {
    grid: Vec<Vec<bool>>,
}

impl Grid {
    fn new(positions: &str) -> Self {
        let positions: Vec<_> = positions
            .split_whitespace()
            .map(|line| {
                let mut parts = line.split(',');
                (
                    parts.next().unwrap().parse::<usize>().unwrap(),
                    parts.next().unwrap().parse::<usize>().unwrap(),
                )
            })
            .collect();
        let max_x = *positions.iter().map(|(x, _)| x).max().unwrap() + 1;
        let max_y = *positions.iter().map(|(_, y)| y).max().unwrap() + 1;
        let mut grid = vec![vec![false; max_x]; max_y];
        for (x, y) in positions {
            grid[y][x] = true;
        }
        Self { grid }
    }

    fn fold(&mut self, fold_instruction: &FoldInstruction) {
        match fold_instruction {
            FoldInstruction::X(n) => {
                for row in self.grid.iter_mut() {
                    let end = &row.split_off(*n)[1..];
                    for (x, d) in end.iter().rev().enumerate() {
                        if !row[x] {
                            row[x] = *d;
                        }
                    }
                }
            }
            FoldInstruction::Y(n) => {
                let bottom = &self.grid.split_off(*n)[1..];
                for (y, row) in bottom.iter().rev().enumerate() {
                    for (x, d) in row.iter().enumerate() {
                        if !self.grid[y][x] {
                            self.grid[y][x] = *d;
                        }
                    }
                }
            }
        }
    }

    fn count_dots(&self) -> usize {
        self.grid.iter().flatten().filter(|d| **d).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let example_1_positions = "6,10 0,14 9,10 0,3 10,4 4,11 6,0 6,12 4,1
                                   0,13 10,12 3,4 3,0 8,4 1,10 2,14 8,10 9,0";
        let _example_1_fold_instructions = "fold along y=7\nfold along x=5";
        let mut grid = Grid::new(example_1_positions);
        grid.fold(&FoldInstruction::Y(7));
        assert_eq!(17, grid.count_dots());
    }
}
