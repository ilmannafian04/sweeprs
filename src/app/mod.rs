use std::io::stdout;
use std::ops::{AddAssign, SubAssign};

use crossterm::event::{read, Event::Key};
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor::Hide, execute, Result};
use crossterm::{
    event::KeyCode::{Char, Down, Left, Right, Up},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::sweeper::Sweeper;

struct BoundedIndex {
    index: usize,
    max: usize,
}

impl AddAssign<usize> for BoundedIndex {
    fn add_assign(&mut self, other: usize) {
        let new = self.index.saturating_add(other);
        *self = Self {
            max: self.max,
            index: if new < self.max { new } else { self.index },
        };
    }
}

impl SubAssign<usize> for BoundedIndex {
    fn sub_assign(&mut self, other: usize) {
        *self = Self {
            max: self.max,
            index: self.index.saturating_sub(other),
        };
    }
}

pub struct App {
    i: BoundedIndex,
    j: BoundedIndex,
    sweeper: Sweeper,
}

impl App {
    pub fn new(sweeper: Sweeper) -> App {
        App {
            i: BoundedIndex {
                index: 0,
                max: sweeper.get_height(),
            },
            j: BoundedIndex {
                index: 0,
                max: sweeper.get_width(),
            },
            sweeper,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode().ok();
        execute!(stdout(), EnterAlternateScreen, Hide)?;
        loop {
            match read() {
                Ok(event) => {
                    if let Key(ke) = event {
                        execute!(stdout(), Clear(ClearType::All))?;
                        match ke.code {
                            Char('q') => break,
                            Char('a') => {
                                self.sweeper.open(self.i.index, self.j.index);
                            }
                            Char('s') => {
                                self.sweeper.flag(self.i.index, self.j.index);
                            }
                            Up => self.i -= 1,
                            Down => self.i += 1,
                            Left => self.j -= 1,
                            Right => self.j += 1,
                            _ => (),
                        }
                        println!(
                            "{:?}{:?} {} {} {:?}",
                            self.sweeper,
                            self.sweeper.game_state(),
                            self.i.index,
                            self.j.index,
                            ke.code,
                        )
                    }
                }
                Err(e) => {
                    disable_raw_mode().ok();
                    execute!(stdout(), LeaveAlternateScreen)?;
                    panic!("{}", e)
                }
            }
        }
        disable_raw_mode().ok();
        execute!(stdout(), LeaveAlternateScreen)?;
        Ok(())
    }
}
