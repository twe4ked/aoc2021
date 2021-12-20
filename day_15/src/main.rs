use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    let graph = Graph::new(input);

    let part_1 = graph.lowest_risk_path();
    println!("Part 1: {}", part_1);
    assert_eq!(720, part_1);

    let part_2 = graph.expand().lowest_risk_path();
    println!("Part 2: {}", part_2);
    assert_eq!(3025, part_2);
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    risk: u32,
    position: (usize, usize),
}

// The priority queue depends on `Ord`.
//
// Explicitly implement the trait so the queue becomes a min-heap instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        //
        // In case of a tie we compare positions - this step is necessary to make implementations
        // of `PartialEq` and `Ord` consistent.
        other
            .risk
            .cmp(&self.risk)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Graph {
    graph: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl Graph {
    fn new(input: &str) -> Self {
        let graph = input
            .split_whitespace()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let width = graph[0].len();
        let height = graph.len();
        Self {
            graph,
            width,
            height,
        }
    }

    // https://www.geeksforgeeks.org/dijkstras-shortest-path-algorithm-greedy-algo-7/
    // https://doc.rust-lang.org/std/collections/binary_heap/index.html#examples
    fn lowest_risk_path(&self) -> u32 {
        let mut distances: HashMap<(usize, usize), u32> = HashMap::new();
        for y in 0..self.height {
            for x in 0..self.width {
                distances.insert((x, y), u32::MAX);
            }
        }

        let start = (0, 0);
        distances.insert(start, 0);

        let mut heap = BinaryHeap::new();
        heap.push(State {
            risk: 0,
            position: start,
        });

        while let Some(State { risk, position }) = heap.pop() {
            if risk > distances[&position] {
                continue;
            }

            // Update distances for adjacent vertexes
            for v in self.adjacent(position.0, position.1) {
                let next = State {
                    risk: risk + self.graph[v.1][v.0],
                    position: v,
                };
                if next.risk < distances[&next.position] {
                    heap.push(next);

                    // We have now found a better way
                    distances.insert(next.position, next.risk);
                }
            }
        }

        *distances.get(&(self.height - 1, self.width - 1)).unwrap()
    }

    fn adjacent(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        [(-1, 0), (0, -1), (0, 1), (1, 0)]
            .iter()
            .filter_map(move |(y_offset, x_offset)| {
                let y = y as isize + y_offset;
                let x = x as isize + x_offset;
                if y < 0 || y >= self.height as isize || x < 0 || x >= self.width as isize {
                    None
                } else {
                    Some((x as usize, y as usize))
                }
            })
    }

    fn expand(&self) -> Graph {
        let rollover = |n: u32| {
            if n > 9 {
                n - 9
            } else {
                n
            }
        };

        let width = self.width * 5;
        let height = self.height * 5;
        let mut graph = vec![vec![0; width]; height];

        // Expand right
        for i in 0..5 {
            #[allow(clippy::needless_range_loop)]
            for y in 0..self.height {
                for x in 0..self.width {
                    graph[y][x + i * self.width] = rollover(self.graph[y][x] + i as u32);
                }
            }
        }

        // Expand down
        for i in 0..5 {
            for y in 0..self.height {
                for x in 0..width {
                    graph[y + i * self.height][x] = rollover(graph[y][x] + i as u32);
                }
            }
        }

        Graph {
            graph,
            width,
            height,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "1163751742
                     1381373672
                     2136511328
                     3694931569
                     7463417111
                     1319128137
                     1359912421
                     3125421639
                     1293138521
                     2311944581";
        let graph = Graph::new(&input);

        let part_1 = graph.lowest_risk_path();
        assert_eq!(40, part_1);

        let graph = graph.expand();

        for y in 0..graph.height {
            for x in 0..graph.width {
                print!("{}", graph.graph[y][x]);
            }
            println!();
        }

        let part_2 = graph.lowest_risk_path();
        assert_eq!(315, part_2);
    }
}
