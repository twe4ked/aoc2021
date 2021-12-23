use std::cmp::Ordering;
use std::ops::RangeInclusive;

const SEARCH_SPACE: i32 = 13_000;

fn main() {
    // target area: x=94..151, y=-156..-103
    let target_area = TargetArea::new(94..=151, -156..=-103); // Input

    let mut positions = search(&target_area);

    let part_1 = positions.iter().max_by(|a, b| a.2.cmp(&b.2)).unwrap().2;

    positions.sort_unstable();
    positions.dedup();
    let part_2 = positions.len();

    println!("Part 1: {}", part_1);
    assert_eq!(12090, part_1);

    println!("Part 2: {}", part_2);
    assert_eq!(5059, part_2);
}

#[derive(Debug, Default)]
struct Probe {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
    max_y: i32,
}

impl Probe {
    fn new(vx: i32, vy: i32) -> Self {
        Self {
            vx,
            vy,
            ..Default::default()
        }
    }

    fn iterate(target_area: &TargetArea, x: i32, y: i32) -> Option<i32> {
        let mut probe = Probe::new(x, y);
        loop {
            probe.step();
            if target_area.contains(probe.px, probe.py) {
                return Some(probe.max_y);
            }
            if probe.bounds_check() {
                return None;
            }
        }
    }

    // - The probe's x position increases by its x velocity.
    // - The probe's y position increases by its y velocity.
    // - Due to drag, the probe's x velocity changes by 1 toward the value 0; that is, it decreases by
    //   1 if it is greater than 0, increases by 1 if it is less than 0, or does not change if it is
    //   already 0.
    // - Due to gravity, the probe's y velocity decreases by 1.
    fn step(&mut self) {
        self.px += self.vx;
        self.py += self.vy;

        match self.vx.cmp(&0) {
            Ordering::Less => self.vx += 1,
            Ordering::Greater => self.vx -= 1,
            Ordering::Equal => {}
        }

        self.vy -= 1;

        self.max_y = self.max_y.max(self.py);
    }

    fn bounds_check(&self) -> bool {
        self.px.abs() > SEARCH_SPACE
            || self.py.abs() > SEARCH_SPACE
            || self.vx.abs() > SEARCH_SPACE
            || self.vy.abs() > SEARCH_SPACE
    }
}

struct TargetArea {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
}

impl TargetArea {
    fn new(x: RangeInclusive<i32>, y: RangeInclusive<i32>) -> Self {
        Self { x, y }
    }

    fn contains(&self, x: i32, y: i32) -> bool {
        self.x.contains(&x) && self.y.contains(&y)
    }
}

fn search(target_area: &TargetArea) -> Vec<(i32, i32, i32)> {
    (-SEARCH_SPACE..SEARCH_SPACE)
        .flat_map(|x| {
            (-SEARCH_SPACE..SEARCH_SPACE)
                .filter_map(move |y| Probe::iterate(target_area, x, y).map(|max_y| (x, y, max_y)))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let target_area = TargetArea::new(20..=30, -10..=-5);
        assert_eq!(3, Probe::iterate(&target_area, 7, 2).unwrap());
        assert_eq!(6, Probe::iterate(&target_area, 6, 3).unwrap());
        assert_eq!(0, Probe::iterate(&target_area, 9, 0).unwrap());
        assert_eq!(45, Probe::iterate(&target_area, 6, 9).unwrap());
        assert!(Probe::iterate(&target_area, 17, -4).is_none());
    }
}
