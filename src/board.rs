use std::{error::Error, fmt::Display};

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

impl Display for PlayerMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PlayerMove::X => "X",
                PlayerMove::O => "O",
            }
        )
    }
}

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

impl TryFrom<usize> for SquarePosition {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(SquarePosition::TopLeft),
            2 => Ok(SquarePosition::TopMiddle),
            3 => Ok(SquarePosition::TopRight),
            4 => Ok(SquarePosition::CenterLeft),
            5 => Ok(SquarePosition::CenterMiddle),
            6 => Ok(SquarePosition::CenterRight),
            7 => Ok(SquarePosition::BottomLeft),
            8 => Ok(SquarePosition::BottomMiddle),
            9 => Ok(SquarePosition::BottomRight),
            _ => Err(()),
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

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let player_move = match self.0 {
            Some(PlayerMove::X) => "X",
            Some(PlayerMove::O) => "O",
            None => " ",
        };

        write!(f, "{}", player_move)
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
        write!(f, "\nSquare already taken!\n")
    }
}

pub enum Status {
    InProgress,
    Draw,
    Complete,
}

pub struct Board {
    squares: [Square; 9],
    player_turn: PlayerMove,
    turn_num: u8,
}

impl Board {
    pub fn new() -> Board {
        Board {
            squares: [Square(None); 9],
            player_turn: PlayerMove::X,
            turn_num: 0,
        }
    }

    pub fn get_player_turn(&self) -> &PlayerMove {
        &self.player_turn
    }

    pub fn make_move(&mut self, position: SquarePosition) -> Result<Status, SquareAlreadyTaken> {
        self.squares[usize::from(position)].try_make_move(self.player_turn)?;
        self.player_turn = PlayerMove::toggle(self.player_turn);
        self.turn_num += 1;

        Ok(if self.has_winner() {
            Status::Complete
        } else if self.turn_num > 8 {
            Status::Draw
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
            SquarePosition::CenterLeft,
            SquarePosition::BottomLeft,
        ));

        let center_column = self.has_winner_in((
            SquarePosition::TopMiddle,
            SquarePosition::CenterMiddle,
            SquarePosition::BottomMiddle,
        ));

        let right_column = self.has_winner_in((
            SquarePosition::TopRight,
            SquarePosition::CenterRight,
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
        let a = self.squares[usize::from(a)].0;
        let b = self.squares[usize::from(b)].0;
        let c = self.squares[usize::from(c)].0;
        if a.is_none() || b.is_none() || c.is_none() {
            false
        } else {
            a == b && a == c
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board = format!(
            "\n
     |     |     
  {}  |  {}  |  {}  
_____|_____|_____
     |     |     
  {}  |  {}  |  {}  
_____|_____|_____
     |     |     
  {}  |  {}  |  {}  
     |     |     \n
            ",
            self.squares[0],
            self.squares[1],
            self.squares[2],
            self.squares[3],
            self.squares[4],
            self.squares[5],
            self.squares[6],
            self.squares[7],
            self.squares[8],
        );
        write!(f, "{}", board)
    }
}
