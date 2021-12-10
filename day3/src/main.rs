use std::fs;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Bit {
    One,
    Zero,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
struct BitNumber {
    pub data: Vec<Bit>,
}

impl BitNumber {
    pub fn new(raw: &str) -> BitNumber {
        let tmp: Vec<char> = raw.chars().collect();
        let mut data: Vec<Bit> = Vec::new();
        for current in tmp.iter() {
            let val = match current {
                '1' => Bit::One,
                '0' => Bit::Zero,
                _ => Bit::Unknown,
            };
            data.push(val);
        }
        BitNumber { data: data }
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn invert(&self) -> BitNumber {
	let mut tmp: Vec<Bit> = Vec::new();
	for val in &self.data {
	    if *val == Bit::One {
		tmp.push(Bit::Zero);
	    } else {
		tmp.push(Bit::One);
	    }
	}
	BitNumber{data: tmp}
    }

    pub fn to_u64(&self) -> u64 {
	let mut result:u64 = 0;
	for (index, bit) in self.data.iter().rev().enumerate() {
	    if *bit==Bit::One {
		result = result + (2u64.pow(index.try_into().unwrap()));
	    }
	}
	result
    }

    pub fn is_this_byte_one(&self, index: usize) -> bool {
	self.data[index] == Bit::One
    }
}

struct Report(Vec<BitNumber>);

impl Report {
    fn new(rows: Vec<BitNumber>) -> Report {
        Report(rows)
    }

    fn size_row(&self) -> usize {
        self.0.len()
    }

    fn size_column(&self) -> usize {
        self.0[0].size()
    }

    fn gamma_rate(&self) -> BitNumber {
        let number_column = self.size_column();

        let mut tmp: Vec<char> = Vec::new();
        for y in 0..number_column {
	    let foo = self
                .0
                .iter()
                .map(|val| val.data[y].clone())
                .collect::<Vec<Bit>>();

            let mut count = 0;
	    for x in &foo {
		if *x == Bit::One {
		    count +=1;
		}
	    }

            if count <= foo.len() / 2 {
                tmp.push('0');
            } else {
                tmp.push('1');
            }
        }

        BitNumber::new(String::from_iter(tmp).as_str())
    }

    fn epsilon_rate(&self) -> BitNumber {
	self.gamma_rate().invert()
    }

    fn filter_on_index(input: Vec<BitNumber>, index: usize, val: Bit) -> Vec<BitNumber> {
	let mut output: Vec<BitNumber> = Vec::new();
	for current in input {
	    if val == Bit::One {
		if current.is_this_byte_one(index) {
		    output.push(current.clone());
		}
	    } else {
		if !current.is_this_byte_one(index) {
		    output.push(current.clone());
		}
	    }
	}
	output
    }

    fn oxygen_rate(&self) -> BitNumber {

	let gamma = self.gamma_rate();
	let mut input = self.0.clone();
	for (index, bit) in gamma.data.iter().enumerate() {
	    input = Report::filter_on_index(input, index, *bit);
	    println!("input : {:?}", input);

	}

	input[0].clone()
    }
}

fn parse(filename: &str) -> Vec<BitNumber> {
    let content = fs::read_to_string(filename).expect("can't read input");
    content
        .lines()
        .into_iter()
        .map(|value| BitNumber::new(value))
        .collect()
}

fn main() {
    let numbers = parse("input");
    let report = Report::new(numbers);
    let gamma = report.gamma_rate();
    let epsilon = report.epsilon_rate();

    println!("part 1 : {}", gamma.to_u64() * epsilon.to_u64());
}

#[cfg(test)]
mod test_bitnumber {
    use super::*;

    #[test]
    fn test_new() {
        let bits = BitNumber::new("01010");
        assert_eq!(bits.data[0], Bit::Zero);
        assert_eq!(bits.data[1], Bit::One);

        let bits = BitNumber::new("010101010101101011010101");
        assert_eq!(bits.data[0], Bit::Zero);
        assert_eq!(bits.data[1], Bit::One);
    }

    #[test]
    fn test_size() {
        let bits = BitNumber::new("01010");
        assert_eq!(bits.size(), 5);
    }

    #[test]
    fn test_invert() {
	let bits = BitNumber::new("01010");
	let invert = bits.invert();

	assert_eq!(invert, BitNumber::new("10101"));
    }

    #[test]
    fn to_u64() {
	let bits = BitNumber::new("00000000");
	assert_eq!(bits.to_u64(), 0);

	let bits = BitNumber::new("00000001");
	assert_eq!(bits.to_u64(), 1);

	let bits = BitNumber::new("11111111");
	assert_eq!(bits.to_u64(), 255);

	let bits = BitNumber::new("10110");
	assert_eq!(bits.to_u64(), 22);

	let bits = BitNumber::new("01001");
	assert_eq!(bits.to_u64(), 9);
    }

    #[test]
    fn test_is_byte_one() {
	let bits = BitNumber::new("01001");
	assert!(!bits.is_this_byte_one(0));
	assert!(bits.is_this_byte_one(1))
    }
}

#[cfg(test)]
mod test_report {
    use super::*;

    #[test]
    fn test_new() {
        let report = Report::new(vec![
            BitNumber::new("01"),
            BitNumber::new("10"),
            BitNumber::new("10"),
        ]);

        assert_eq!(report.size_row(), 3);
        assert_eq!(report.size_column(), 2);
    }

    #[test]
    fn test_gamma_rate() {
        let report = Report::new(vec![
            BitNumber::new("01"),
            BitNumber::new("10"),
            BitNumber::new("10"),
        ]);
        assert_eq!(report.gamma_rate(), BitNumber::new("10"));
    }

    #[test]
    fn test_epsilon_rate() {
	let report = Report::new(vec![
            BitNumber::new("01"),
            BitNumber::new("10"),
            BitNumber::new("10"),
        ]);
        assert_eq!(report.epsilon_rate(), BitNumber::new("01"));
    }


    #[test]
    fn test_oxygen_rate() {
	let report = Report::new(vec![
	    BitNumber::new("00100"),
	    BitNumber::new("11110"),
	    BitNumber::new("10110"),
	    BitNumber::new("10111"),
	    BitNumber::new("10101"),
	    BitNumber::new("01111"),
	    BitNumber::new("00111"),
	    BitNumber::new("11100"),
	    BitNumber::new("10000"),
	    BitNumber::new("11001"),
	    BitNumber::new("00010"),
	    BitNumber::new("01010"),
	]);

	assert_eq!(report.oxygen_rate(), BitNumber::new("10111"));
    }
}
