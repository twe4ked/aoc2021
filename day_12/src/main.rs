use std::collections::HashMap;

const INPUT: &str = "ln-nr ln-wy fl-XI qc-start qq-wy qc-ln ZD-nr qc-YN XI-wy ln-qq ln-XI YN-start
                     qq-XI nr-XI start-qq qq-qc end-XI qq-YN ln-YN end-wy qc-nr end-nr";

fn main() {
    let graph = Graph::new(INPUT);

    let part_1 = graph.distinct_paths(Path::default(), &Cave::Start).len();
    println!("Part 1: {}", part_1);
    assert_eq!(4773, part_1);

    let part_2 = graph
        .distinct_paths(Path::with_allow_double_visit_one_small_cave(), &Cave::Start)
        .len();
    println!("Part 2: {}", part_2);
    assert_eq!(116985, part_2);
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, Ord, PartialOrd)]
enum Cave<'a> {
    Start,
    End,
    Small(&'a str),
    Big(&'a str),
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Path<'a> {
    path: Vec<Cave<'a>>,
    allow_double_visit_one_small_cave: bool,
    has_double_visited_one_small_cave: bool,
}

impl<'a> Path<'a> {
    fn with_allow_double_visit_one_small_cave() -> Self {
        Self {
            allow_double_visit_one_small_cave: true,
            ..Default::default()
        }
    }

    fn push(&mut self, cave: Cave<'a>) {
        // If we're allowed to double visit one small cave, we need to track that state
        if self.allow_double_visit_one_small_cave
            && !self.has_double_visited_one_small_cave
            && matches!(cave, Cave::Small(_))
            && self.path.contains(&cave)
        {
            self.has_double_visited_one_small_cave = true;
        }

        self.path.push(cave)
    }

    fn can_visit_small_cave(&self, cave: &Cave<'a>) -> bool {
        if self.allow_double_visit_one_small_cave {
            // A single small cave can be visited at most twice, and the remaining small caves can
            // be visited at most once.
            !self.has_double_visited_one_small_cave || !self.path.contains(cave)
        } else {
            // Small caves can only be visited once.
            !self.path.contains(cave)
        }
    }
}

#[derive(Debug, Default)]
struct Graph<'a> {
    graph: HashMap<Cave<'a>, Vec<Cave<'a>>>,
}

impl<'a> Graph<'a> {
    fn new(input: &'a str) -> Self {
        let mut graph = Graph::default();

        for line in input.split_whitespace() {
            let mut caves = line.split('-').map(|name| {
                if name == "start" {
                    Cave::Start
                } else if name == "end" {
                    Cave::End
                } else if name.chars().next().unwrap().is_ascii_uppercase() {
                    Cave::Big(name)
                } else {
                    Cave::Small(name)
                }
            });

            graph.add_edge(caves.next().unwrap(), caves.next().unwrap())
        }

        graph
    }

    // Add edges pointing both directions
    fn add_edge(&mut self, u: Cave<'a>, v: Cave<'a>) {
        self.graph
            .entry(u.clone())
            .or_insert_with(Vec::new)
            .push(v.clone());
        self.graph.entry(v).or_insert_with(Vec::new).push(u);
    }

    // Your goal is to find the number of distinct paths that start at start, end at end, and don't
    // visit small caves more than once. There are two types of caves: big caves (written in
    // uppercase, like A) and small caves (written in lowercase, like b). It would be a waste of
    // time to visit any small cave more than once, but big caves are large enough that it might be
    // worth visiting them multiple times. So, all paths you find should visit small caves at most
    // once, and can visit big caves any number of times.
    fn distinct_paths(&self, mut visited: Path<'a>, cave: &Cave<'a>) -> Vec<Path<'a>> {
        visited.push(cave.clone());

        let mut ret = Vec::new();
        for adjacent_cave in self.graph.get(cave).unwrap() {
            match adjacent_cave {
                Cave::Start => {
                    // Start can only be visited once, this is a dead end.
                }
                Cave::End => {
                    // Reached the end, keep the path to return
                    ret.push(visited.clone())
                }
                Cave::Big(_name) => {
                    // Allowed to visit more than once
                    ret.append(&mut self.distinct_paths(visited.clone(), adjacent_cave))
                }
                Cave::Small(_name) => {
                    if visited.can_visit_small_cave(adjacent_cave) {
                        ret.append(&mut self.distinct_paths(visited.clone(), adjacent_cave));
                    }
                }
            }
        }
        ret
    }
}
