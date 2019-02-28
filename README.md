# goliard

[![Build status](https://img.shields.io/travis/com/billyrieger/goliard.svg)](https://travis-ci.com/billyrieger/smeagol)
[![Lines of code](https://tokei.rs/b1/github/billyrieger/goliard)](https://github.com/Aaronepower/tokei)
[![Version](https://img.shields.io/crates/v/goliard.svg)](https://crates.io/crates/goliard)
[![License](https://img.shields.io/crates/l/goliard.svg)](https://github.com/billyrieger/goliard/blob/master/LICENSE)
[![Dependency status](https://deps.rs/repo/github/billyrieger/goliard/status.svg)](https://deps.rs/repo/github/billyrieger/goliard)
[![Rust version](https://img.shields.io/badge/rust-nightly-lightgrey.svg)](https://www.rust-lang.org/)

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
