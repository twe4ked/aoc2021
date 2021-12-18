use std::{env, fmt, thread, time};

fn main() {
    let mut grid = Grid::new([
        [3, 1, 1, 3, 2, 8, 4, 8, 8, 6],
        [2, 8, 5, 1, 8, 7, 6, 1, 4, 4],
        [2, 7, 7, 4, 6, 6, 4, 4, 8, 4],
        [6, 7, 1, 5, 1, 1, 2, 5, 7, 8],
        [7, 1, 4, 6, 2, 7, 2, 1, 5, 3],
        [6, 2, 5, 6, 6, 5, 6, 3, 6, 7],
        [3, 1, 4, 8, 6, 6, 6, 2, 4, 5],
        [3, 8, 5, 7, 4, 4, 6, 5, 2, 8],
        [7, 3, 2, 2, 4, 2, 2, 8, 3, 3],
        [8, 1, 5, 2, 1, 7, 5, 1, 6, 8],
    ]);

    let mut part_1 = 0;
    for _ in 0..100 {
        part_1 += grid.step();
        print(&grid);
    }

    let mut part_2 = 100; // Already at 100 steps
    loop {
        part_2 += 1;
        grid.step();
        print(&grid);
        if grid.synchronized() {
            break;
        }
    }

    println!("Part 1: {}", part_1);
    assert_eq!(1705, part_1);

    println!("Part 2: {}", part_2);
    assert_eq!(265, part_2);
}

#[derive(PartialEq)]
struct Grid<const N: usize> {
    octopuses: [[u8; N]; N],
}

impl<const N: usize> fmt::Debug for Grid<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.octopuses {
            for o in row {
                write!(f, "{}", o)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<const N: usize> Grid<N> {
    fn new(octopuses: [[u8; N]; N]) -> Self {
        Self { octopuses }
    }

    fn step(&mut self) -> u32 {
        // First, the energy level of each octopus increases by 1.
        for y in 0..N {
            for x in 0..N {
                self.octopuses[y][x] += 1;
            }
        }

        let mut part_1 = 0;

        // Then, any octopus with an energy level greater than 9 flashes. This increases the energy
        // level of all adjacent octopuses by 1, including octopuses that are diagonally adjacent.
        // If this causes an octopus to have an energy level greater than 9, it also flashes. This
        // process continues as long as new octopuses keep having their energy level increased
        // beyond 9. (An octopus can only flash at most once per step.)
        loop {
            let mut flashed = false;
            for y in 0..N {
                for x in 0..N {
                    if let Some(flashes) = self.try_flash(x, y) {
                        part_1 += flashes;
                        if flashes > 1 {
                            flashed = true;
                        }
                    }
                }
            }
            if !flashed {
                break;
            }
        }

        // Finally, any octopus that flashed during this step has its energy level set to 0, as it
        // used all of its energy to flash.
        for y in 0..N {
            for x in 0..N {
                if self.octopuses[y][x] > 9 {
                    self.octopuses[y][x] = 0;
                }
            }
        }

        part_1
    }

    fn try_flash(&mut self, x: usize, y: usize) -> Option<u32> {
        if self.octopuses[y][x] <= 9 || self.octopuses[y][x] == u8::MAX {
            // Not ready to flash OR already flashed
            return None;
        }
        // Set value to MAX to mark it as flashed
        self.octopuses[y][x] = u8::MAX;

        let mut flashes = 0;
        for (x, y) in adjacent(x, y, N) {
            self.octopuses[y][x] = self.octopuses[y][x].saturating_add(1);
            if let Some(adjacent_flashes) = self.try_flash(x, y) {
                flashes += adjacent_flashes;
            }
        }
        Some(flashes + 1)
    }

    fn synchronized(&self) -> bool {
        self.octopuses.iter().flatten().all(|o| o == &0)
    }
}

fn adjacent(x: usize, y: usize, max: usize) -> impl Iterator<Item = (usize, usize)> {
    #[rustfmt::skip]
    const OFFSETS: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];

    OFFSETS.iter().filter_map(move |(y_offset, x_offset)| {
        let y = y as isize + y_offset;
        let x = x as isize + x_offset;
        if y < 0 || y >= max as isize || x < 0 || x >= max as isize {
            None
        } else {
            Some((x as usize, y as usize))
        }
    })
}

fn print<const N: usize>(grid: &Grid<N>) {
    if env::var("PRINT").is_ok() {
        thread::sleep(time::Duration::from_millis(100));
        print!("\x1b[2J"); // clear
        print!("\x1b[{};{}H", 1, 1); // move cursor
        print!("{:?}", &grid);
    }
}
