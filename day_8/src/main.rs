use std::collections::HashSet as Set;
use std::convert::TryInto;
use std::str::Split;

//   0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....
//
//   5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg

fn main() {
    let entries: Vec<_> = include_str!("../input").lines().map(parse_line).collect();

    let part_1 = entries
        .iter()
        .map(|(_, outputs)| count_number_of_easy_digits(outputs))
        .sum();
    println!("Part 1: {}", part_1);
    assert_eq!(310, part_1);

    let mut part_2 = 0;
    for entry in entries {
        let (signal_patterns, outputs) = entry;
        let segments = Segments::new(signal_patterns);
        // Combine the digits into a single number
        let mut acc = 0;
        for output in outputs {
            acc *= 10;
            acc += segments.decode(&output);
        }
        part_2 += acc;
    }
    println!("Part 2: {}", part_2);
    assert_eq!(915941, part_2);
}

// Each entry consists of ten unique signal patterns, a | delimiter, and finally the four digit
// output value.
fn parse_line(input: &str) -> ([Set<char>; 10], [Set<char>; 4]) {
    let mut parts = input.split('|');

    fn to_set(split: &mut Split<char>) -> Vec<Set<char>> {
        split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|pattern| pattern.chars().collect::<Set<_>>())
            .collect()
    }

    (
        to_set(&mut parts).try_into().unwrap(),
        to_set(&mut parts).try_into().unwrap(),
    )
}

// Numbers with a unique number of segments:
//
// 1 => 2 segments
// 4 => 4 segments
// 7 => 3 segments
// 8 => 7 segments
fn count_number_of_easy_digits(outputs: &[Set<char>]) -> i32 {
    outputs
        .iter()
        .filter_map(|e| {
            let number_of_segments = e.len();
            match number_of_segments {
                2 => Some(()), // 1
                3 => Some(()), // 7
                4 => Some(()), // 4
                7 => Some(()), // 8
                _ => None,
            }
        })
        .count() as _
}

#[derive(Default, Debug)]
struct Segments {
    numbers: [Set<char>; 10],
}

impl Segments {
    fn new(signal_patterns: [Set<char>; 10]) -> Segments {
        let mut segments = Segments::default();

        // First set the numbers with a unique number of segments
        for entry in signal_patterns.clone() {
            let number_of_segments = entry.len();
            match number_of_segments {
                2 => segments.numbers[1] = entry,
                3 => segments.numbers[7] = entry,
                4 => segments.numbers[4] = entry,
                7 => segments.numbers[8] = entry,
                5 | 6 => { /* Non-unique */ }
                _ => unreachable!(),
            }
        }

        for entry in signal_patterns {
            let number_of_segments = entry.len();
            match number_of_segments {
                5 => {
                    // "3" has 2 intersections with "1"
                    if segments.numbers[1].intersection(&entry).count() == 2 {
                        segments.numbers[3] = entry;
                        continue;
                    }

                    // "5" has 3 intersections with "4"
                    if segments.numbers[4].intersection(&entry).count() == 3 {
                        segments.numbers[5] = entry;
                        continue;
                    }

                    // Otherwise we have "2"
                    segments.numbers[2] = entry;
                }
                6 => {
                    // "9" has 4 intersections with "4"
                    if segments.numbers[4].intersection(&entry).count() == 4 {
                        segments.numbers[9] = entry;
                        continue;
                    }

                    // "0" has 2 intersections with "1"
                    if segments.numbers[1].intersection(&entry).count() == 2 {
                        segments.numbers[0] = entry;
                        continue;
                    }

                    // Otherwise we have "6"
                    segments.numbers[6] = entry;
                }
                2 | 3 | 4 | 7 => { /* Already set */ }
                _ => unreachable!(),
            }
        }

        segments
    }

    fn decode(&self, input: &Set<char>) -> i32 {
        self.numbers.iter().position(|n| n == input).unwrap() as _
    }
}
