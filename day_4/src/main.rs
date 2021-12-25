use std::convert::TryInto;

#[derive(Debug)]
struct Number {
    value: u8,
    marked: bool,
}

impl Number {
    fn new(value: u8) -> Self {
        Number {
            value,
            marked: false,
        }
    }

    fn marked_value(&self) -> Option<usize> {
        if self.marked {
            None
        } else {
            Some(self.value as usize)
        }
    }
}

#[derive(Debug)]
struct Board {
    numbers: [Number; 25],
}

impl Board {
    fn new(numbers: [Number; 25]) -> Self {
        Self { numbers }
    }

    fn mark(&mut self, value: u8) {
        if let Some(i) = self.numbers.iter().position(|n| n.value == value) {
            self.numbers[i].marked = true;
        }
    }

    // 0 1 # # 4
    // 5 6 # # 9
    // # # # # #
    // # # # # #
    // # # # # #
    fn finished(&self) -> bool {
        for i in 0..5 {
            // Check rows
            if self.numbers.iter().skip(i * 5).take(5).all(|b| b.marked) {
                return true;
            }

            // Check cols
            if self.numbers.iter().skip(i).step_by(5).all(|b| b.marked) {
                return true;
            }
        }

        false
    }

    fn score(&self, winning_number: u8) -> usize {
        self.numbers
            .iter()
            .filter_map(|n| n.marked_value())
            .sum::<usize>()
            * winning_number as usize
    }
}

fn main() {
    let (numbers, boards_str) = include_str!("../input").split_once("\n\n").unwrap();

    let mut boards: Vec<_> = (0..)
        .step_by(25)
        .into_iter()
        .map_while(|i| {
            let b: Vec<_> = boards_str
                .split_whitespace()
                .by_ref()
                .skip(i)
                .take(25)
                .map(|n| Number::new(n.parse::<u8>().unwrap()))
                .collect();
            if b.is_empty() {
                None
            } else {
                Some(Board::new(b.try_into().unwrap()))
            }
        })
        .collect();

    let mut part_1 = None;
    let mut part_2 = None;

    for n in numbers.split(',').map(|n| n.parse::<u8>().unwrap()) {
        for board in boards.iter_mut().filter(|b| !b.finished()) {
            board.mark(n);

            if board.finished() {
                let score = board.score(n);
                if part_1.is_none() {
                    part_1 = Some(score);
                }
                part_2 = Some(score);
            }
        }
    }

    let part_1 = part_1.unwrap();
    let part_2 = part_2.unwrap();

    println!("Part 1: {}", part_1);
    assert_eq!(10374, part_1);

    println!("Part 2: {}", part_2);
    assert_eq!(24742, part_2);
}
