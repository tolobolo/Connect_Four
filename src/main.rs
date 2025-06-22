fn main() {
    let game = ConnectFour::new();

    game.round()
}

struct ConnectFour {
    board: Vec<String>,
    symbol: String
}

impl ConnectFour {
    fn input(&self, message: &str) -> String {
        let mut input = String::new();

        loop {
            println!("{}", message);
            input.clear();

            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            input.trim().to_string();
        }
    }

    fn new() -> Self {
        let mut board: Vec<String> = Vec::new();
        for i in 0..20 {
            board.push(i.to_string());
        }


        let symbol = "X".to_string();

        Self {board, symbol}
    }

    fn choice_symbol(&mut self) {
        if self.symbol == "X".to_string() {
            self.symbol = "O".to_string()
        } else if self.symbol == "O".to_string() {
            self.symbol = "X".to_string()
        } else {
            println!("symbol is wrong {:?}", self.symbol)
        }
    }

    fn print_board(&self) {
        for (index, i) in self.board.iter().enumerate() {
            print!("{} ", i);
            if (index + 1) % 4 == 0 {
                println!();
            }
        }
        println!()
    }

    fn round(&self) {
        self.print_board()
    }

}
