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
- Bazelisk (recommended) or Bazel 6+
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

## Quick Start (Bazel)

```bash
# Build everything
bazelisk build //...

# Test everything
bazelisk test //...

# Run a specific day (example: day2 with "atleast" arg)
bazelisk run //day2:day2 -- atleast
```

## CI/CD

GitHub Actions runs `bazelisk build //...` and `bazelisk test //...`; the tests include smoke runs for each day/mode (default plus `day2 -- atleast`, `day4 -- multi`, `day6 -- part2`). See `.github/workflows/rust.yml`.

## License

This project is for educational purposes as part of Advent of Code 2025.
