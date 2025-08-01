fn main() {
    let mut game = ConnectFour::new();

    game.round()
}
#[derive(Debug, PartialEq, Clone, Copy)]
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

impl Player {
    fn as_coin(&self) -> Location {
        match self {
            Player::Red => Location::Red,
            Player::Yellow => Location::Yellow,
        }
    }
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

    fn new_with_state(board: Vec<Location>) -> Self {
        let nb_columns = 7;
        let nb_rows = 6;
        let board = board;

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

        match input.trim().parse::<usize>() {
            Ok(num) => {
                if num <= self.nb_columns
                    && self.board[self.nb_columns * (1 - 1) + num - 1] == Location::Empty
                {
                    return num;
                } else {
                    println!("Invalid input. The number '{}' is not a valid column.", num);
                    return self.input(message);
                }
            }
            Err(_) => {
                println!("that is not a number");
                return self.input(message);
            }
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

    fn set_next_player(&mut self) {
        if self.current_player == Player::Red {
            self.current_player = Player::Yellow
        } else if self.current_player == Player::Yellow {
            self.current_player = Player::Red
        } else {
            println!("symbol is wrong {:?}", self.current_player)
        }
    }

    fn check_current_player_has_won(&mut self) -> bool {
        self.has_won(&self.current_player)
    }

    fn has_won(&self, player: &Player) -> bool {
        if self.has_won_vertical(&player) {
            return true;
        }
        if self.has_won_horizontal(&player) {
            return true;
        }
        if self.has_won_diagonal(&player) {
            return true;
        }
        return false;
    }

    fn has_won_diagonal(&self, player: &Player) -> bool {
        let coin = player.as_coin();
        for idx in 0..self.board.len() - 3 * self.nb_columns {
            let mut ajecent_coin = 0;
            //dbg!((idx..idx + 4 * self.nb_columns + 3).step_by(self.nb_columns + 1));
            if idx % self.nb_columns < self.nb_columns - 3 {
                for i in (idx..idx + 3 * self.nb_columns + 4).step_by(self.nb_columns + 1) {
                    if self.board[i] == coin {
                        ajecent_coin += 1
                    }
                }
            } else {
                for i in (idx..idx + 3 * self.nb_columns - 2).step_by(self.nb_columns - 1) {
                    if self.board[i] == coin {
                        ajecent_coin += 1
                    }
                }
            }
            if ajecent_coin == 4 {
                println!(" {:?} has won", player);
                return true;
            }
        }
        return false;
    }

    fn has_won_horizontal(&self, player: &Player) -> bool {
        let coin = player.as_coin();
        for row in 0..self.nb_rows {
            for idx in row * self.nb_columns..(row + 1) * self.nb_columns - 3 {
                if self.board[idx..(idx + 4)].iter().all(|tile| *tile == coin) {
                    println!(" {:?} has won", player);
                    return true;
                }
            }
        }
        false
    }
    fn has_won_vertical(&self, player: &Player) -> bool {
        let coin = player.as_coin();
        let mut ajecent_coin: usize;
        for idx in 0..self.board.len() - 3 * self.nb_columns {
            ajecent_coin = 0;

            for i in (idx..idx + 4 * self.nb_columns).step_by(self.nb_columns) {
                if self.board[i] == coin {
                    ajecent_coin += 1
                }
            }

            if ajecent_coin == 4 {
                println!(" {:?} has won", player);
                return true;
            }
        }
        false
    }

    fn round(&mut self) {
        self.print_board();
        while !self.check_current_player_has_won() {
            self.set_next_player();
            self.place_tile();
            self.print_board();
        }
    }
}

#[cfg(test)]
#[warn(unused_variables)]
mod tests {
    use all_asserts::{assert_false, assert_true};

    use crate::{ConnectFour, Location, Player};

    #[test]
    fn test_empty() {
        let e = Location::Empty;
        let cf = ConnectFour::new_with_state(vec![
            e, e, e, e, e, e, e, //
            e, e, e, e, e, e, e, //
            e, e, e, e, e, e, e, //
            e, e, e, e, e, e, e, //
            e, e, e, e, e, e, e, //
            e, e, e, e, e, e, e, //
        ]);
        assert_false!(cf.has_won(&Player::Red));
        assert_false!(cf.has_won(&Player::Red));
    }

    #[test]
    fn test_horizontal_top_left() {
        let e = Location::Empty;
        let y = Location::Yellow;
        let r = Location::Red;
        let cf = ConnectFour::new_with_state(vec![
            r, r, r, r, y, y, y, //
            e, e, e, e, e, e, e, //
            e, e, e, e, e, e, e, //
            e, e, e, e, e, e, e, //
            e, e, e, e, e, e, e, //
            e, e, e, e, e, e, e, //
        ]);
        assert_false!(cf.has_won_vertical(&Player::Red));
        assert_false!(cf.has_won_vertical(&Player::Yellow));
        assert_true!(cf.has_won_horizontal(&Player::Red));
        assert_false!(cf.has_won_horizontal(&Player::Yellow));
    }

    #[test]
    fn test_horizontal_top_right() {
        let e = Location::Empty;
        let y = Location::Yellow;
        let r = Location::Red;
        let cf = ConnectFour::new_with_state(vec![
            r, r, r, y, y, y, y, //
            e, e, e, e, e, e, e, //
            e, e, e, e, e, e, e, //
            e, e, e, e, e, e, e, //
            e, e, e, e, e, e, e, //
            e, e, e, e, e, e, e, //
        ]);
        assert_false!(cf.has_won_vertical(&Player::Red));
        assert_false!(cf.has_won_vertical(&Player::Yellow));
        assert_false!(cf.has_won_horizontal(&Player::Red));
        assert_true!(cf.has_won_horizontal(&Player::Yellow));
    }

    #[test]
    fn test_diagonal() {
        let e = Location::Empty;
        let r = Location::Red;
        let y = Location::Yellow;
        let cf = ConnectFour::new_with_state(vec![
            e, e, r, e, e, e, e, //
            e, e, e, r, e, e, e, //
            e, e, e, e, r, y, e, //
            e, e, e, e, y, r, e, //
            e, e, e, y, e, e, e, //
            r, e, e, y, y, e, e, //
        ]);
        assert_true!(cf.has_won_diagonal(&Player::Red));
        assert_false!(cf.has_won_diagonal(&Player::Yellow));
    }
    #[test]
    fn test_horizontal_bottom_right() {
        let e = Location::Empty;
        let r = Location::Red;
        let y = Location::Yellow;
        let cf = ConnectFour::new_with_state(vec![
            e, e, e, e, y, y, y, //
            e, e, e, e, e, e, e, //
            e, e, e, e, e, e, e, //
            e, e, e, e, e, e, e, //
            e, e, e, e, e, e, e, //
            e, e, r, r, r, r, r, //
        ]);
        assert_true!(cf.has_won_horizontal(&Player::Red));
        assert_false!(cf.has_won_horizontal(&Player::Yellow));
        assert_false!(cf.has_won_vertical(&Player::Red));
        assert_false!(cf.has_won_vertical(&Player::Yellow));
    }

    #[test]
    fn test_horizontal_bottom_left() {
        let e = Location::Empty;
        let r = Location::Red;
        let y = Location::Yellow;
        let cf = ConnectFour::new_with_state(vec![
            e, e, e, e, y, y, y, //
            e, e, e, e, e, e, e, //
            e, e, e, e, e, e, e, //
            e, e, e, e, e, e, e, //
            e, e, e, e, e, e, e, //
            e, e, r, r, r, r, r, //
        ]);
        assert_true!(cf.has_won_horizontal(&Player::Red));
        assert_false!(cf.has_won_horizontal(&Player::Yellow));
        assert_false!(cf.has_won_vertical(&Player::Red));
        assert_false!(cf.has_won_vertical(&Player::Yellow));
    }

    #[test]
    fn test_vertical() {
        let e = Location::Empty;
        let y = Location::Yellow;
        let cf = ConnectFour::new_with_state(vec![
            e, e, e, e, e, e, e, //
            e, e, e, e, e, e, e, //
            y, e, e, e, e, e, e, //
            y, e, e, e, e, e, e, //
            y, e, e, e, e, e, e, //
            y, e, e, e, e, e, e, //
        ]);
        assert_false!(cf.has_won_vertical(&Player::Red));
        assert_true!(cf.has_won_vertical(&Player::Yellow));
        assert_false!(cf.has_won_horizontal(&Player::Red));
        assert_false!(cf.has_won_horizontal(&Player::Yellow));
    }
}
