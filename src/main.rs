mod board;
mod solve;
use std::time::SystemTime;

fn main() {
  let board = [
    [0, 0, 0,   0, 0, 0,    0, 0, 0],
    [0, 0, 0,   0, 0, 0,    0, 0, 0],
    [0, 0, 0,   0, 0, 0,    0, 0, 0],

    [0, 0, 0,   0, 0, 0,    0, 0, 0],
    [0, 0, 0,   0, 0, 0,    0, 0, 0],
    [0, 0, 0,   0, 0, 0,    0, 0, 0],

    [0, 0, 0,   0, 0, 0,    0, 0, 0],
    [0, 0, 0,   0, 0, 0,    0, 0, 0],
    [0, 0, 0,   0, 0, 0,    0, 0, 0]
];

  let mut b = board::Board::new(board);
  let st = SystemTime::now();
  solve::board(&mut b);
  let et = SystemTime::now();
  println!("{:?}", et.duration_since(st));
  b.show();
}