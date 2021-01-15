use clap::{App, Arg, ArgGroup};
use sweeper::{BoardConfig, Sweeper};

mod sweeper;

fn main() {
    let matches = App::new("sweeprs")
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
        sweeper::MED_CONFIG
    } else if matches.is_present("hard") {
        sweeper::HARD_CONFIG
    } else if matches.is_present("custom") {
        let args: Vec<usize> = matches
            .values_of("custom")
            .unwrap()
            .map(|x| x.to_string().parse::<usize>().unwrap())
            .collect();
        BoardConfig {
            width: args[0],
            height: args[1],
            mine_count: args[2],
        }
    } else {
        sweeper::EASY_CONFIG
    };
    match Sweeper::new(config) {
        Ok(mut board) => {
            board.open(4, 4);
            println!("{:?}{:?}", board, board.game_state());
            board.flag(0, 0);
            println!("{:?}{:?}", board, board.game_state());
        }
        _ => println!("error: invalid board configuration"),
    }
}
