# Day 4: Grid-Based Roll Removal

Removes rolls from a grid based on 8-directional neighbor detection with single or multi-pass modes.

## Problem Description

Processes a grid to remove rolls ('o') that have 4 or more neighboring rolls:

- **Single pass mode**: Removes all qualifying rolls simultaneously
- **Multi-pass mode**: Uses cascading BFS where removals can trigger subsequent removals

## Usage

**Single pass mode** (default):

```bash
cargo run
```

**Multi-pass mode**:

```bash
cargo run -- multi
```

## Testing

```bash
cargo test
```

Includes 14 comprehensive tests covering:

- Stable configurations
- Cascading removals
- Edge cases and boundary conditions
- Various grid patterns

## Requirements

- Set `AOC_SESSION` environment variable with your Advent of Code session cookie
