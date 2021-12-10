fn main() {
    let input: Vec<_> = include_str!("../input")
        .trim()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let max = *input.iter().max().unwrap();

    let mut part_1 = i32::MAX;
    let mut part_2 = i32::MAX;

    for i in 0..max {
        let mut fuel_1 = 0;
        let mut fuel_2 = 0;

        for position in &input {
            let distance = (*position - i).abs();

            fuel_1 += distance;

            // https://en.wikipedia.org/wiki/Triangular_number
            fuel_2 += distance * (distance + 1) / 2;
        }

        part_1 = part_1.min(fuel_1);
        part_2 = part_2.min(fuel_2);
    }

    println!("Part 1: {}", part_1);
    assert_eq!(336131, part_1);

    println!("Part 2: {}", part_2);
    assert_eq!(92676646, part_2);
}
