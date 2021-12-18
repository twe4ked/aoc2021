fn main() {
    let input = parse(include_str!("../input"));

    let part_1 = part_1(&input);
    println!("Part 1: {}", part_1);
    assert_eq!(374061, part_1);

    let part_2 = part_2(&input);
    println!("Part 2: {}", part_2);
    assert_eq!(2116639949, part_2);
}

enum Line {
    Corrupt(char),
    Incomplete(Vec<char>),
}

impl Line {
    fn parse(line: &[char]) -> Self {
        let mut stack = Vec::new();
        for c in line {
            match c {
                '(' | '[' | '{' | '<' => stack.push(*c),
                ')' => {
                    if stack.pop().unwrap() != '(' {
                        return Line::Corrupt(*c);
                    }
                }
                ']' => {
                    if stack.pop().unwrap() != '[' {
                        return Line::Corrupt(*c);
                    }
                }
                '}' => {
                    if stack.pop().unwrap() != '{' {
                        return Line::Corrupt(*c);
                    }
                }
                '>' => {
                    if stack.pop().unwrap() != '<' {
                        return Line::Corrupt(*c);
                    }
                }
                _ => unreachable!(),
            }
        }
        Line::Incomplete(stack)
    }
}

fn part_1(program: &[Vec<char>]) -> u32 {
    let mut acc = 0;
    for line in program {
        match Line::parse(line) {
            Line::Corrupt(')') => acc += 3,
            Line::Corrupt(']') => acc += 57,
            Line::Corrupt('}') => acc += 1197,
            Line::Corrupt('>') => acc += 25137,
            Line::Incomplete(_) => {}
            _ => unreachable!(),
        }
    }
    acc
}

fn part_2(program: &[Vec<char>]) -> u64 {
    let mut scores = Vec::new();
    for line in program {
        match Line::parse(line) {
            Line::Corrupt(_) => {}
            Line::Incomplete(line) => {
                // For each character, multiply the total score by 5 and then increase the total
                // score by the point value given for the character.
                let mut score = 0;
                for c in line.iter().rev() {
                    score *= 5;
                    score += match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => unreachable!(),
                    };
                }
                scores.push(score);
            }
        }
    }
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.trim().chars().collect::<Vec<_>>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let example = parse(
            "[({(<(())[]>[[{[]{<()<>>
             [(()[<>])]({[<{<<[]>>(
             {([(<{}[<>[]}>{[]{[(<()>
             (((({<>}<{<{<>}{[]{[]{}
             [[<[([]))<([[{}[[()]]]
             [{[{({}]{}}([{[{{{}}([]
             {<[[]]>}<{[{[{[]{()[[[]
             [<(<(<(<{}))><([]([]()
             <{([([[(<>()){}]>(<<{{
             <{([{{}}[<[[[<>{}]]]>[]]",
        );

        assert_eq!(26397, part_1(&example));
        assert_eq!(288957, part_2(&example));
    }
}
