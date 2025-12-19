# Day 7: Timeline Grid Traversal

Processes grid-based timeline data using bitset operations and BigInt arithmetic to compute total possible states.

## Problem Description

Analyzes grid traversal patterns to determine the total number of possible timelines based on column operations starting from position 'S'.

Supports two modes:

- **Part 1**: Direct column-wise processing
- **Part 2**: Advanced bitset-based computation with BigInt support

## Usage

**Part 1** (default):

```bash
# Bazel
bazelisk run //day7:day7

# Cargo (optional)
cargo run
```

**Part 1** (explicit):

```bash
# Bazel
bazelisk run //day7:day7 -- part1

# Cargo (optional)
cargo run -- part1
```

**Part 2**:

```bash
# Bazel
bazelisk run //day7:day7 -- part2

# Cargo (optional)
cargo run -- part2
```

## Testing

```bash
# Bazel unit tests (9 tests: 6 part1, 3 part2)
bazelisk test //day7:day7_test

# Bazel smoke tests
bazelisk test //day7:day7_run_default   # part1 smoke test
bazelisk test //day7:day7_run_part1     # part1 explicit
bazelisk test //day7:day7_run_part2     # part2 smoke test

# Cargo (optional)
cargo test
```

## Algorithm

Uses bitset masking for efficient grid processing:

1. **Grid parsing**: Validate rectangular input and store as bytes
2. **Bitset computation**: Split masks represent '^' positions using u64 chunks
3. **Column tracking**: Range queries over valid column ranges
4. **BigInt arithmetic**: Compute timeline totals using arbitrary precision integers

## Dependencies

- `ureq` 3.1.4 – HTTP requests for puzzle input
- `httparse` 1.10.1 – HTTP parsing with std feature
- `anyhow` 1 – Error handling
- `num-bigint` 0.4.6 – Arbitrary precision integers
- `num-traits` 0.2.19 – Numeric traits

## Requirements

- Set `AOC_SESSION` environment variable with your Advent of Code session cookie
