# Day 5: Interval Merging and Membership Testing

Merges overlapping intervals and performs efficient membership testing using binary search.

## Problem Description

Given a list of intervals and test numbers:

1. Merges overlapping or adjacent intervals
2. Tests which numbers fall within the merged intervals
3. Counts total members across all intervals

## Usage

```bash
cargo run
```

## Testing

```bash
cargo test
```

## Algorithm

- **Merge**: Sort intervals by start position, then merge overlapping ranges
- **Search**: Binary search to locate relevant interval
- **Count**: Sum interval lengths accounting for inclusiveness

## Requirements

- Set `AOC_SESSION` environment variable with your Advent of Code session cookie
