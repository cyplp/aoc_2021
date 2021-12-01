use std::fs;

use std::cmp::{Ord, Ordering, PartialOrd};
use std::ops::Add;

#[derive(Debug, Eq, PartialOrd, Copy, Clone)]
struct Depth(u32);

impl Depth {
    pub fn new(val: &str) -> Depth {
        Depth(val.parse::<u32>().unwrap())
    }
}

impl PartialEq for Depth {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Ord for Depth {
    /// We sort by alphabical order.
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl Add for Depth {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}
#[derive(Debug, Eq, PartialOrd, Copy, Clone)]
struct WindowDepth(Depth, Depth, Depth);

impl PartialEq for WindowDepth {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == self.2
    }
}
impl Ord for WindowDepth {
    /// We sort by alphabical order.
    fn cmp(&self, other: &Self) -> Ordering {
        self.sum().cmp(&other.sum())
    }
}

impl WindowDepth {
    pub fn new(values: Vec<Depth>) -> WindowDepth {
        WindowDepth(values[0].clone(), values[1].clone(), values[2].clone())
    }
    pub fn sum(&self) -> Depth {
        self.clone().0 + self.clone().1 + self.clone().2
    }
}

fn parse(filename: &str) -> String{
    fs::read_to_string(filename).expect("can't read input")
}


fn main() {
    let contents = parse("input");
    let depths: Vec<Depth> = contents
        .lines()
        .into_iter()
        .map(|value| Depth::new(value))
        .collect();
    let mut cpt: u32 = 0;
    let mut previous: Option<Depth> = None;
    for depth in depths.clone() {
        match previous {
            Some(previous) if previous < depth => cpt += 1,
            _ => (),
        }
        previous = Some(depth);
    }

    println!("part1 cpt : {}", cpt);

    let mut cpt: u32 = 0;
    let mut previous: Option<Depth> = None;

    for depthw in depths.windows(3) {
        let wd = WindowDepth::new(depthw.to_vec());
        match previous {
            Some(previous) if previous < wd.sum() =>  cpt += 1,
            _ => ( ),
        }
        previous = Some(wd.sum());
    }
    println!("part2 cpt : {}", cpt);
}

#[cfg(test)]
mod test_depth {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Depth::new("4").0, 4);
    }

    #[test]
    fn test_eq() {
        assert_eq!(Depth::new("4"), Depth::new("4"));
    }

    #[test]
    fn test_ord() {
        assert!(Depth::new("4") > Depth::new("3"));
        assert!(Depth::new("4") < Depth::new("5"));
        assert!(Depth::new("4") == Depth::new("4"));
    }

    #[test]
    fn test_add() {
        assert_eq!(Depth::new("2") + Depth::new("3"), Depth::new("5"));
    }
}

#[cfg(test)]
mod tests_window {
    use super::*;

    #[test]
    fn test_new() {
        let wd = WindowDepth::new(vec![Depth::new("1"), Depth::new("2"), Depth::new("3")]);
        assert_eq!(wd.0, Depth::new("1"));
    }

    #[test]
    fn test_sum() {
        let wd = WindowDepth::new(vec![Depth::new("1"), Depth::new("2"), Depth::new("3")]);
        assert_eq!(wd.sum(), Depth::new("6"));
    }

    #[test]
    fn test_eq() {
        assert_eq!(
            WindowDepth::new(vec![Depth::new("1"), Depth::new("2"), Depth::new("3")]),
            WindowDepth::new(vec![Depth::new("1"), Depth::new("2"), Depth::new("3")])
        );

        assert_ne!(
            WindowDepth::new(vec![Depth::new("2"), Depth::new("1"), Depth::new("3")]),
            WindowDepth::new(vec![Depth::new("1"), Depth::new("2"), Depth::new("3")])
        );
    }

    #[test]
    fn test_cmp() {
        assert!(
            WindowDepth::new(vec![Depth::new("1"), Depth::new("2"), Depth::new("3")])
                > WindowDepth::new(vec![Depth::new("1"), Depth::new("2"), Depth::new("2")])
        );
        assert!(
            WindowDepth::new(vec![Depth::new("1"), Depth::new("2"), Depth::new("3")])
                == WindowDepth::new(vec![Depth::new("1"), Depth::new("2"), Depth::new("3")])
        );

    }
}
