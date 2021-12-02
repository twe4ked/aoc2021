// https://adventofcode.com/2021/day/1

struct Data {
    part_1: i32,
    part_1_last: i32,
    part_2: i32,
    part_2_last: i32,
}

impl Default for Data {
    fn default() -> Self {
        // The "last" variables starts at -1 because we shouldn't be counting the first "increase"
        Self {
            part_1: -1,
            part_1_last: 0,
            part_2: -1,
            part_2_last: 0,
        }
    }
}

fn main() {
    let input: Vec<_> = include_str!("../input")
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect();

    let result = (0..input.len())
        .into_iter()
        .fold(Data::default(), |mut acc, i| {
            // Part 1
            if input[i] > acc.part_1_last {
                acc.part_1 += 1;
            }
            acc.part_1_last = input[i];

            // Part 2
            if i + 2 >= input.len() {
                return acc;
            }
            let sum = input[i] + input[i + 1] + input[i + 2];
            if sum > acc.part_2_last {
                acc.part_2 += 1;
            }
            acc.part_2_last = sum;

            acc
        });

    println!("Part 1: {}", result.part_1);
    println!("Part 2: {}", result.part_2);

    assert_eq!(1228, result.part_1);
    assert_eq!(1257, result.part_2);
}
