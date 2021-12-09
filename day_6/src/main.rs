const INPUT: [usize; 300] = [
    2, 5, 2, 3, 5, 3, 5, 5, 4, 2, 1, 5, 5, 5, 5, 1, 2, 5, 1, 1, 1, 1, 1, 5, 5, 1, 5, 4, 3, 3, 1, 2,
    4, 2, 4, 5, 4, 5, 5, 5, 4, 4, 1, 3, 5, 1, 2, 2, 4, 2, 1, 1, 2, 1, 1, 4, 2, 1, 2, 1, 2, 1, 3, 3,
    3, 5, 1, 1, 1, 3, 4, 4, 1, 3, 1, 5, 5, 1, 5, 3, 1, 5, 2, 2, 2, 2, 1, 1, 1, 1, 3, 3, 3, 1, 4, 3,
    5, 3, 5, 5, 1, 4, 4, 2, 5, 1, 5, 5, 4, 5, 5, 1, 5, 4, 4, 1, 3, 4, 1, 2, 3, 2, 5, 1, 3, 1, 5, 5,
    2, 2, 2, 1, 3, 3, 1, 1, 1, 4, 2, 5, 1, 2, 4, 4, 2, 5, 1, 1, 3, 5, 4, 2, 1, 2, 5, 4, 1, 5, 5, 2,
    4, 3, 5, 2, 4, 1, 4, 3, 5, 5, 3, 1, 5, 1, 3, 5, 1, 1, 1, 4, 2, 4, 4, 1, 1, 1, 1, 1, 3, 4, 5, 2,
    3, 4, 5, 1, 4, 1, 2, 3, 4, 2, 1, 4, 4, 2, 1, 5, 3, 4, 1, 1, 2, 2, 1, 5, 5, 2, 5, 1, 4, 4, 2, 1,
    3, 1, 5, 5, 1, 4, 2, 2, 1, 1, 1, 5, 1, 3, 4, 1, 3, 3, 5, 3, 5, 5, 3, 1, 4, 4, 1, 1, 1, 3, 3, 2,
    3, 1, 1, 1, 5, 4, 2, 5, 3, 5, 4, 4, 5, 2, 3, 2, 5, 2, 1, 1, 1, 2, 1, 5, 3, 5, 1, 4, 1, 2, 1, 5,
    3, 5, 2, 1, 3, 1, 2, 4, 5, 3, 4, 3,
];

fn main() {
    let mut school = vec![0; 9];
    for fish in INPUT {
        school[fish] += 1;
    }
    assert_eq!(INPUT.len(), school.iter().sum());

    let school = simulate_n(school, 80);
    let part_1 = sum(&school);

    println!("Part 1: {}", part_1);
    assert_eq!(350605, part_1);

    let school = simulate_n(school, 256 - 80);
    let part_2 = sum(&school);

    println!("Part 2: {}", part_2);
    assert_eq!(1592778185024, part_2);
}

// Each simulation, a 0 becomes a 6 and adds a new 8 to the end of the list, while each other
// number decreases by 1 if it was present at the start of the day
fn simulate(mut school: Vec<usize>, _: ()) -> Vec<usize> {
    let count = school.remove(0); // Remove the fish with 0 days left
    school.push(count); // Spawn new fish at day 8
    school[6] += count; // Add old "zero" fish to the 6 day fish
    school
}

fn simulate_n(school: Vec<usize>, n: usize) -> Vec<usize> {
    std::iter::repeat(()).take(n).fold(school, simulate)
}

fn sum(input: &[usize]) -> usize {
    input.iter().sum()
}
