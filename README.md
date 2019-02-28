# goliard

`goliard` is a command-line viewer for patterns in Conway's Game of Life. It's powered by
[`smeagol`](https://github.com/billyrieger/smeagol), which uses HashLife and SIMD to efficiently
evolve repetitive patterns.

![screenshot](./screenshot.gif)

## Installation

```
cargo install goliard --force
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

## License

`goliard` is licensed under the Mozilla Public License version 2.0. See the [license
file](https://github.com/billyrieger/goliard/blob/master/LICENSE) and the [MPL 2.0
FAQ](https://www.mozilla.org/en-US/MPL/2.0/FAQ/) for more details.
