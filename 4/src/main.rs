use std::{collections::HashMap, io::BufRead, str::FromStr, usize};

fn main() {
	println!(
		"Part 1 result: {}",
		parse_game(include_str!("input.txt")).part1()
	);
	println!(
		"Part 2 result: {}",
		parse_game(include_str!("input.txt")).part2()
	);
}

fn str_to_bingo_board(input: &str) -> Board {
	let mut board: Board = HashMap::new();
	let mut y = 0;
	for line in input.lines() {
		let mut x = 0;
		for row_cell in line.split_whitespace().collect::<Vec<&str>>() {
			board.insert((x, y), Cell {
				value: <usize as FromStr>::from_str(row_cell).unwrap(),
			});
			x += 1;
		}
		y += 1;
	}
	board
}

fn parse_game(input: &str) -> Bingo {
	let mut bingo = Bingo {
		boards: vec![],
		moves: input.lines().collect::<Vec<&str>>()[0]
			.split(",")
			.map(|v| <usize as FromStr>::from_str(v).unwrap())
			.collect::<Vec<usize>>(),
	};
	let mut lines = input.split("\n\n").collect::<Vec<&str>>();
	lines.remove(0);
	bingo.boards = lines
		.iter()
		.map(|board| str_to_bingo_board(board))
		.collect::<Vec<Board>>();

	bingo
}

type Board = HashMap<(usize, usize), Cell>;
type MarkedBoard = HashMap<(usize, usize), MarkedCell>;

#[derive(Debug)]
struct Cell {
	value: usize,
}

#[derive(Debug)]
struct MarkedCell {
	marked: bool,
	value: usize,
}

struct Bingo {
	boards: Vec<Board>,
	moves: Vec<usize>,
}
impl Bingo {
	fn part1(&self) -> usize {
		for i in 0..self.moves.len() {
			let current_moves = &self.moves[0..i];
			for board in &self.boards {
				let (marked, bingo_count) = has_bingo(&board, current_moves.to_vec());
				if bingo_count > 0 {
					let sum_of_unmarked =
						marked
							.values()
							.filter(|item| !item.marked)
							.fold(0, |sum, v| {
								let x = sum + v.value;
								x
							});
					return sum_of_unmarked * current_moves.last().unwrap();
				}
			}
		}
		panic!("failed to find answer");
	}

	fn part2(&self) -> usize {
		let mut boards_won: HashMap<usize, usize> = HashMap::new();
		'outer: for board in &self.boards {
			for i in 0..self.moves.len() {
				let current_moves = &self.moves[0..i];
				let (marked, bingo_count) = has_bingo(&board, current_moves.to_vec());
				if bingo_count > 0 {
					let sum_of_unmarked =
						marked
							.values()
							.filter(|item| !item.marked)
							.fold(0, |sum, v| {
								let x = sum + v.value;
								x
							});
					boards_won.insert(
						current_moves.len(),
						sum_of_unmarked * current_moves.last().unwrap(),
					);
					continue 'outer;
				}
			}
		}
		let mut biggest_key = 0;
		for (k, _) in &boards_won {
			if k > &biggest_key {
				biggest_key = *k;
			}
		}
		return *boards_won.get(&biggest_key).unwrap();
	}
}

fn has_bingo(board: &Board, values: Vec<usize>) -> (MarkedBoard, usize) {
	let mut marked: MarkedBoard = HashMap::new();
	for (key, val) in board {
		for v in &values {
			let key = (key.0, key.1);
			let found = marked.get(&key);
			if !found.is_some() || (found.is_some() && !found.unwrap().marked) {
				marked.insert(
					key,
					MarkedCell {
						marked: val.value == *v,
						value: val.value,
					},
				);
			}
		}
	}
	let mut bingo_counter = 0;
	for i in 0..5 {
		if marked
			.iter()
			.filter(|&(key, val)| val.marked && (key.0 >= 0 && key.0 <= 4) && key.1 == i)
			.count() >= 5
		{
			bingo_counter += 1;
		}
	}
	for i in 0..5 {
		if marked
			.iter()
			.filter(|&(key, val)| val.marked && (key.1 >= 0 && key.1 <= 4) && key.0 == i)
			.count() >= 5
		{
			bingo_counter += 1;
		}
	}

	(marked, bingo_counter)
}

#[test]
fn test() {
	let b = &str_to_bingo_board(
		"22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19",
	);

	//check horizontal
	{
		let (_, a) = has_bingo(b, vec![22, 13, 17, 11, 0]);
		assert_eq!(a, 1);
	}
	{
		let (_, a) = has_bingo(b, vec![22, 13, 17, 11, 0]);
		assert_eq!(a, 1);
	}
	{
		let (_, a) = has_bingo(b, vec![22, 13, 17, 11, 0, 8, 2, 23, 4, 24]);
		assert_eq!(a, 2);
	}
	{
		// // vertical
		let (_, a) = has_bingo(b, vec![22, 8, 21, 6, 1]);
		assert_eq!(a, 1);
	}
	{
		// both
		let (_, a) = has_bingo(b, vec![22, 8, 21, 6, 1, 8, 2, 23, 4, 24]);
		assert_eq!(a, 2);
	}

	let game = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
	let bingo = parse_game(game);
	assert_eq!(bingo.part1(), 4512);
}
