# Spelling Bee Solver

Solves the [New York Times Spelling Bee](https://www.nytimes.com/puzzles/spelling-bee).

Their dictionary is not the same as the one included so you find missing words as well as extras.

## Usage

```
$ ./beesolver --help
Usage: beesolver <required_letter> <other_letters> [--dict <dict>] [--words-output <words-output>]

A Spelling Bee solver

Positional Arguments:
  required_letter   the letter required in all words
  other_letters     the 6 other allowed letters

Options:
  --dict            path to a custom dictionary file
  --words-output    when off, only the stats for the solution are output
                    (default on)
  --help            display usage information
```

## Requirements

* Rust 1.68 — though given how little language functionality is used it likely works on much earlier versions of Rust.

## Why

This was an excuse to play around with Rust. I come from a Java background and
[did this in Java first](https://github.com/kevinoliver/beesolver-java).
I'm fairly certain much of the code is not idiomatic Rust as I struggled through
a couple different parts of Rust.

## Development

Run all tests:
```
$ cargo test
```
