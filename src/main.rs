fn main() {
    let mut game = ConnectFour::new();

    game.round()
}
#[derive(Debug, PartialEq)]
enum Location {
    Empty,
    Red,
    Yellow,
}
#[derive(Debug, PartialEq)]
enum Player {
    Red,
    Yellow,
}

struct ConnectFour {
    board: Vec<Location>,
    current_player: Player,
    nb_columns: usize,
    nb_rows: usize,
}

impl ConnectFour {
    fn new() -> Self {
        let nb_columns = 7;
        let nb_rows = 6;
        let mut board = Vec::new();
        for _ in 0..nb_rows * nb_columns {
            board.push(Location::Empty);
        }

        Self {
            board,
            current_player: Player::Red,
            nb_rows,
            nb_columns,
        }
    }

    fn input(&self, message: &str) -> usize {
        let mut input = String::new();

        println!("{}", message);
        input.clear();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let num = input.trim().parse::<usize>().expect("Failed to read line");

        if num <= self.nb_columns
            && self.board[self.nb_columns * (1 - 1) + num - 1] == Location::Empty
        {
            return num;
        } else {
            println!("Invalid input. The number '{}' is not a valid column.", num);
            return self.input(message);
        }
    }

    fn place_tile(&mut self) {
        let input: usize = self.input("whith row do you want to place in");
        let coin = match self.current_player {
            Player::Red => Location::Red,
            Player::Yellow => Location::Yellow,
        };
        for row in (1..=self.nb_rows).rev() {
            let idx = self.nb_columns * (row - 1) + input - 1;
            if self.board[idx] == Location::Empty {
                self.board[idx] = coin;
                break;
            }
        }
    }

    fn print_board(&self) {
        for i in 1..self.nb_columns + 1 {
            print!("{}", i)
        }
        println!();
        for (idx, val) in self.board.iter().enumerate() {
            match val {
                Location::Red => print!("X"),
                Location::Yellow => print!("O"),
                Location::Empty => print!("."),
            }
            if (idx + 1) % self.nb_columns == 0 {
                println!()
            }
        }
        println!();
    }

    fn choice_symbol(&mut self) {
        if self.current_player == Player::Red {
            self.current_player = Player::Yellow
        } else if self.current_player == Player::Yellow {
            self.current_player = Player::Red
        } else {
            println!("symbol is wrong {:?}", self.current_player)
        }
    }

    fn check_four_row(&self) -> bool {
        false
    }

    fn round(&mut self) {
        self.print_board();
        while !self.check_four_row() {
            self.place_tile();
            self.print_board();
            self.choice_symbol();
        }
    }
}
