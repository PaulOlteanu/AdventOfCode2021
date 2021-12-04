use std::fmt;
use std::fs;

#[derive(Clone, Copy, Debug, Default)]
struct Position {
    value: u8,
    filled: bool,
}

struct Board {
    squares: [[Position; 5]; 5],
}

impl Board {
    fn new() -> Self {
        Board {
            squares: [[Position::default(); 5]; 5],
        }
    }

    fn set_value(&mut self, value: u8) {
        for row in self.squares.iter_mut() {
            for square in row.iter_mut() {
                if square.value == value {
                    square.filled = true;
                }
            }
        }
    }

    fn get_column(&self, n: usize) -> [Position; 5] {
        [
            self.squares[0][n],
            self.squares[1][n],
            self.squares[2][n],
            self.squares[3][n],
            self.squares[4][n],
        ]
    }

    fn get_columns(&self) -> [[Position; 5]; 5] {
        [
            self.get_column(0),
            self.get_column(1),
            self.get_column(2),
            self.get_column(3),
            self.get_column(4),
        ]
    }

    fn is_win(&self) -> bool {
        let row_win = self.squares.iter().any(|row| row.iter().all(|x| x.filled));

        let colummn_win = self
            .get_columns()
            .iter()
            .any(|column| column.iter().all(|x| x.filled));

        row_win || colummn_win
    }

    fn sum_unfilled(&self) -> u32 {
        self.squares
            .iter()
            .flat_map(|row| {
                row.map(|square| {
                    if square.filled {
                        0u32
                    } else {
                        square.value as u32
                    }
                })
            })
            .sum()
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output: Vec<String> = Vec::new();
        for row in self.squares {
            output.push(format!(
                "{:^2} {:^2} {:^2} {:^2} {:^2} | {:^5} {:^5} {:^5} {:^5} {:^5}",
                row[0].value,
                row[1].value,
                row[2].value,
                row[3].value,
                row[4].value,
                row[0].filled,
                row[1].filled,
                row[2].filled,
                row[3].filled,
                row[4].filled
            ));
        }
        write!(f, "{}", output.join("\n"))
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day_4.txt").unwrap();
    let mut input = input.lines().filter(|&x| !x.is_empty()).peekable();
    let mut numbers = input
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap());

    let mut boards: Vec<Board> = Vec::new();

    while input.peek().is_some() {
        let mut board = Board::new();

        for row in board.squares.iter_mut() {
            let numbers: Vec<u8> = input
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u8>().unwrap())
                .collect();

            for (j, square) in row.iter_mut().enumerate().take(5) {
                square.value = numbers[j];
            }
        }
        boards.push(board)
    }

    let mut number = 0;
    while !boards.iter().any(|board| board.is_win()) {
        number = numbers.next().unwrap();
        boards.iter_mut().for_each(|board| board.set_value(number));
    }

    let victor: &Board = boards
        .iter()
        .filter(|&board| board.is_win())
        .collect::<Vec<&Board>>()[0];

    println!("{:?}", victor);
    println!("Winning number: {:?}", number);
    println!("Score: {:?}", number as u32 * victor.sum_unfilled());
    println!("Finished");
}
