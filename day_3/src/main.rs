fn count_zeros_and_ones(input: &[u16], i: usize) -> (usize, usize) {
    input.iter().fold((0, 0), |(z, o), n| {
        if n >> i & 1 == 0 {
            (z + 1, o)
        } else {
            (z, o + 1)
        }
    })
}

fn main() {
    let input: Vec<_> = include_str!("../input")
        .lines()
        .map(|n| u16::from_str_radix(n, 2).unwrap())
        .collect();
    let bit_count = 12; // Binary numbers in `input` are 12bits wide

    // Part 1
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for i in (0..bit_count).into_iter().rev() {
        let (zeros, ones) = count_zeros_and_ones(&input, i);
        if zeros > ones {
            gamma_rate <<= 1;
            epsilon_rate = (epsilon_rate << 1) | 1;
        } else {
            gamma_rate = (gamma_rate << 1) | 1;
            epsilon_rate <<= 1;
        }
    }
    let part_1 = gamma_rate * epsilon_rate;

    println!("Part 1: {}", part_1);
    assert_eq!(2640986, part_1);

    // Part 2
    let mut oxygen_generator_rating = input.clone();
    let mut c02_scrubber_rating = input;
    for i in (0..bit_count).into_iter().rev() {
        // To find oxygen generator rating, determine the most common value (0 or 1) in the current
        // bit position, and keep only numbers with that bit in that position. If 0 and 1 are
        // equally common, keep values with a 1 in the position being considered.
        if oxygen_generator_rating.len() > 1 {
            let (zeros, ones) = count_zeros_and_ones(&oxygen_generator_rating, i);
            dbg!((zeros, ones));
            if zeros > ones {
                oxygen_generator_rating.retain(|n| n >> i & 1 == 0);
            } else {
                oxygen_generator_rating.retain(|n| n >> i & 1 == 1);
            }
        }

        // To find CO2 scrubber rating, determine the least common value (0 or 1) in the current
        // bit position, and keep only numbers with that bit in that position. If 0 and 1 are
        // equally common, keep values with a 0 in the position being considered.
        if c02_scrubber_rating.len() > 1 {
            let (zeros, ones) = count_zeros_and_ones(&c02_scrubber_rating, i);
            if zeros > ones {
                c02_scrubber_rating.retain(|n| n >> i & 1 == 1);
            } else {
                c02_scrubber_rating.retain(|n| n >> i & 1 == 0);
            }
        }
    }

    assert_eq!(1, oxygen_generator_rating.len());
    assert_eq!(1, c02_scrubber_rating.len());

    let oxygen_generator_rating = oxygen_generator_rating[0];
    let c02_scrubber_rating = c02_scrubber_rating[0];

    let part_2 = oxygen_generator_rating as u32 * c02_scrubber_rating as u32;

    println!("Part 2: {}", part_2);
    assert_eq!(6822109, part_2);
}
