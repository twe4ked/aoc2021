use std::collections::HashMap;

type PairInsertionRules = HashMap<(char, char), char>;

#[derive(Default, Debug)]
struct PolymerTemplate {
    template: HashMap<(char, char), u64>,
    counts: HashMap<char, u64>,
}

impl PolymerTemplate {
    fn new(input: &str) -> Self {
        let chars: Vec<_> = input.chars().collect();
        let mut pt = PolymerTemplate::default();
        for i in 0..(chars.len() - 1) {
            *pt.template.entry((chars[i], chars[i + 1])).or_insert(0) += 1;
            // Keep counts of each char
            *pt.counts.entry(chars[i]).or_insert(0) += 1;
        }
        // Because the above loop doesn't reach the last element,
        // increase the count for the final char
        *pt.counts.entry(*chars.last().unwrap()).or_insert(0) += 1;
        pt
    }

    fn step(&mut self, pair_insertion_rules: &PairInsertionRules) {
        for (key, count) in self.template.clone() {
            if let Some(&value) = pair_insertion_rules.get(&key) {
                *self.template.entry((key.0, value)).or_insert(0) += count;
                *self.template.entry((value, key.1)).or_insert(0) += count;
                *self.template.entry(key).or_insert(0) -= count;

                // Increase the count any time we add a new char
                *self.counts.entry(value).or_insert(0) += count;
            }
        }
    }

    fn result(&self) -> u64 {
        let min = self.counts.values().min().unwrap();
        let max = self.counts.values().max().unwrap();
        max - min
    }
}

fn main() {
    let input = include_str!("../input");
    let mut parts = input.split("\n\n");
    let mut polymer_template = PolymerTemplate::new(parts.next().unwrap());
    let pair_insertion_rules = parse_pair_insertion_rules(parts.next().unwrap());

    for _ in 0..10 {
        polymer_template.step(&pair_insertion_rules);
    }

    let part_1 = polymer_template.result();
    println!("Part 1: {}", part_1);
    assert_eq!(2712, part_1);

    for _ in 0..30 {
        polymer_template.step(&pair_insertion_rules);
    }

    let part_2 = polymer_template.result();
    println!("Part 2: {}", part_2);
    assert_eq!(8336623059567, part_2);
}

fn parse_pair_insertion_rules(input: &str) -> PairInsertionRules {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(" -> ");
            let mut key = parts.next().unwrap().chars();
            (
                (key.next().unwrap(), key.next().unwrap()),
                parts.next().unwrap().chars().next().unwrap(),
            )
        })
        .collect()
}
