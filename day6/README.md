# Day 6: Grid Number Processing

Processes grid-based numeric data with two different parsing strategies.

## Problem Description

Implements two parsing modes:

- **Part 1**: Horizontal whitespace-separated column parsing
- **Part 2**: Vertical block-based parsing with all-space column separators

Each mode:

1. Separates numbers into columns/blocks
2. Applies operators (`+` or `*`) to operands
3. Sums results across all operations

## Usage

**Part 1** (horizontal parsing):

```bash
 # Bazel
 bazelisk run //day6:day6

 # Cargo (optional)
 cargo run
```

**Part 2** (vertical/block-based parsing):

```bash
 # Bazel
 bazelisk run //day6:day6 -- part2

 # Cargo (optional)
 cargo run -- part2
```

## Testing

```bash
 # Bazel
 bazelisk test //day6:day6_test

 # Cargo (optional)
 cargo test
```

Tests validate both parsing modes with proper input formatting.

## Input Format

- **Part 1**: Numbers are whitespace-separated in columns
- **Part 2**: Numbers form blocks separated by all-space columns; each column within a block represents one operand read vertically (top to bottom)

## Requirements

- Set `AOC_SESSION` environment variable with your Advent of Code session cookie
