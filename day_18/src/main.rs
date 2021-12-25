use std::fmt;
use std::ops::Add;

fn main() {
    let input = include_str!("../input");

    let numbers: Vec<_> = input.lines().map(FlatSnailfishNumber::from).collect();

    let part_1 = numbers
        .clone()
        .into_iter()
        .reduce(|acc, sfn| acc + sfn)
        .unwrap()
        .magnitude();
    println!("Part 1: {}", part_1);
    assert_eq!(2501, part_1);

    let mut part_2 = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            part_2 = part_2.max((numbers[i].clone() + numbers[j].clone()).magnitude());
        }
    }
    assert_eq!(4935, part_2);
    println!("Part 2: {}", part_2);
}

#[derive(PartialEq)]
enum RecursiveSnailfishNumber {
    Literal(u32),
    Pair(Box<RecursiveSnailfishNumber>, Box<RecursiveSnailfishNumber>),
}

impl fmt::Debug for RecursiveSnailfishNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecursiveSnailfishNumber::Literal(n) => write!(f, "{}", n),
            RecursiveSnailfishNumber::Pair(a, b) => write!(f, "[{:?},{:?}]", a, b),
        }
    }
}

impl From<&str> for RecursiveSnailfishNumber {
    fn from(input: &str) -> Self {
        let (input, snailfish_number) = parse_recursive_snailfish_number(input);
        assert!(input.is_empty());
        snailfish_number
    }
}

fn parse_recursive_snailfish_number(input: &str) -> (&str, RecursiveSnailfishNumber) {
    match &input[0..1] {
        "[" => {
            let (input, a) = parse_recursive_snailfish_number(&input[1..]); // 1 skips the `[`
            let (input, b) = parse_recursive_snailfish_number(&input[1..]); // 1 skips the `,`
            (&input[1..], RecursiveSnailfishNumber::pair(a, b)) // 1 skips the `]`
        }
        num => (
            &input[1..],
            RecursiveSnailfishNumber::literal(num.parse().unwrap()),
        ),
    }
}

impl RecursiveSnailfishNumber {
    fn pair(a: Self, b: Self) -> Self {
        Self::Pair(Box::new(a), Box::new(b))
    }

    fn literal(n: u32) -> Self {
        Self::Literal(n)
    }

    // The magnitude of a pair is 3 times the magnitude of its left element plus 2 times the
    // magnitude of its right element.
    fn magnitude(self) -> u32 {
        match self {
            Self::Literal(n) => n,
            Self::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

impl From<&FlatSnailfishNumber> for RecursiveSnailfishNumber {
    fn from(n: &FlatSnailfishNumber) -> Self {
        let (input, n) = parse_flat_snailfish_number(&n.items[..]);
        assert!(input.is_empty());
        n
    }
}

fn parse_flat_snailfish_number(input: &[Item]) -> (&[Item], RecursiveSnailfishNumber) {
    match input[0..1] {
        [Item::Open] => {
            let (input, a) = parse_flat_snailfish_number(&input[1..]); // 1 skips the `[`
            let (input, b) = parse_flat_snailfish_number(&input[1..]); // 1 skips the `,`
            (&input[1..], RecursiveSnailfishNumber::pair(a, b)) // 1 skips the `]`
        }
        [Item::Integer(n)] => (&input[1..], RecursiveSnailfishNumber::literal(n)),
        _ => unreachable!(),
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Item {
    Open,
    Comma,
    Close,
    Integer(u32),
}

impl Item {
    fn integer(&self) -> Option<u32> {
        match self {
            Item::Integer(n) => Some(*n),
            _ => None,
        }
    }
}

#[derive(PartialEq, Clone)]
struct FlatSnailfishNumber {
    items: Vec<Item>,
}

impl fmt::Debug for FlatSnailfishNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in &self.items {
            match i {
                Item::Open => write!(f, "[")?,
                Item::Close => write!(f, "]")?,
                Item::Comma => write!(f, ",")?,
                Item::Integer(n) => write!(f, "{}", n)?,
            }
        }
        Ok(())
    }
}

impl Add for FlatSnailfishNumber {
    type Output = Self;

    fn add(mut self, mut other: Self) -> Self {
        // [
        self.items.insert(0, Item::Open);
        // [ ... ,
        self.items.push(Item::Comma);
        // [ self.items , other.items
        self.items.append(&mut other.items);
        // [ self.items , other.items ]
        self.items.push(Item::Close);

        self.reduce();
        self
    }
}

impl From<&str> for FlatSnailfishNumber {
    fn from(input: &str) -> Self {
        let items = input
            .chars()
            .map(|c| match c {
                '[' => Item::Open,
                ']' => Item::Close,
                ',' => Item::Comma,
                num => Item::Integer(num.to_digit(10).unwrap()),
            })
            .collect();
        FlatSnailfishNumber { items }
    }
}

impl FlatSnailfishNumber {
    fn reduce(&mut self) {
        while self.explode() || self.split() {}
    }

    fn explode(&mut self) -> bool {
        let mut i = 0;
        let mut depth = 0;

        loop {
            match self.items[i] {
                Item::Open => {
                    depth += 1;
                    i += 1;
                }
                Item::Close => {
                    depth -= 1;
                    i += 1;
                }
                Item::Comma => i += 1,
                Item::Integer(_) => {
                    // If any pair is nested inside four pairs, the leftmost such pair explodes.
                    if depth == 5 {
                        // To explode a pair, the pair's left value is added to the first regular
                        // number to the left of the exploding pair (if any), and the pair's right
                        // value is added to the first regular number to the right of the exploding
                        // pair (if any). Exploding pairs will always consist of two regular
                        // numbers. Then, the entire exploding pair is replaced with the regular
                        // number 0.

                        // Back to the "["
                        i -= 1;

                        assert!(matches!(self.items.remove(i), Item::Open));
                        let left = self.items.remove(i).integer().unwrap();
                        assert!(matches!(self.items.remove(i), Item::Comma));
                        let right = self.items.remove(i).integer().unwrap();
                        assert!(matches!(self.items.remove(i), Item::Close));

                        let separator = self.items.remove(i);
                        assert!(matches!(separator, Item::Comma | Item::Close));

                        // Try place number on the left
                        if let Some(Item::Integer(ref mut n)) = self.items[..i - 1]
                            .iter_mut()
                            .rfind(|i| matches!(i, Item::Integer(_)))
                        {
                            *n += left;
                        }

                        // Try place number on the right
                        if let Some(Item::Integer(ref mut n)) = self.items[i..]
                            .iter_mut()
                            .find(|i| matches!(i, Item::Integer(_)))
                        {
                            *n += right;
                        }

                        self.items.insert(i, Item::Integer(0));
                        self.items.insert(i + 1, separator);
                        return true;
                    }

                    i += 1;
                }
            }

            if i >= self.items.len() {
                break false;
            }
        }
    }

    fn split(&mut self) -> bool {
        let mut i = 0;

        loop {
            match self.items[i] {
                Item::Integer(n) => {
                    // If any regular number is 10 or greater, the leftmost such regular number
                    // splits.
                    if n >= 10 {
                        let number_to_split = self.items.remove(i).integer().unwrap();

                        fn div_round_up(a: u32, b: u32) -> u32 {
                            (a + b - 1) / b
                        }

                        let left = number_to_split / 2;
                        let right = div_round_up(number_to_split, 2);

                        self.items.insert(i, Item::Open);
                        self.items.insert(i + 1, Item::Integer(left));
                        self.items.insert(i + 2, Item::Comma);
                        self.items.insert(i + 3, Item::Integer(right));
                        self.items.insert(i + 4, Item::Close);

                        return true;
                    }

                    i += 1;
                }
                _ => i += 1,
            }

            if i >= self.items.len() {
                break;
            }
        }

        false
    }

    fn magnitude(&self) -> u32 {
        RecursiveSnailfishNumber::from(self).magnitude()
    }
}
