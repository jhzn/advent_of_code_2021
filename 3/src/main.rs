fn main() {
	println!(
		"Part 1 result: {:?}",
		part1(include_str!("input.txt").lines().collect())
	);

	println!(
		"Part 2 result: {:?}",
		part2(include_str!("input.txt").lines().collect())
	);
}

#[derive(Debug, PartialEq)]
enum Binary {
	Zero,
	One,
	Equal,
}

fn char_to_common(c: char) -> Option<Binary> {
	match c {
		'0' => Some(Binary::Zero),
		'1' => Some(Binary::One),
		_ => None,
	}
}

fn least_common(input: &str) -> Binary {
	let _0m = input.matches('0').collect::<Vec<&str>>().len();
	let _1m = input.matches('1').collect::<Vec<&str>>().len();

	if _1m == _0m {
		return Binary::Equal;
	}
	if _1m < _0m {
		return Binary::One;
	}
	Binary::Zero
}

fn most_common(input: &str) -> Binary {
	let _0m = input.matches('0').collect::<Vec<&str>>().len();
	let _1m = input.matches('1').collect::<Vec<&str>>().len();

	if _1m == _0m {
		return Binary::Equal;
	}
	if _1m < _0m {
		return Binary::Zero;
	}
	Binary::One
}

fn part1(input: Vec<&str>) -> i32 {
	let mut gamma_rate: String = "".to_string();
	let mut epsilon_rate: String = "".to_string();
	for index in 0..input[0].len() {
		let column: String = input
			.iter()
			.map(|x| x.chars().nth(index).unwrap())
			.collect();
		gamma_rate += match &most_common(&column) {
			Binary::Zero => "0",
			Binary::One => "1",
			_ => panic!("invalid resposnse"),
		};
		epsilon_rate += match &most_common(&column) {
			Binary::Zero => "1",
			Binary::One => "0",
			_ => panic!("invalid resposnse"),
		};
	}
	i32::from_str_radix(&epsilon_rate, 2).unwrap() * i32::from_str_radix(&gamma_rate, 2).unwrap()
}

enum CompareOp {
	LeastCommonBinary,
	MostCommonBinary,
}

fn part2_calculate(mut input: Vec<&str>, compare: CompareOp) -> i32 {
	let mut current_index = 0;
	for index in 0..input[0].len() {
		let column: String = input
			.iter()
			.map(|x| x.chars().nth(index).unwrap())
			.collect();

		let (most_com_binary, bin_comp) = match compare {
			CompareOp::MostCommonBinary => (most_common(&column), Binary::One),
			CompareOp::LeastCommonBinary => (least_common(&column), Binary::Zero),
		};

		let filtered: Vec<&str> = input
			.iter()
			.filter(|binary_char| {
				match char_to_common(binary_char.chars().nth(current_index).unwrap()) {
					Some(binary) => {
						if most_com_binary == Binary::Equal && binary == bin_comp {
							return true;
						}
						most_com_binary == binary
					}
					_ => false,
				}
			})
			.cloned()
			.collect();

		if filtered.len() == 0 {
			break;
		}

		input = filtered;
		current_index += 1;
	}
	i32::from_str_radix(&input.iter().nth(0).unwrap(), 2).unwrap()
}

fn part2(input: Vec<&str>) -> i32 {
	part2_calculate(input.clone(), CompareOp::MostCommonBinary)
		* part2_calculate(input.clone(), CompareOp::LeastCommonBinary)
}

#[test]
fn test() {
	let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
	assert_eq!(part1(input.lines().collect()), 198);
	assert_eq!(part2(input.lines().collect()), 230);
}
