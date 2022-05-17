use crate::board::{Board, PlayerMove, SquarePosition, Status};

pub struct Game;

impl Game {
    pub fn new() {
        loop {
            let mut board = Board::new();

            'in_progress: loop {
                println!("{}", board);
                let turn = board.get_player_turn().clone();
                let position = Game::get_player_move_position(&turn);

                match board.make_move(position) {
                    Ok(Status::Complete) => {
                        println!("{}", board);
                        println!("\nPlayer \"{}\" wins!\n", turn);
                        break 'in_progress;
                    }
                    Ok(Status::Draw) => {
                        println!("{}", board);
                        println!("\nDraw!\n");
                        break 'in_progress;
                    }
                    Ok(Status::InProgress) => continue 'in_progress,
                    Err(err) => {
                        println!("{}", err);
                        continue 'in_progress;
                    }
                }
            }

            if !Game::play_again() {
                break;
            }
        }
    }

    fn play_again() -> bool {
        println!("Play again? (y/n)");
        let mut res = String::new();
        std::io::stdin().read_line(&mut res).unwrap();
        res.trim() == "y"
    }

    fn get_player_move_position(turn: &PlayerMove) -> SquarePosition {
        println!(
            "\
            Player \"{}\" to move!\n
top left    (1)  top middle    (2)     top right (3)
center left (4)  center middle (5)  center right (6)
bottom left (7)  bottom middle (8)  bottom right (9)
            ",
            turn
        );

        loop {
            let mut position = String::new();
            std::io::stdin().read_line(&mut position).unwrap();

            let num = match position.trim().parse::<usize>() {
                Ok(num) => num,
                Err(_) => {
                    println!("\nChoose a valid square!\n");
                    continue;
                }
            };

            match SquarePosition::try_from(num) {
                Ok(position) => return position,
                Err(_) => {
                    println!("\nChoose a valid square!\n");
                    continue;
                }
            };
        }
    }
}
