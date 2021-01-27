#![allow(dead_code)]
use std::ops::{AddAssign, SubAssign};
use std::{
    io::{BufWriter, Stdout, Write},
    u16,
};

use crossterm::{
    cursor,
    event::KeyCode::{Char, Down, Left, Right, Up},
    event::{read, DisableMouseCapture, EnableMouseCapture, Event::Key},
    queue,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    QueueableCommand, Result,
};

use crate::sweeper::Sweeper;

type W = BufWriter<Stdout>;

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

pub struct App<'a> {
    i: BoundedIndex,
    j: BoundedIndex,
    sweeper: Sweeper,
    w: &'a mut W,
}

impl<'a> App<'a> {
    pub fn new(sweeper: Sweeper, w: &'a mut W) -> Self {
        Self {
            i: BoundedIndex {
                index: 0,
                max: sweeper.get_height(),
            },
            j: BoundedIndex {
                index: 0,
                max: sweeper.get_width(),
            },
            sweeper,
            w,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        queue!(self.w, EnterAlternateScreen, cursor::Hide)?;
        enable_raw_mode()?;
        self.w.flush()?;
        self.draw()?;

        loop {
            match read() {
                Ok(event) => {
                    if let Key(ke) = event {
                        queue!(self.w, Clear(ClearType::All))?;
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
                    }
                }
                Err(e) => {
                    self.tear_down().ok();
                    panic!("{}", e)
                }
            }
        }
        self.tear_down()?;
        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        self.draw_border()?;
        self.draw_board()?;
        print!("\n\r");
        Ok(())
    }

    fn draw_border(&mut self) -> Result<()> {
        let (height, width) = (
            self.sweeper.get_height() as u16,
            self.sweeper.get_width() as u16,
        );
        self.w.queue(Print("┌"))?;
        for _ in 0..width {
            queue!(self.w, Print("──"))?;
        }
        self.w.queue(Print("┐"))?;
        for i in [0_u16, width * 2 + 1].iter() {
            self.w.queue(cursor::MoveTo(*i, 0))?;
            for _ in 0..height {
                queue!(self.w, cursor::MoveDown(1), Print("│"), cursor::MoveLeft(1))?;
            }
        }
        queue!(self.w, cursor::MoveTo(0, height + 1), Print("└"))?;
        for _ in 0..width {
            self.w.queue(Print("──"))?;
        }
        self.w.queue(Print("┘"))?;
        self.w.flush()?;
        Ok(())
    }

    fn draw_board(&mut self) -> Result<()> {
        for (idx, row) in self.sweeper.get_board().iter().enumerate() {
            // self.w.queue(cursor::MoveTo)
        }
        Ok(())
    }

    fn tear_down(&mut self) -> Result<()> {
        disable_raw_mode()?;
        queue!(self.w, LeaveAlternateScreen, cursor::Show)?;
        self.w.flush()?;
        Ok(())
    }
}
