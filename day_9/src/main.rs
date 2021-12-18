fn main() {
    let grid = Grid::new(include_str!("../input"));

    let part_1 = part_1(&grid);
    println!("Part 1: {}", part_1);
    assert_eq!(537, part_1);

    let part_2 = part_2(&grid);
    println!("Part 2: {}", part_2);
    assert_eq!(1142757, part_2);
}

struct Grid {
    grid: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|l| {
                l.trim()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let width = grid[0].len();
        let height = grid.len();
        Self {
            grid,
            width,
            height,
        }
    }

    fn adjacent(&self, x: usize, y: usize) -> [u8; 4] {
        let up = if y == 0 { 9 } else { self.grid[y - 1][x] };
        let down = if y >= (self.height - 1) {
            9
        } else {
            self.grid[y + 1][x]
        };
        let left = if x == 0 { 9 } else { self.grid[y][x - 1] };
        let right = if x >= (self.width - 1) {
            9
        } else {
            self.grid[y][x + 1]
        };
        [up, down, left, right]
    }

    // https://en.wikipedia.org/wiki/Flood_fill
    fn flood_fill(&self, visited: &mut Vec<Vec<bool>>, x: usize, y: usize) -> u32 {
        if self.grid[y][x] == 9 || visited[y][x] {
            return 0;
        }

        visited[y][x] = true;

        let mut n = 0;
        if x < (self.width - 1) {
            n += self.flood_fill(visited, x + 1, y)
        }
        if x > 0 {
            n += self.flood_fill(visited, x - 1, y)
        }
        if y > 0 {
            n += self.flood_fill(visited, x, y - 1)
        }
        if y < (self.height - 1) {
            n += self.flood_fill(visited, x, y + 1);
        }
        n + 1
    }
}

fn part_1(grid: &Grid) -> u32 {
    let mut lowest = Vec::new();
    for y in 0..grid.height {
        for x in 0..grid.width {
            let current = grid.grid[y][x];
            if grid.adjacent(x, y).iter().all(|n| n > &current) {
                lowest.push(current as u32 + 1);
            }
        }
    }
    lowest.iter().sum()
}

fn part_2(grid: &Grid) -> u32 {
    let mut basin_sizes = Vec::new();
    let mut visited = vec![vec![false; grid.width]; grid.height];
    for y in 0..grid.height {
        for x in 0..grid.width {
            basin_sizes.push(grid.flood_fill(&mut visited, x, y));
        }
    }
    basin_sizes.sort_unstable();
    basin_sizes.iter().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let grid = Grid::new(
            "2199943210
             3987894921
             9856789892
             8767896789
             9899965678"
                .trim(),
        );

        assert_eq!(15, part_1(&grid));
        assert_eq!(1134, part_2(&grid));
    }
}
