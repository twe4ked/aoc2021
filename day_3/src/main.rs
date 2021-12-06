fn count_zeros_and_ones(input: &[Vec<bool>], i: usize) -> (usize, usize) {
    input.iter().fold(
        (0, 0),
        |(o, z), line| {
            if line[i] {
                (o, z + 1)
            } else {
                (o + 1, z)
            }
        },
    )
}

fn main() {
    let input: Vec<_> = include_str!("../input")
        .lines()
        .map(|l| l.chars().map(|c| c == '1').collect::<Vec<_>>())
        .collect();
    let bit_count = input[0].len();

    // Part 1
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for i in 0..bit_count {
        let (zeros, ones) = count_zeros_and_ones(&input, i);
        if zeros > ones {
            gamma_rate = gamma_rate << 1;
            epsilon_rate = (epsilon_rate << 1) | 1;
        } else {
            gamma_rate = (gamma_rate << 1) | 1;
            epsilon_rate = epsilon_rate << 1;
        }
    }
    let part_1 = gamma_rate * epsilon_rate;

    println!("Part 1: {}", part_1);
    assert_eq!(2640986, part_1);

    // Part 2

    // To find oxygen generator rating, determine the most common value (0 or 1) in the current bit
    // position, and keep only numbers with that bit in that position. If 0 and 1 are equally
    // common, keep values with a 1 in the position being considered.
    let mut oxygen_generator_rating = input.clone();
    for i in 0..bit_count {
        let (zeros, ones) = count_zeros_and_ones(&oxygen_generator_rating, i);
        if zeros > ones {
            oxygen_generator_rating.retain(|l| !l[i]);
        } else {
            oxygen_generator_rating.retain(|l| l[i]);
        }
        if oxygen_generator_rating.len() == 1 {
            break;
        }
    }
    assert!(oxygen_generator_rating.len() == 1);
    let oxygen_generator_rating = oxygen_generator_rating[0]
        .iter()
        .fold(0, |acc, b| (acc << 1) | if *b { 1 } else { 0 });

    // To find CO2 scrubber rating, determine the least common value (0 or 1) in the current bit
    // position, and keep only numbers with that bit in that position. If 0 and 1 are equally
    // common, keep values with a 0 in the position being considered.
    let mut c02_scrubber_rating = input.clone();
    for i in 0..bit_count {
        let (zeros, ones) = count_zeros_and_ones(&c02_scrubber_rating, i);
        if zeros > ones {
            c02_scrubber_rating.retain(|l| l[i]);
        } else {
            c02_scrubber_rating.retain(|l| !l[i]);
        }
        if c02_scrubber_rating.len() == 1 {
            break;
        }
    }
    assert!(c02_scrubber_rating.len() == 1);
    let c02_scrubber_rating = c02_scrubber_rating[0]
        .iter()
        .fold(0, |acc, b| (acc << 1) | if *b { 1 } else { 0 });

    let part_2 = oxygen_generator_rating * c02_scrubber_rating;

    println!("Part 2: {}", part_2);
    assert_eq!(6822109, part_2);
}
