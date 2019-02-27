# goliard

![screenshot](./screenshot.gif)

## Installation

```
git clone https://github.com/billyrieger/smeagol
git clone https://github.com/billyrieger/goliard
cd goliard
cargo install --path . --force
```

## Quick start

```
curl http://www.conwaylife.com/patterns/sirrobin.rle | goliard -s
```

## Usage

`goliard --help`

```
goliard 0.1.0
Billy Rieger <wrieger@protonmail.com>
Command-line viewer for patterns in Conway's Game of Life 

USAGE:
    goliard [FLAGS] <FILE>

FLAGS:
    -s, --stdin      Reads a file from stdin
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <FILE>    The file to load. Not required if reading a file from stdin.
```

To see a list of available key commands, press `?`.
