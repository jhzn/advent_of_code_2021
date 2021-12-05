use std::str::FromStr;

fn main() {
	let input = parse(include_str!("input.txt"));
	println!("Part 1 result: {:?}", part1(&input));
	println!("Part 2 result: {:?}", part2(&input));
}

fn parse(input: &str) -> Vec<&str> {
	input.split_terminator("\n").collect()
}

#[derive(Debug)]
struct PosPart1 {
	x: i32,
	y: i32,
}

fn part1(input: &Vec<&str>) -> i32 {
	let mut out: Vec<PosPart1> = vec![];
	for ele in input {
		let sp = ele.split(" ").collect::<Vec<&str>>();
		match sp[0] {
			"up" => out.push(PosPart1 {
				x: 0,
				y: <i32 as FromStr>::from_str(sp[1]).unwrap(),
			}),
			"down" => out.push(PosPart1 {
				x: 0,
				y: <i32 as FromStr>::from_str(sp[1]).unwrap() * -1,
			}),
			"forward" => out.push(PosPart1 {
				x: <i32 as FromStr>::from_str(sp[1]).unwrap(),
				y: 0,
			}),
			_ => panic!("banan"),
		}
	}
	let mut result = PosPart1 { x: 0, y: 0 };
	for ele in out {
		result.x = result.x + ele.x;
		result.y = result.y + ele.y;
	}
	result.x * result.y * -1
}

#[derive(Debug)]
struct PosPart2 {
	x: i32,
	y: i32,
	aim: i32,
}
fn part2(input: &Vec<&str>) -> i32 {
	fn sum_pos(p: &Vec<PosPart2>) -> PosPart2 {
		let mut result = PosPart2 { x: 0, y: 0, aim: 0 };
		for ele in p {
			result.x = result.x + ele.x;
			result.y = result.y + ele.y;
			result.aim = result.aim + ele.aim;
		}
		result
	}
	let mut out: Vec<PosPart2> = vec![];
	for ele in input {
		let sp = ele.split(" ").collect::<Vec<&str>>();
		match sp[0] {
			"up" => out.push(PosPart2 {
				x: 0,
				y: 0,
				aim: <i32 as FromStr>::from_str(sp[1]).unwrap() * -1,
			}),
			"down" => out.push(PosPart2 {
				x: 0,
				y: 0,
				aim: <i32 as FromStr>::from_str(sp[1]).unwrap(),
			}),
			"forward" => {
				let sum = sum_pos(&out);
				let x = <i32 as FromStr>::from_str(sp[1]).unwrap();
				out.push(PosPart2 {
					x,
					y: sum.aim * x,
					aim: 0,
				})
			}
			_ => panic!("banan"),
		}
	}
	let sum = sum_pos(&out);
	println!("{:?}", sum);
	sum.x * sum.y
}

#[test]
fn test() {
	let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2
";
	assert_eq!(part1(&parse(input)), 150);
	assert_eq!(part2(&parse(input)), 900);
}
