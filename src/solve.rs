use crate::board::Board;

fn index_to_coordinate(index: u8) -> (u8, u8) {
    return (index / 9 as u8, index % 9 as u8);
}

pub fn board(board: &mut Board) {
    let mut index = 0;

    // false for backwards, true for forwards
    let mut direction = true;

    while !board.complete() {
        let coordinates = index_to_coordinate(index);
        if board.reserved.contains(&coordinates) {
            match direction {
                false => index -= 1,
                true => index += 1
            };
        } else {
            board.board[coordinates.0 as usize][coordinates.1 as usize] = board.board[coordinates.0 as usize][coordinates.1 as usize] + 1;

            let number = board.board[coordinates.0 as usize][coordinates.1 as usize];

            let allowed = board.check();

            if (number == 9 && !allowed) || (number > 9) {
                direction = false;
                board.board[coordinates.0 as usize][coordinates.1 as usize] = 0;
                index -= 1;
            } else if number == 0 {
                board.board[coordinates.0 as usize][coordinates.1 as usize] = 1;
            } else if allowed {
                direction = true;
                index += 1;
            }
        }
    }
}