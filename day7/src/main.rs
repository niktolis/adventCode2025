use anyhow::{bail, Context, Result};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::env;

const INPUT_URL: &str = "https://adventofcode.com/2025/day/7/input";

/// Parsed grid representation.
/// 
/// rows: Vec<Vecv<u8>> where each row is a byte slice of '.' '^' 'S'
/// width: fixed width, all rows are padded/validated to this width
struct Grid {
    rows: Vec<Vec<u8>>,
    width: usize,
}

/// Parse input text into a rectangular grid.
/// 
/// Steps:
/// 1) Keep non-empty lines.
/// 2) Validate all lines have the same width (AoC grids are rectangular).
/// 3) Store each line as bytes for fast indexing (no UTF-8 surprises).
/// 
fn parse_grid(input: &str) -> Result<Grid> {
    let lines: Vec<&str> = input.lines().filter(|l| !l.trim().is_empty()).collect();
    if lines.is_empty() {
        bail!("Empty input");
    }

    let width = lines[0].len();
    if width == 0 {
        bail!("First line is empty");
    }

    let mut rows = Vec::with_capacity(lines.len());
    for (i, &line) in lines.iter().enumerate() {
        if line.len() != width {
            bail!(
                "Ragged grid: line {i} has length {}, expected {width}",
                line.len()
            );
        }
        rows.push(line.as_bytes().to_vec());
    }

    Ok(Grid {rows, width })
}

/// Find the column of 'S' in the top row.
/// 
/// Steps:
/// 1) Scan the row for byte 'S'.
/// 2) REturn its index, or error if missing.
fn find_start_column(top_row: &[u8]) -> Result<usize> {
    top_row
        .iter()
        .position(|&c| c == b'S')
        .with_context(|| "No 'S' found in top row")
}

/// Build splitter masks for all rows.
/// 
/// Each row becomes a bitset (Vec<u64>) where:
/// - bit c = 1 if grid[row][c] == '^'
/// 
/// Steps per row:
/// 1) Create zeroed u64 chunks.
/// 2) For each column with '^', set the corresponding bit.
/// 3) Mask last chunk to clear unused bits.
fn build_split_masks(rows: &[Vec<u8>], width: usize, chunks: usize, last_mask: u64) -> Vec<Vec<u64>> {
    let mut out = Vec::with_capacity(rows.len());

    for row in rows {
        let mut mask_row = vec![0u64; chunks];

        for c in 0..width {
            if row[c] == b'^' {
                mask_row[c / 64] |= 1u64 << (c % 64);
            }
        }

        // Ensure unused bits are always 0 (important after shifts/or).
        if let Some(last) = mask_row.last_mut() {
            *last &= last_mask;
        }

        out.push(mask_row);
    }

    out
}

/// Perform one DP step: propagate beams from `cur` into `next` for a specific row,
/// and count how many splitters are hit.
/// 
/// Inputs:
/// - cur: current beam bitset (row r-1)
/// - split: bitset of '^' positions for row r
/// - next output bitset for row r (overwritten)
/// - last_mask: masks unused tail bits (width not multiple of 64)
/// 
/// Output:
/// - number of split events on this row (popcount of hit splitters)
/// 
/// Algorithm:
/// 1) hit = cur & split
/// 2) straight = cur & !split
/// 3) next = straight
/// 4) next |= (hit << 1)   // split right
/// 5) next |= (hit >> 1)   // split left
/// 6) next[last] &= last_mask
/// 7) return popcount(hit)
fn step_row_part1(cur: &[u64], split: &[u64], next: &mut [u64], last_mask: u64) -> u64 {
    debug_assert_eq!(cur.len(), split.len());
    debug_assert_eq!(cur.len(), next.len());

    let chunks = cur.len();
    next.fill(0);

    // Pass 1: compute hit + straight, write straight into next, count splits.
    let mut splits_on_row: u64 = 0;
    for k in 0..chunks {
        let hit = cur[k] & split[k];
        let straight = cur[k] & !split[k];
        next[k] = straight; 
        splits_on_row += hit.count_ones() as u64
    }

    // Pass 2: OR in right-shifted split beams: (hit << 1)
    //
    // We recompute `hit` (cur & split) to avoid allocating a temporary `hit` vector.
    // This is still 0(chunks) and typically faster than heap traffic.
    let mut carry: u64 = 0;
    for k in 0..chunks {
        let hit = cur[k] & split[k];
        let new_carry = hit >> 63;      // MSB spills into next chunk as LSB
        let shifted = (hit << 1) | carry;   // carry comes from previous chunk
        next[k] |= shifted;
        carry = new_carry
    }

    // Pass 3: OR in left-shifted split beams: (hit >> 1)
    let mut carry: u64 = 0;
    for k in (0..chunks).rev() {
        let hit = cur[k] & split[k];
        let new_carry = hit & 1;        // LSB spills into previous chunk as MSB
        let shifted = (hit >> 1) | (carry << 63);
        next[k] |= shifted;
        carry = new_carry;
    }

    // Clear unused tail bits (so they never leak and cause false hits).
    if let Some(last) = next.last_mut() {
        *last &= last_mask;
    }

    splits_on_row
}

/// Set a single beam bit in a bitset at column `col`.
#[inline]
fn set_bit(bits: &mut [u64], col: usize) {
    bits[col / 64] |= 1u64 << (col % 64);
}

/// Process part1 input
/// 
/// High level abstract steps:
/// 1) Parse the grid into rows of bytes.
/// 2) Find the start column 'S' in the top row.
/// 3) Precompute splitter masks: for each row, a bitset with 1s where '^' exists.
/// 4) Run a row-by-row bitset DP that updates beam positions and counts splitter hits.

fn process_part1_int(grid: &Grid, s_col: usize) -> u64 {

    let (h, w) = (grid.rows.len(), grid.width);

    if h <= 1 {
        return 0;
    }

    // Bitset layout:
    // - one u64 = 64 columns
    // - chunks = ceil(w / 64)
    let chunks = (w + 63) / 64;

    // Last chunk may have unused bits if w is not multiple of 64.
    // last_mask keeps only valid column bits (lower bits).
    let last_mask: u64 = if w % 64 == 0 {
        !0u64
    } else {
        (1u64 << (w % 64)) - 1
    };

    // Precompute: split_masks[r][k] has bit=1 if grid[r][col] == '^'.
    let split_masks = build_split_masks(&grid.rows, w, chunks, last_mask);

    // Beam state:
    // cur: bitset for current row
    // next: bitset for next row
    let mut cur = vec![0u64; chunks];
    let mut next = vec![0u64; chunks];

    // Initialize beam "presence" at row 0, column S.
    set_bit(&mut cur, s_col);

    let mut splits_total: u64 = 0;

    // We start from row 1 because row 0 is the header with 'S'.
    // The beam enters row 1 from row 0.
    for r in 1..h {
        // Compute next row's beam bitset and number of splits on this row.
        let  splits_on_row = step_row_part1(&cur, &split_masks[r], &mut next, last_mask);

        splits_total += splits_on_row;
        std::mem::swap(&mut cur, &mut next);
    }

    splits_total
}

fn process_part1(input: &str) -> Result<u64> {
    let grid = parse_grid(input)?;
    let s_col = find_start_column(&grid.rows[0])?;
    Ok(process_part1_int(&grid, s_col))
}

/// One DP step for Part2
///
/// Inputs:
/// - row: current grid row bytes
/// - cur: current timelines per column (active in [l..r])
/// - next: output timelines per column (will be cleared/filled only in needed range)
/// -l, r: active window in cur
/// 
/// Returns:
/// - (new_l, new_r): active window in `next` after propagation
/// 
/// Part2 counts distinct timelines (paths).
/// Timelines do NOT merge, even if they end at the same cell.
/// DP state cur[c] = number of timelines arriving at column c for the current row.
/// On '.' : next[c]  += cur[c]
/// On '^' : next[c-1] += cur[c] (if in bounds)
///          next[c+1] += cur[c] (if in bounds)
/// Answer: sum(cur) at the bottom row.
/// 
/// Using BigUint because values can be huge.
/// 
/// Optimization: track active window [l..r] where cur[c] != 0 so we avoid full width
fn step_row_part2(row: &[u8], cur: &[BigUint], next: &mut [BigUint], l: usize, r: usize) -> (usize, usize) {
    let w = cur.len();
    debug_assert_eq!(row.len(), w);
    debug_assert_eq!(next.len(), w);
    debug_assert!(l <= r && r < w);

    // Next activity can expand by at most 1 to each side
    let nl = l.saturating_sub(1);
    let nr = (r + 1).min(w - 1);

    // Clear only the region that might be written.
    for c in nl..=nr {
        next[c].set_zero();
    }

    // Propagate counts.
    for c in l..=r {
        if cur[c].is_zero() {
            continue;
        }

        if row[c] == b'^' {
            if c > 0 {
                next[c - 1] += &cur[c];
            }
            if c + 1 < w {
                next[c + 1] += &cur[c];
            }
        } else {
            next[c] += &cur[c];
        }
    }

    //Compute new active window in [nl..nr]
    let mut new_l = nl;
    while new_l <= nr && next[new_l].is_zero() {
        new_l += 1;
    }
    if new_l > nr {
        // No timelines survived (everything fell off the edges).
        return (0, 0);
    }

    let mut new_r = nr;
    while next[new_r].is_zero() {
        new_r -= 1;
    }

    (new_l, new_r)
}

/// Internal Part2. Returns total number of timlines as BigUint
fn process_part2_int(grid: &Grid, s_col: usize) -> BigUint {
    let (h, w) = (grid.rows.len(), grid.width);

    if h <= 1 {
        return BigUint::one(); // timeline is already "done" on the start
    }

    let mut cur = vec![BigUint::zero(); w];
    let mut next = vec![BigUint::zero(); w];

    cur[s_col] = BigUint::one();
    let mut l = s_col;
    let mut r = s_col;

    for row_idx in 1..h {
        let row = &grid.rows[row_idx];

        let (new_l, new_r) = step_row_part2(row, &cur, &mut next, l, r);

        if new_l == 0 && new_r == 0 && next[0].is_zero() {
            return BigUint::zero();
        }

        std::mem::swap(&mut cur, &mut next);
        l = new_l;
        r = new_r;
    }

    // Total timelines is the sum at the final row.
    let mut total = BigUint::zero();
    for c in l..=r {
        total += &cur[c];
    }

    total

}

fn process_part2(input: &str) -> Result<BigUint> {
    let grid = parse_grid(input)?;
    let s_col = find_start_column(&grid.rows[0])?;
    Ok(process_part2_int(&grid, s_col))
}

fn main() -> Result<()> {

    let mut args = std::env::args().skip(1);
    let mode = args.next().unwrap_or_else(|| "part1".to_string());

    let session = env::var("AOC_SESSION")
        .context("AOC_SESSION environment variable is not set")?;

    let body = ureq::get(INPUT_URL)
        .header("Cookie", &format!("session={session}"))
        .call()?
        .into_body()
        .read_to_string()?;

match mode.as_str() {
    "part1" | "1" => {
        let total = process_part1(&body)?;
        println!("{total}");
    }
    "part2" | "2" => {
        let total = process_part2(&body)?;
        println!("{total}");
    }
    _ => bail!("Unknown mode '{mode}'. Use part1/1 or part2/2."),
}

    Ok(())
} 

#[cfg(test)]
mod tests {
    use super::*;

     /// Helper: parse + start for tests
    fn grid_and_start(input: &str) -> Result<(Grid, usize)> {
        let g = parse_grid(input)?;
        let s = find_start_column(&g.rows[0])?;
        Ok((g, s))
    }

     // -------------------------
    // Part 1: unit + regression
    // -------------------------

    #[test]
    fn parse_rejects_empty() -> Result<()> {
        let err = parse_grid("").err().context("expected error")?;
        let _ = err; // just to silence unused warning in case you expand
        Ok(())
    }

    #[test]
    fn parse_rejects_ragged() -> Result<()> {
        let input = "S..\n....\n";
        if parse_grid(input).is_ok() {
            bail!("expected ragged grid to fail");
        }
        Ok(())
    }

    #[test]
    fn start_must_exist() -> Result<()> {
        let input = "....\n.^..\n";
        let g = parse_grid(input)?;
        if find_start_column(&g.rows[0]).is_ok() {
            bail!("expected missing S to fail");
        }
        Ok(())
    }

    #[test]
    fn part1_tiny_single_split() -> Result<()> {
        // r0: S at col 2
        // r1: ^ at col 2 => hit 1
        // r2: . => no more hits
        let input = "\
..S..
..^..
.....
";
        let (g, s) = grid_and_start(input)?;
        let ans = process_part1_int(&g, s);
        assert_eq!(ans, 1);
        Ok(())
    }

    #[test]
    fn part1_two_splits_in_one_row() -> Result<()> {
        // r0: S at col 2
        // r1: ^ at col 2 => split -> beams at 1 and 3
        // r2: ^ at col 1 and 3 => hit 2 => total 3
        let input = "\
..S..
..^..
.^.^.
.....
";
        let (g, s) = grid_and_start(input)?;
        let ans = process_part1_int(&g, s);
        assert_eq!(ans, 3);
        Ok(())
    }

    #[test]
    fn part1_example_from_prompt() -> Result<()> {
        // IMPORTANT: no indentation in the literal.
        let input = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";
        let ans = process_part1(input)?;
        assert_eq!(ans, 21);
        Ok(())
    }
}
