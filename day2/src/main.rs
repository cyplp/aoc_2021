use std::fs;

use std::cmp::{Ord, Ordering, PartialOrd};
use std::ops::Add;

#[derive(Debug, Eq, PartialOrd, Copy, Clone)]
struct Depth(u32);

impl Depth {
    pub fn new(val: &str) -> Depth {
        Depth(val.parse::<u32>().unwrap())
    }

    pub fn up(&mut self, val: u32) {
        self.0 = self.0 - val;
    }

    pub fn down(&mut self, val: u32) {
        self.0 = self.0 + val;
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
struct Horizontal(u32);

impl Horizontal {
    pub fn new(val: &str) -> Horizontal {
        Horizontal(val.parse::<u32>().unwrap())
    }

    pub fn foward(&mut self, val: u32) {
        self.0 = self.0 + val;
    }
}

impl PartialEq for Horizontal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Ord for Horizontal {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

#[derive(PartialEq, Debug)]
enum Mouvement {
    Forward,
    Up,
    Down,
    Unkwown,
}

struct Instruction {
    verb: Mouvement,
    measure: u32,
}

impl Instruction {
    pub fn parse(raw: &str) -> Instruction {
        let mut data: Vec<&str> = raw.split(" ").collect();
        let verb_str = data[0];
        let verb = match verb_str {
            "forward" => Mouvement::Forward,
            "up" => Mouvement::Up,
            "down" => Mouvement::Down,
            _ => Mouvement::Unkwown,
        };
        let measure: u32 = data[1].parse().unwrap();

        Instruction {
            verb: verb,
            measure: measure,
        }
    }
}

#[derive(Debug, Eq, Clone)]
struct Aim(u32);
impl Aim {
    pub fn new(val: u32) -> Aim {
        Aim(val)
    }
    pub fn down(&mut self, val: u32) {
        self.0 = self.0 + val;
    }
    pub fn up(&mut self, val: u32) {
        self.0 = self.0 - val;
    }
}

impl PartialEq for Aim {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

fn parse(filename: &str) -> Vec<Instruction> {
    let content = fs::read_to_string(filename).expect("can't read input");
    content
        .lines()
        .into_iter()
        .map(|value| Instruction::parse(value))
        .collect()
}

fn follow_instructions_part1(instructions: &Vec<Instruction>) -> (Depth, Horizontal) {
    let mut depth = Depth::new("0");
    let mut hor = Horizontal::new("0");

    for instruction in instructions {
        match instruction.verb {
            Mouvement::Forward => hor.foward(instruction.measure),
            Mouvement::Up => depth.up(instruction.measure),
            Mouvement::Down => depth.down(instruction.measure),
            _ => panic!("oh shit"),
        }
    }
    (depth, hor)
}

fn follow_instructions_part2(instructions: &Vec<Instruction>) -> (Depth, Horizontal, Aim) {
    let mut depth = Depth::new("0");
    let mut hor = Horizontal::new("0");
    let mut aim = Aim::new(0);

    for instruction in instructions {
        match instruction.verb {
            Mouvement::Forward => {
                hor.foward(instruction.measure);
                depth.down(aim.0 * instruction.measure);
            }
            Mouvement::Up => {
                aim.up(instruction.measure);
            }
            Mouvement::Down => {
                aim.down(instruction.measure);
            }
            _ => panic!("oh shit"),
        }
    }
    (depth, hor, aim)
}

fn main() {
    let instructions = parse("input");
    let (depth, ho) = follow_instructions_part1(&instructions);

    println!("response part1 {}", depth.0 * ho.0);

    let (depth, ho, aim) = follow_instructions_part2(&instructions);
    println!("response part1 {}", depth.0 * ho.0);
}

#[cfg(test)]
mod test_depth {
    use super::*;

    #[test]
    fn test_mouvement() {
        let mut depth = Depth::new("0");

        depth.down(5);
        assert_eq!(depth, Depth::new("5"));

        depth.up(2);
        assert_eq!(depth, Depth::new("3"));
    }
}

#[cfg(test)]
mod test_horizontal {
    use super::*;

    #[test]
    fn test_foward() {
        let mut ho = Horizontal::new("0");
        ho.foward(1);

        assert_eq!(ho, Horizontal::new("1"));
    }
}

#[cfg(test)]
mod test_instruction {
    use super::*;

    #[test]
    fn test_parse() {
        let instruction = Instruction::parse("forward 5");
        assert_eq!(instruction.verb, Mouvement::Forward);
        assert_eq!(instruction.measure, 5);
    }
}

#[cfg(test)]
mod test_function {
    use super::*;

    #[test]
    fn tests_follow_instructions_part1() {
        let raw = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        let instructions = raw
            .into_iter()
            .map(|value| Instruction::parse(value))
            .collect();
        let (depth, ho) = follow_instructions_part1(&instructions);

        assert_eq!(depth, Depth::new("10"));
        assert_eq!(ho, Horizontal::new("15"));
    }

    #[test]
    fn tests_follow_instructions_part2() {
        let raw = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        let instructions = raw
            .into_iter()
            .map(|value| Instruction::parse(value))
            .collect();
        let (depth, ho, aim) = follow_instructions_part2(&instructions);

        assert_eq!(depth, Depth::new("60"));
        assert_eq!(ho, Horizontal::new("15"));
        assert_eq!(aim, Aim::new(10));
    }
}
