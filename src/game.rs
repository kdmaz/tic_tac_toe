use crate::board::{Board, SquarePosition, Status};

pub struct Game;

impl Game {
    pub fn new() {
        // let stats = {};

        loop {
            let mut board = Board::new();
            println!("{}", board);

            'in_progress: loop {
                let position = Game::get_player_move_position();

                match board.make_move(position) {
                    Ok(Status::Complete) => break 'in_progress,
                    Ok(Status::InProgress) => continue 'in_progress,
                    Err(err) => {
                        println!("{}", err);
                        continue 'in_progress;
                    }
                }
            }

            // play_again
        }
    }

    pub fn get_player_move_position() -> SquarePosition {
        loop {}
    }
}
