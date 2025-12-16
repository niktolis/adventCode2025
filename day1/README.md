# Day 1: Dial Rotation Simulator

Simulates a circular dial (0-99) with zero-crossing detection.

## Problem Description

Starting at position 50 on a circular dial (0-99), process rotation instructions:
- Lines starting with 'R': rotate right
- Lines starting with 'L': rotate left
- Track how many times the dial crosses position 0

## Usage

```bash
cargo run
```

## Testing

```bash
cargo test
```

## Requirements

- Set `AOC_SESSION` environment variable with your Advent of Code session cookie
