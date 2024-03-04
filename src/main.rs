#[derive(Debug)]
struct Board {
    rows: Vec<u8>,
    cols: Vec<u8>,
    boxs: Vec<u8>,
    nums: Vec<char>,
}

impl Board {
    fn new(data: &str) -> Board {
        let mut board = Board {
            rows: vec![0; 9],
            cols: vec![0; 9],
            boxs: vec![0; 9],
            nums: data.chars().collect(),
        };

        for (i, c) in data.chars().enumerate() {
            if let Some(value) = c.to_digit(10) {
                board.rows[row(i)] |= 1 << value - 1;
                board.cols[col(i)] |= 1 << value - 1;
                board.boxs[b_x(i)] |= 1 << value - 1;
            }
        }

        board
    }
}

fn main() {
    let mut board: Board = Board::new(
        ".5.9...3.347.61..8192.35.6.73.1.8..6.1...7..9.695..8..6.5...4...7.45.6....3.8.91.",
    );

    if solve(&mut board) {
        println!("board was solved!")
    }

    println!("{:?}", board)
}

fn solve(board: &mut Board) -> bool {
    // i fucking despise how many indentations there are here
    if let Some(empty_cell) = next_empty(board) {
        if let Some(possibles) = get_possible(board, empty_cell) {
            for possible in possibles {
                fill_cell(board, empty_cell, possible);
                if solve(board) {
                    return true;
                }
                unfill_cell(board, empty_cell, possible);
            }
        }
        return false;
    }
    true
}

fn next_empty(board: &Board) -> Option<usize> {
    board.nums.iter().position(|&cell| cell == '.')
}

fn get_possible(board: &Board, cell: usize) -> Option<Vec<u8>> {
    None
}

fn fill_cell(board: &mut Board, cell: usize, value: u8) {
    board.rows[row(cell)] |= 1 << value - 1;
    board.cols[col(cell)] |= 1 << value - 1;
    board.boxs[b_x(cell)] |= 1 << value - 1;
    board.nums[cell] = char::from_digit(value as u32, 10).unwrap();
}

fn unfill_cell(board: &mut Board, cell: usize, value: u8) {
    board.rows[row(cell)] ^= 1 << value - 1;
    board.cols[col(cell)] ^= 1 << value - 1;
    board.boxs[b_x(cell)] ^= 1 << value - 1;
    board.nums[cell] = '.';
}

fn row(i: usize) -> usize {
    i / 9
}

fn col(i: usize) -> usize {
    i % 9
}

fn b_x(i: usize) -> usize {
    (i / 27 * 3) + (i % 9 / 3)
}
