# sweeprs

[![Docker Pulls](https://img.shields.io/docker/pulls/ilmannafian/sweeprs?label=Docker%20Pulls)](https://hub.docker.com/repository/docker/ilmannafian/sweeprs)
[![Release](https://github.com/ilmannafian04/sweeprs/actions/workflows/release.yml/badge.svg)](https://github.com/ilmannafian04/sweeprs/actions/workflows/release.yml)

[![asciicast](https://asciinema.org/a/388134.svg)](https://asciinema.org/a/388134)

A terminal based minesweeper built in rust.

## Usage

- `a` to open a cell
- `s` to flag a cell
- `q` to quit
- Arrow keys to move around

## Option

```
> sweeprs --help
sweeprs 1.0.0
M. Ilman Nafian <milmannafian04@gmail.com>
A terminal based minesweeper. Use arrow keys to move arround, `a` to open and `s` to flag a cell, `q` to quit the game.

USAGE:
    sweeprs [FLAGS] [OPTIONS]

FLAGS:
    -e, --easy       Easy difficulty with 9x9 board and 10 mines.
    -h, --hard       Hard difficulty with 24x24 board and 99 mines.
    -m, --medium     Medium difficulty with 16x16 board and 40 mines.
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --custom <WIDTH> <HEIGHT> <MINE>    Custom board configuration

```

## Docker

```shell
docker pull ilmannafian/sweeprs
docker run -it --rm ilmannafian/sweeprs
```

