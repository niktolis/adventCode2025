# Advent of Code 2025 🎄

[![Rust](https://github.com/niktolis/adventCode2025/actions/workflows/rust.yml/badge.svg)](https://github.com/niktolis/adventCode2025/actions/workflows/rust.yml)

Solutions for [Advent of Code 2025](https://adventofcode.com/2025) written in Rust.

## Structure

Each day is implemented as a separate Rust project with its own README:

- [day1/](day1/) - Dial rotation simulator with zero-crossing detection
- [day2/](day2/) - Invalid ID pattern detector with multiple validation modes
- [day3/](day3/) - Maximum k-digit ordered number selector using greedy algorithm
- [day4/](day4/) - Grid-based roll removal with single/multi-pass neighbor detection
- [day5/](day5/) - Interval merging and membership testing with binary search
- [day6/](day6/) - Grid number processing with horizontal and vertical/block-based parsing

See each day's README for specific usage instructions and details.

## Prerequisites

- Rust (latest stable version)
- Advent of Code session cookie

## Setup

Set your AOC session cookie as an environment variable:

```bash
export AOC_SESSION="your-session-cookie-here"
```

To get your session cookie:

1. Log in to [adventofcode.com](https://adventofcode.com)
2. Open DevTools → Application → Cookies → <https://adventofcode.com>
3. Copy the value of the `session` cookie

## Quick Start

```bash
cd day1
cargo run
cargo test
```

## Running Tests

Run tests for all days:

```bash
for day in day{1..6}; do
  cd $day && cargo test && cd ..
done
```

## CI/CD

The project uses GitHub Actions to automatically:

- Cache dependencies for faster builds
- Build all days in parallel
- Run all tests
- Execute each day's main function with various modes

## License

This project is for educational purposes as part of Advent of Code 2025.
