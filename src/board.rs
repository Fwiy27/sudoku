pub struct Board {
    pub board: [[u8; 9]; 9],
    pub reserved: Vec<(u8, u8)>
}

fn get_reserved(board: [[u8; 9]; 9]) -> Vec<(u8, u8)> {
    let mut reserved = Vec::new();
  
    for row in 0..board.len() {
      for col in 0..board[row].len() {
        let number = board[row][col];
        if number != 0 {
          reserved.push((row as u8, col as u8));
        }
      }
    }
    return reserved;
}

impl Board {
    pub fn new(board: [[u8; 9]; 9]) -> Board{
        let reserved = get_reserved(board);
        return Board {
            board: board,
            reserved: reserved
        };
    }
    pub fn show(&self) {
        for (i, line) in self.board.iter().enumerate() {
            if i % 3 == 0 {
                println!("-------------");
            }
            for (n, number) in line.iter().enumerate() {
                if n % 3 == 0 {
                    print!("|");
                }
                print!("{}", number);
            }
            println!("|");
        }
        println!("-------------");
    }

    pub fn complete(&self) -> bool {
        for line in self.board {
            if line.contains(&0) {
                return false
            }
        }
        if !self.check() {
            return false
        }
        return true
    }

    pub fn check(&self) -> bool {
        let board = self.board;

        let mut numbers = Vec::new();
        
        // Horizontal
        for row in board {
            numbers.clear();
            for number in row {
                if number != 0 {
                    if numbers.contains(&number) {
                    return false;
                    } else {
                    numbers.push(number);
                    }
                }
            }
        }
        
        // Vertical
        for col in 0..board[0].len() {
            numbers.clear();
            for row in 0..board.len() {
                let number: u8 = board[row][col];
                if number != 0 {
                    if numbers.contains(&number) {
                        return false;
                    } else {
                        numbers.push(number);
                    }
                }
            }
        }
        
        // Groups of 9
        for x in 0..3 {
            for n in 0..3 {
                numbers.clear();
                for i in x*3..x*3+3 {
                    for e in n*3..n*3+3 {
                        let number = board[i][e];
                        if number != 0 {
                            if numbers.contains(&number) {
                                return false;
                            } else {
                                numbers.push(number);
                            }
                        }
                    }
                }
            }
        }
        return true;
    }
}