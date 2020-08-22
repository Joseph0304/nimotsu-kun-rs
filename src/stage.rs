use std::str::FromStr;
use thiserror::Error;

enum State {
    Nothing,
    Wall,
    Hall,
    Block,
    BlockOnHall,
    Player,
    PlayerOnHall,
}

#[derive(Error, Debug)]
pub enum ParseStageError {
    #[error("Stage width validation error: Must be length {0}")]
    StageWidthValidationError(usize),
    #[error("Unknown stage object")]
    UnknownObjectError,
}

pub struct Stage {
    stage: Vec<State>,
    w: usize,
    h: usize,
}

impl Stage {
    pub fn width(&self) -> usize {
        self.w
    }

    pub fn height(&self) -> usize {
        self.h
    }

    pub fn check_clear(&self) -> bool {
        !self.stage.iter().any(|x| match x {
            State::Hall | State::PlayerOnHall => true,
            _ => false,
        })
    }

    pub fn update(&mut self, dx: i32, dy: i32) {
        let h = self.height();
        let w = self.width();
        let player_position = self
            .stage
            .iter()
            .position(|x| match x {
                State::Player | State::PlayerOnHall => true,
                _ => false,
            })
            .unwrap();
        let x = player_position % w;
        let y = player_position / w;

        if x as i32 + dx >= 0
            && x as i32 + dx < w as i32 - 1
            && y as i32 + dy >= 0
            && y as i32 + dy < h as i32 - 1
        {
            let pos = (player_position as i32 + dy * w as i32 + dx) as usize;
            self.stage[pos] = match self.stage[pos] {
                State::Wall => {
                    return;
                }
                State::Hall => State::PlayerOnHall,
                State::Nothing => State::Player,
                State::Block | State::BlockOnHall => {
                    let second_pos = (player_position as i32 + 2 * dy * w as i32 + 2 * dx) as usize;
                    self.stage[second_pos] = match self.stage[second_pos] {
                        State::Wall | State::Block | State::BlockOnHall => {
                            return;
                        }
                        State::Nothing => State::Block,
                        State::Hall => State::BlockOnHall,
                        State::Player | State::PlayerOnHall => {
                            panic!("move block position must not be player!");
                        }
                    };
                    match self.stage[pos] {
                        State::Block => State::Player,
                        State::BlockOnHall => State::PlayerOnHall,
                        _ => {
                            panic!("stage[pos] is not block");
                        }
                    }
                }
                State::Player | State::PlayerOnHall => {
                    panic!("this position must not be player!");
                }
            };
            self.stage[player_position] = match self.stage[player_position] {
                State::PlayerOnHall => State::Hall,
                State::Player => State::Nothing,
                _ => {
                    panic!("this position must be player!");
                }
            };
        }
    }
}

impl FromStr for Stage {
    type Err = ParseStageError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split('\n').collect();
        let w = v.first().unwrap().len();
        if !v.iter().all(|&s| s.len() == w) {
            return Err(ParseStageError::StageWidthValidationError(w));
        }
        let stage = v.iter().map(|s| s.chars()).flatten().collect::<String>();
        if stage.as_str().chars().any(|x| match x {
            '#' | ' ' | '.' | 'o' | 'O' | 'p' | 'P' => false,
            _ => true,
        }) {
            return Err(ParseStageError::UnknownObjectError);
        }
        Ok(Stage {
            stage: stage
                .chars()
                .map(|c| match c {
                    '#' => State::Wall,
                    ' ' => State::Nothing,
                    'o' => State::Block,
                    'p' => State::Player,
                    '.' => State::Hall,
                    'O' => State::BlockOnHall,
                    'P' => State::PlayerOnHall,
                    _ => {
                        panic!("unknown object error!");
                    }
                })
                .collect(),
            w,
            h: v.len(),
        })
    }
}

impl ToString for Stage {
    fn to_string(&self) -> String {
        self.stage
            .iter()
            .map(|o| match o {
                State::Nothing => ' ',
                State::Wall => '#',
                State::Hall => '.',
                State::Player => 'p',
                State::PlayerOnHall => 'P',
                State::Block => 'o',
                State::BlockOnHall => 'O',
            })
            .collect::<String>()
    }
}
