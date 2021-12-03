fn main() {
    let input: Vec<_> = include_str!("../input")
        .lines()
        .map(|l| l.chars().map(|c| c == '1').collect::<Vec<_>>())
        .collect();

    // Part 1
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for i in 0..input[0].len() {
        let (zeros, ones) =
            input.iter().fold(
                (0, 0),
                |(o, z), line| {
                    if line[i] {
                        (o, z + 1)
                    } else {
                        (o + 1, z)
                    }
                },
            );
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
}
