# Advent of Code 2025 🎄

[![Rust](https://github.com/niktolis/adventCode2025/actions/workflows/rust.yml/badge.svg)](https://github.com/niktolis/adventCode2025/actions/workflows/rust.yml)

Solutions for [Advent of Code 2025](https://adventofcode.com/2025) written in Rust.

## Structure

Each day is implemented as a separate Rust project:

- `day1/` - Day 1: Dial rotation simulator with zero-crossing detection
- `day2/` - Day 2: Invalid ID pattern detector with multiple validation modes
- `day3/` - Day 3: Maximum k-digit ordered number selector using greedy algorithm

## Prerequisites

- Rust (latest stable version)
- Advent of Code session cookie

## Setup

1. Set your AOC session cookie as an environment variable:

```bash
export AOC_SESSION="your-session-cookie-here"
```

To get your session cookie:

- Log in to [adventofcode.com](https://adventofcode.com)
- Open DevTools → Application → Cookies → <https://adventofcode.com>
- Copy the value of the `session` cookie

1. Build and run a specific day:

```bash
cd day1
cargo build
cargo run
cargo test
```

## Running Individual Days

### Day 1

```bash
cd day1
cargo run
```

### Day 2

Day 2 supports two modes:

**Exact Double mode** (default):

```bash
cd day2
cargo run
```

**At Least Double mode**:

```bash
cd day2
cargo run -- atleast
```

### Day 3

```bash
cd day3
cargo run
```

## Running Tests

Run tests for all days:

```bash
for day in day1 day2 day3; do
  cd $day
  cargo test
  cd ..
done
```

Or for a specific day:

```bash
cd day1
cargo test
```

## License

This project is for educational purposes as part of Advent of Code 2025.
