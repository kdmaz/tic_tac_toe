use std::{
    error::Error,
    fmt::{write, Display},
};

#[derive(Copy, Clone, PartialEq)]
pub enum PlayerMove {
    X,
    O,
}

impl PlayerMove {
    fn toggle(player_move: Self) -> Self {
        match player_move {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }
}

#[derive(Clone, Copy)]
pub enum SquarePosition {
    TopLeft,
    TopMiddle,
    TopRight,
    CenterLeft,
    CenterMiddle,
    CenterRight,
    BottomLeft,
    BottomMiddle,
    BottomRight,
}

impl From<SquarePosition> for usize {
    fn from(position: SquarePosition) -> Self {
        match position {
            SquarePosition::TopLeft => 0,
            SquarePosition::TopMiddle => 1,
            SquarePosition::TopRight => 2,
            SquarePosition::CenterLeft => 3,
            SquarePosition::CenterMiddle => 4,
            SquarePosition::CenterRight => 5,
            SquarePosition::BottomLeft => 6,
            SquarePosition::BottomMiddle => 7,
            SquarePosition::BottomRight => 8,
        }
    }
}

#[derive(Copy, Clone)]
struct Square(Option<PlayerMove>);

impl Square {
    fn try_make_move(&mut self, player_move: PlayerMove) -> Result<(), SquareAlreadyTaken> {
        if self.0.is_none() {
            self.0 = Some(player_move);
            Ok(())
        } else {
            Err(SquareAlreadyTaken)
        }
    }
}

#[derive(Debug)]
pub struct SquareAlreadyTaken;

impl Error for SquareAlreadyTaken {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl std::fmt::Display for SquareAlreadyTaken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Square already taken!")
    }
}

pub enum Status {
    InProgress,
    Complete,
}

pub struct Board {
    squares: [Square; 9],
    player_turn: PlayerMove,
    status: Status,
}

impl Board {
    pub fn new() -> Board {
        Board {
            squares: [Square(None); 9],
            player_turn: PlayerMove::X,
            status: Status::InProgress,
        }
    }

    pub fn get_player_turn(&self) -> &PlayerMove {
        &self.player_turn
    }

    pub fn get_status(&self) -> &Status {
        &self.status
    }

    pub fn make_move(&mut self, position: SquarePosition) -> Result<Status, SquareAlreadyTaken> {
        self.squares[usize::from(position)].try_make_move(self.player_turn)?;
        self.player_turn = PlayerMove::toggle(self.player_turn);

        Ok(if self.has_winner() {
            Status::Complete
        } else {
            Status::InProgress
        })
    }

    fn has_winner(&self) -> bool {
        let top_row = self.has_winner_in((
            SquarePosition::TopLeft,
            SquarePosition::TopMiddle,
            SquarePosition::TopRight,
        ));

        let center_row = self.has_winner_in((
            SquarePosition::CenterLeft,
            SquarePosition::CenterMiddle,
            SquarePosition::CenterRight,
        ));

        let bottom_row = self.has_winner_in((
            SquarePosition::BottomLeft,
            SquarePosition::BottomMiddle,
            SquarePosition::BottomRight,
        ));

        let left_column = self.has_winner_in((
            SquarePosition::TopLeft,
            SquarePosition::TopMiddle,
            SquarePosition::TopRight,
        ));

        let center_column = self.has_winner_in((
            SquarePosition::CenterLeft,
            SquarePosition::CenterMiddle,
            SquarePosition::CenterRight,
        ));

        let right_column = self.has_winner_in((
            SquarePosition::BottomLeft,
            SquarePosition::BottomMiddle,
            SquarePosition::BottomRight,
        ));

        let major_diagonal = self.has_winner_in((
            SquarePosition::TopLeft,
            SquarePosition::CenterMiddle,
            SquarePosition::BottomRight,
        ));

        let minor_diagonal = self.has_winner_in((
            SquarePosition::BottomLeft,
            SquarePosition::CenterMiddle,
            SquarePosition::TopRight,
        ));

        let row = top_row || center_row || bottom_row;
        let column = left_column || center_column || right_column;
        let diagonal = major_diagonal || minor_diagonal;

        row || column || diagonal
    }

    fn has_winner_in(&self, (a, b, c): (SquarePosition, SquarePosition, SquarePosition)) -> bool {
        if self.squares[usize::from(a)].0.is_none()
            || self.squares[usize::from(b)].0.is_none()
            || self.squares[usize::from(c)].0.is_none()
        {
            false
        } else {
            self.squares[usize::from(a)].0 == self.squares[usize::from(b)].0
                && self.squares[usize::from(a)].0 == self.squares[usize::from(c)].0
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!();
        // self.squares
        // write!(f, "")
    }
}
