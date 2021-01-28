use std::{
    io::{stdout, BufWriter, Stdout, Write},
    ops::{AddAssign, SubAssign},
};

use clap::{Arg, ArgGroup};
use crossterm::{
    cursor,
    event::{
        read,
        Event::Key,
        KeyCode::{Char, Down, Left, Right, Up},
    },
    queue,
    terminal::{disable_raw_mode, enable_raw_mode},
};

use sweeprs::{Sweeper, SweeperConfig, EASY_CONFIG, HARD_CONFIG, MED_CONFIG};

fn main() {
    let matches = clap::App::new("sweeprs")
        .version("0.1.0")
        .about("lol")
        .author("M. Ilman Nafian <milmannafian04@gmail.com>")
        .arg(
            Arg::with_name("easy")
                .short("e")
                .long("easy")
                .display_order(1)
                .help("Easy difficulty with 9x9 board and 10 mines."),
        )
        .arg(
            Arg::with_name("medium")
                .short("m")
                .long("medium")
                .display_order(1)
                .help("Medium difficulty with 16x16 board and 40 mines."),
        )
        .arg(
            Arg::with_name("hard")
                .short("h")
                .long("hard")
                .display_order(1)
                .help("Hard difficulty with 24x24 board and 99 mines."),
        )
        .arg(
            Arg::with_name("custom")
                .short("c")
                .long("custom")
                .takes_value(true)
                .number_of_values(3)
                .value_names(&["WIDTH", "HEIGHT", "MINE"])
                .validator(|arg| match arg.parse::<usize>() {
                    Ok(_) => Ok(()),
                    Err(_) => Err("only accept positive integer".to_string()),
                })
                .display_order(1)
                .help("Custom board configuration"),
        )
        .group(ArgGroup::with_name("difficulty").args(&["easy", "medium", "hard", "custom"]))
        .get_matches();
    let config = if matches.is_present("medium") {
        MED_CONFIG
    } else if matches.is_present("hard") {
        HARD_CONFIG
    } else if matches.is_present("custom") {
        let args: Vec<usize> = matches
            .values_of("custom")
            .unwrap()
            .map(|x| x.to_string().parse::<usize>().unwrap())
            .collect();
        SweeperConfig {
            width: args[0],
            height: args[1],
            mine_count: args[2],
        }
    } else {
        EASY_CONFIG
    };
    match Sweeper::new(config) {
        Ok(board) => {
            let mut stdout = BufWriter::new(stdout());
            Game::new(board, &mut stdout).run().ok();
        }
        Err(e) => println!("error: {}", e),
    }
}

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

pub struct Game<'a> {
    i: BoundedIndex,
    j: BoundedIndex,
    sweeper: Sweeper,
    w: &'a mut W,
}

impl<'a> Game<'a> {
    fn new(sweeper: Sweeper, w: &'a mut W) -> Self {
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

    fn run(&mut self) -> crossterm::Result<()> {
        queue!(self.w, cursor::Hide,)?;
        enable_raw_mode()?;
        self.w.flush()?;
        self.draw()?;

        loop {
            print!("{:?}\n\r", cursor::position());
            match read() {
                Ok(event) => {
                    if let Key(ke) = event {
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

    fn draw(&mut self) -> crossterm::Result<()> {
        print!("\n\r");
        Ok(())
    }

    fn tear_down(&mut self) -> crossterm::Result<()> {
        disable_raw_mode()?;
        queue!(self.w, cursor::Show)?;
        self.w.flush()?;
        Ok(())
    }
}
