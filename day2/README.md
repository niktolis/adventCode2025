# Day 2: Invalid ID Pattern Detector

Detects invalid IDs based on repeating digit patterns with two validation modes.

## Problem Description

Identifies invalid IDs by checking for repeating patterns in digit sequences:

- **Exact Double mode**: Pattern appears exactly twice consecutively
- **At Least Double mode**: Pattern appears two or more times consecutively

## Usage

**Exact Double mode** (default):

```bash
cargo run
```

**At Least Double mode**:

```bash
cargo run -- atleast
```

Alternative mode syntax: `at-least` or `at_least`

## Testing

```bash
cargo test
```

## Requirements

- Set `AOC_SESSION` environment variable with your Advent of Code session cookie
