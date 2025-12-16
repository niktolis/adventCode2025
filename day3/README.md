# Day 3: Maximum K-Digit Ordered Number Selector

Finds the maximum k-digit number from strings where selected digits must maintain their original order.

## Problem Description

Uses a greedy algorithm with a monotonic stack to select k digits from a string that form the largest possible number while preserving their relative order.

Default configuration: k=12 digits

## Usage

```bash
cargo run
```

## Testing

```bash
cargo test
```

## Algorithm

Employs a greedy approach:
1. Maintain a monotonic decreasing stack
2. For each digit, pop smaller digits if there are enough remaining digits
3. Keep exactly k digits that form the maximum value

## Requirements

- Set `AOC_SESSION` environment variable with your Advent of Code session cookie
