use std::fs;

#[derive(Debug, PartialEq, Clone, Copy)]
struct MarkedNumber {
    value: usize,
    marked: bool,
}
impl MarkedNumber {
    pub fn new(val: &str) -> MarkedNumber {
        MarkedNumber {
            value: val.parse().unwrap(),
            marked: false,
        }
    }

    pub fn value(&self) -> usize {
        self.value
    }

    pub fn is_marked(&self) -> bool {
        self.marked
    }

    pub fn mark(&mut self) {
        self.marked = true;
    }
}
#[cfg(test)]
mod test_marked_number {
    use super::*;

    #[test]
    fn test_marked_number() {
        let mut number = MarkedNumber::new("25");
        assert_eq!(number.value(), 25);
        assert!(!number.is_marked());

        number.mark();
        assert!(number.is_marked());
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Grid {
    numbers: Vec<MarkedNumber>,
}

impl Grid {
    pub fn new(values: Vec<&str>) -> Grid {
        Grid {
            numbers: values.iter().map(|val| MarkedNumber::new(val)).collect(),
        }
    }

    pub fn check_number(&mut self, val: usize) {
        for mut current in self.numbers.iter_mut() {
            if current.value() == val {
                current.mark();
            }
        }
    }

    pub fn is_bingo(&self) -> bool {
        for index in 0..5 {
            if self.check_line(index) {
                return true;
            }
        }

        for index in 0..5 {
            if self.check_col(index) {
                return true;
            }
        }
        false
    }

    fn check_line(&self, index: usize) -> bool {
        self.numbers[index * 5..(index * 5) + 5]
            .iter()
            .filter(|val| val.is_marked())
            .collect::<Vec<&MarkedNumber>>()
            .len()
            == 5
    }

    fn check_col(&self, index: usize) -> bool {
        self.numbers
            .iter()
            .skip(index)
            .step_by(5)
            .filter(|val| val.is_marked())
            .collect::<Vec<&MarkedNumber>>()
            .len()
            == 5
    }

    pub fn unmarked_sum(&self) -> u32 {
	self.numbers.iter().filter(|val| !val.is_marked()).map(|val| val.value() as u32).sum()

    }
}

#[cfg(test)]
mod test_grid {
    use super::*;

    #[test]
    fn test_new() {
        let grid = Grid::new(vec![
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16",
            "17", "18", "19", "20", "21", "22", "23", "24", "25",
        ]);
    }

    #[test]
    fn test_check_line() {
        let mut grid = Grid::new(vec![
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16",
            "17", "18", "19", "20", "21", "22", "23", "24", "25",
        ]);

        assert!(!grid.check_line(2));
        for number in 11..16 {
            grid.check_number(number);
        }

        assert!(grid.check_line(2));
    }

    #[test]
    fn test_check_col() {
        let mut grid = Grid::new(vec![
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16",
            "17", "18", "19", "20", "21", "22", "23", "24", "25",
        ]);

        assert!(!grid.check_col(2));
        for number in (3..24).step_by(5) {
            grid.check_number(number);
        }
        assert!(grid.check_col(2));
    }

    #[test]
    fn test_is_bingo() {
        let mut grid = Grid::new(vec![
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16",
            "17", "18", "19", "20", "21", "22", "23", "24", "25",
        ]);

        assert!(!grid.is_bingo());
        for number in (3..24).step_by(5) {
            grid.check_number(number);
        }
        assert!(grid.is_bingo());
	assert_eq!(grid.unmarked_sum(), 260);
    }
}

fn parse(filename: &str) -> (Vec<usize>, Vec<Grid>) {
    let content = fs::read_to_string(filename)
        .expect("can't read input");
    let lines = content
        .lines()
        .filter(|current| !current.is_empty())
        .map(|val| val.clone())
        .collect::<Vec<&str>>();

    let numbers: Vec<usize> =
     lines[0]
        .split(',')
        .map(|val| val.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut tmp: Vec<&str> = Vec::new();
    let mut grids: Vec<Grid> = Vec::new();

    for line in lines.iter().skip(1) {
	let mut foo: Vec<&str> = line.split(' ').filter(|val| val != &"").collect::<Vec<&str>>();
	tmp.append(&mut foo);
	if tmp.len() < 25 { continue ;}
	let grid = Grid::new(tmp.clone());
	grids.push(grid);
	tmp = Vec::new();
    }
    return (numbers, grids);
}

fn main() {
    let (numbers, mut grids) = parse("input");

    'outer: for number in numbers {
	for grid in grids.iter_mut() {
	    grid.check_number(number);
	    if grid.is_bingo() {
		println!("part1 {}", number as u32 * grid.unmarked_sum());
		break 'outer;

	    }
	}
    }

    let mut score: u32 = 0;
    let (numbers, mut grids) = parse("input");
    for number in numbers {
	for grid in grids.iter_mut() {
	    if grid.is_bingo() {continue}
	    grid.check_number(number);
	    if grid.is_bingo() {
		score =  number as u32 * grid.unmarked_sum();
	    }
	}
    }
    println!("part2 {}", score);
}
