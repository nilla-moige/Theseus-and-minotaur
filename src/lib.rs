use std::error::Error;
use std::fmt::Display;
use std::{io, vec};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameStatus {
    Win,
    Lose,
    Continue,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BoardError {
    InvalidCharacter(char),
    InvalidSize,
    NoMinotaur,
    NoTheseus,
    NoGoal,
    MultipleMinotaur,
    MultipleTheseus,
    MultipleGoal,
}
impl Display for BoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardError::InvalidCharacter(c) => write!(f, "Invalid character: {}", c),
            BoardError::InvalidSize => write!(f, "Invalid size"),
            BoardError::NoMinotaur => write!(f, "No minotaur"),
            BoardError::NoTheseus => write!(f, "No theseus"),
            BoardError::NoGoal => write!(f, "No goal"),
            BoardError::MultipleMinotaur => write!(f, "Multiple minotaur"),
            BoardError::MultipleTheseus => write!(f, "Multiple theseus"),
            BoardError::MultipleGoal => write!(f, "Multiple goal"),
        }
    }
}
impl Error for BoardError {}

#[derive(Clone)]
pub struct Grid {
    rows: vec::Vec<Vec<char>>,
    height: usize,
    width: usize,
}
impl Grid {
    pub fn new() -> Grid {
        Grid {
            rows: vec::Vec::new(),
            height: 0,
            width: 0,
        }
    }
    pub fn in_bounds(&self, row: usize, col: usize) -> bool {
        row < self.height && col < self.width
    }
    pub fn get(&self, row: usize, col: usize) -> Option<char> {
        if self.in_bounds(row, col) {
            Some(self.rows[row][col])
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Game {
    grid: Grid,
    theseus_row: usize,
    theseus_col: usize,
    minotaur_row: usize,
    minotaur_col: usize,
    goal_row: usize,
    goal_col: usize,
}

impl Game {
    // TODO: replace the function body with your implementation
    pub fn from_board(board: &str) -> Result<Game, BoardError> {
        let mut rows: Vec<Vec<char>> = Default::default();
        let mut thes_pos: Option<(usize, usize)> = None;
        let mut mino_pos: Option<(usize, usize)> = None;
        let mut goal_pos: Option<(usize, usize)> = None;
        for (r, line) in board.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            let mut row: Vec<char> = Default::default();
            for (c, ch) in line.chars().enumerate() {
                match ch {
                    ' ' | 'X' => row.push(ch),
                    'T' => {
                        if thes_pos.is_some() {
                            return Err(BoardError::MultipleTheseus);
                        }
                        thes_pos = Some((r, c));
                        row.push(' ');
                    }
                    'M' => {
                        if mino_pos.is_some() {
                            return Err(BoardError::MultipleMinotaur);
                        }
                        mino_pos = Some((r, c));
                        row.push(' ');
                    }
                    'G' => {
                        if goal_pos.is_some() {
                            return Err(BoardError::MultipleGoal);
                        }
                        goal_pos = Some((r, c));
                        row.push('G');
                    }
                    _ => return Err(BoardError::InvalidCharacter(ch)),
                }
            }
            if row.is_empty() {
                return Err(BoardError::InvalidSize);
            }
            if !rows.is_empty() && row.len() != rows[0].len() {
                return Err(BoardError::InvalidSize);
            }
            rows.push(row);
        }
        if thes_pos.is_none() {
            return Err(BoardError::NoTheseus);
        }
        if mino_pos.is_none() {
            return Err(BoardError::NoMinotaur);
        }
        if goal_pos.is_none() {
            return Err(BoardError::NoGoal);
        }
        let (theseus_row, theseus_col) = thes_pos.unwrap();
        let (minotaur_row, minotaur_col) = mino_pos.unwrap();
        let (goal_row, goal_col) = goal_pos.unwrap();
        Ok(Game {
            grid: Grid {
                rows: rows.clone(),
                height: rows.len(),
                width: rows[0].len(),
            },
            theseus_row,
            theseus_col,
            minotaur_row,
            minotaur_col,
            goal_row,
            goal_col,
        })
    }

    // TODO
    pub fn show(&self) {
        for r in 0..self.grid.height {
            for c in 0..self.grid.width {
                if self.is_theseus(r, c) {
                    print!("T");
                } else if self.is_minotaur(r, c) {
                    print!("M");
                } else {
                    match self.grid.get(r, c) {
                        Some('X') => print!("X"),
                        Some('G') => print!("G"),
                        _ => print!(" "),
                    }
                }
            }
            println!();
        }
    }

    // TODO
    pub fn minotaur_move(&mut self) {
        if self.status() != GameStatus::Continue {
            return;
        }
        let t_row = self.theseus_row as isize;
        let t_col = self.theseus_col as isize;
        let  m_row = self.minotaur_row as isize;
        let  m_col = self.minotaur_col as isize;

        let col_diff = t_col - m_col;
        let row_diff = t_row - m_row;

        if col_diff != 0 {
            let next_c = m_col + col_diff.signum();
            if self.grid.in_bounds(m_row as usize, next_c as usize)
                && self.grid.get(m_row as usize, next_c as usize) != Some('X')
            {
                self.minotaur_row = m_row as usize;
                self.minotaur_col = next_c as usize;
                return;
            }
        }

        if row_diff != 0 {
            let next_r = m_row + row_diff.signum();
            if self.grid.in_bounds(next_r as usize, m_col as usize)
                && self.grid.get(next_r as usize, m_col as usize) != Some('X')
            {
                self.minotaur_row = next_r as usize;
                self.minotaur_col = m_col as usize;
                return;
            }
        }
    }

    // TODO
    pub fn theseus_move(&mut self, command: Command) {
        let mut new_row = self.theseus_row as isize;
        let mut new_col = self.theseus_col as isize;
        match command {
            Command::Up => new_row -= 1,
            Command::Down => new_row += 1,
            Command::Left => new_col -= 1,
            Command::Right => new_col += 1,
            Command::Skip => return,
        }
        if self.grid.in_bounds(new_row as usize, new_col as usize)
            && self.grid.get(new_row as usize, new_col as usize) != Some('X')
        {
            self.theseus_row = new_row as usize;
            self.theseus_col = new_col as usize;
        }
    }

    // TODO: replace the function body with your implementation
    pub fn status(&self) -> GameStatus {
        if self.theseus_row == self.goal_row && self.theseus_col == self.goal_col {
            return GameStatus::Win;
        }
        if self.theseus_row == self.minotaur_row && self.theseus_col == self.minotaur_col {
            return GameStatus::Lose;
        }
        GameStatus::Continue
    }
}

impl Game {
    // TODO: replace the function body with your implementation
    /// Returns true if the given position is Theseus
    pub fn is_theseus(&self, row: usize, col: usize) -> bool {
        row == self.theseus_row && col == self.theseus_col
    }
    // TODO: replace the function body with your implementation
    /// Returns true if the given position is Minotaur
    pub fn is_minotaur(&self, row: usize, col: usize) -> bool {
        row == self.minotaur_row && col == self.minotaur_col
    }
    // TODO: replace the function body with your implementation
    /// Returns true if the given position is a wall
    pub fn is_wall(&self, row: usize, col: usize) -> bool {
        self.grid.get(row, col) == Some('X')
    }
    // TODO: replace the function body with your implementation
    /// Returns true if the given position is the goal
    pub fn is_goal(&self, row: usize, col: usize) -> bool {
        row == self.goal_row && col == self.goal_col
    }
    // TODO: replace the function body with your implementation
    /// Returns true if the given position is empty
    pub fn is_empty(&self, row: usize, col: usize) -> bool {
        self.grid.get(row, col) == Some(' ')
            && !self.is_theseus(row, col)
            && !self.is_minotaur(row, col)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Command {
    /// Move one tile up
    Up,
    /// Move one tile down
    Down,
    /// Move one tile left
    Left,
    /// Move one tile right
    Right,
    /// Don't move at all
    Skip,
}

//  To get a command from the user, you can use the following code:
//  ```
//  let line = stdin.lines().next().unwrap().unwrap();
//  ```
//  This will read a line from the user and store it in the `buffer` string.
//
//  Unfortunately, since stdin is line-buffered, everytime you enter a command while playing the
//  game you will have to press "enter" afterwards to send a new line.
//
//  While using the arrow keys to take inputs would be natural, it can be difficult to handle arrow
//  keys in a way that works on all devices. Therefore, it's recommended that you either use "w",
//  "a", "s", and "d" to take input, or else the words "up", "down", "left", "right". You can take
//  input however you like, so long as you document it here in a comment and it is reasonable to
//  use as a player.
pub fn input(stdin: impl io::Read + io::BufRead) -> Option<Command> {
    let line = stdin.lines().next().unwrap().unwrap();
    match line.trim().to_lowercase().as_str() {
        "w" | "up" => Some(Command::Up),
        "a" | "left" => Some(Command::Left),
        "s" | "down" => Some(Command::Down),
        "d" | "right" => Some(Command::Right),
        "skip" => Some(Command::Skip),
        _ => None,
    }
}
