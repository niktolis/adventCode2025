use std::env;

// Advent of Code 2025 Day 1 - URL for fetching puzzle input
const INPUT_URL: &str = "https://adventofcode.com/2025/day/1/input";

/// Classifies the starting character of an instruction line.
/// Used to determine whether the dial rotates right (R) or left (L).
enum LineStart {
    Right,
    Left,
    Other,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Retrieve session cookie from environment variable for AOC authentication
    let session = env::var("AOC_SESSION")
        .map_err(|_| "AOC_SESSION environment variable is not set")?;
    
    // Fetch puzzle input from Advent of Code using authenticated session
    let body = ureq::get(INPUT_URL)
        .header("Cookie", &format!("session={session}"))
        .call()?
        .into_body()
        .read_to_string()?;

    // Process all instruction lines starting from dial position 50
    let stats = process_lines(50, body.lines());

    println!("Times dial pointed at 0: {}", stats.zero_hits);
    println!("Final value: {}", stats.value);

    Ok(())
}

/// Classifies a line based on its first character.
/// 
/// Returns:
/// - `LineStart::Right` if the line starts with 'R' (rotate right)
/// - `LineStart::Left` if the line starts with 'L' (rotate left)
/// - `LineStart::Other` for any other character or empty lines
fn classify_line(line: &str) -> LineStart {
    match line.as_bytes().first().copied() {
        Some(b'R') => LineStart::Right,
        Some(b'L') => LineStart::Left,
        _ => LineStart::Other,
    }
}

/// Results from processing a sequence of dial rotation instructions.
#[derive(Debug, PartialEq, Eq)]
struct Stats {
    /// Final position of the dial (0-99)
    value: u32,
    /// Total number of times the dial crossed or landed on position 0
    zero_hits: u32,
}

/// Processes a sequence of dial rotation instructions and tracks statistics.
/// 
/// The dial is modeled as a circular 0-99 range:
/// - 'R' commands rotate clockwise (increment)
/// - 'L' commands rotate counter-clockwise (decrement)
/// - Tracks how many times the dial crosses or lands on position 0
/// 
/// # Arguments
/// * `start` - Initial dial position (will be normalized to 0-99)
/// * `lines` - Iterator of instruction lines (format: "R<number>" or "L<number>")
/// 
/// # Returns
/// `Stats` containing the final dial position and total zero crossings
fn process_lines<'a, I>(start: u32, lines: I) -> Stats
where
    I: IntoIterator<Item = &'a str>,
{
    let mut value = start % 100;  // Normalize starting position to 0-99
    let mut zero_hits = 0;

    for line in lines {
        match classify_line(line) {
            LineStart::Right => {
                if let Some(rest) = line.strip_prefix('R') {
                    if let Ok(delta) = rest.trim().parse::<u32>() {
                        // Count how many times we cross 0 when rotating right
                        zero_hits += zero_hits_right(value, delta);
                        // Update position (use u64 to prevent overflow before modulo)
                        value = ((value as u64 + delta as u64) % 100) as u32;
                    } else {
                        eprintln!("Warning: invalid number after R in line: {line}");
                    }
                }
            }
            LineStart::Left => {
                if let Some(rest) = line.strip_prefix('L') {
                    if let Ok(delta) = rest.trim().parse::<u32>() {
                        // Count how many times we cross 0 when rotating left
                        zero_hits += zero_hits_left(value, delta);
                        // Update position (add 100 before subtracting to avoid underflow)
                        value = (value + 100 - (delta % 100)) % 100;
                    } else {
                        eprintln!("Warning: invalid number after L in line: {line}");
                    }
                }
            }
            LineStart::Other => {
                eprintln!("Warning: unrecognized line start: {line}");
            }
        }
    }

    Stats {
        value,
        zero_hits,
    }
}

/// Calculates how many times the dial crosses 0 when rotating right (clockwise).
/// 
/// When rotating right from position `start` by `delta` steps, we cross 0 each time
/// we complete a full 100-position cycle. This is computed by integer division.
/// 
/// Example: Starting at 50, rotating right by 250 crosses 0 twice (at 100 and 200).
fn zero_hits_right(start: u32, delta: u32) -> u32 {
    ((start as u64 + delta as u64) / 100) as u32
}

/// Calculates how many times the dial crosses 0 when rotating left (counter-clockwise).
/// 
/// When rotating left from position `start` by `delta` steps:
/// - If already at 0: count full cycles (delta / 100)
/// - If delta < start: no zero crossing
/// - Otherwise: cross 0 once immediately, then count additional full cycles
/// 
/// Example: Starting at 5, rotating left by 7 crosses 0 once (goes 5→4→3→2→1→0→99→98).
fn zero_hits_left(start: u32, delta: u32) -> u32 {
    if start == 0 {
        delta / 100
    } else if delta < start {
        0
    } else {
        1 + (delta - start) / 100
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test right rotation with multiple full cycles.
    /// Starting at 50, rotating right 1000 steps = 10 full cycles.
    /// Final position: (50 + 1000) % 100 = 50
    #[test]
    fn right_wraps_correctly() {
        let stats = process_lines(50, ["R1000"]);
        assert_eq!(
            stats,
            Stats {
                value: 50,
                zero_hits: 10,
            }
        );
    }

    /// Test right rotation landing exactly on 0.
    /// Starting at 50, rotating right 950 steps lands on 0.
    /// Crosses 0 at steps: 50, 150, 250, ..., 950 (10 times total)
    #[test]
    fn right_wraps_corner_case() {
        let stats = process_lines(50, ["R950"]);
        assert_eq!(
            stats,
            Stats {
                value: 0,
                zero_hits: 10,
            }
        );
    }

    /// Test left rotation wrapping around 0.
    /// Starting at 5, rotating left 7 steps: 5→4→3→2→1→0→99→98
    /// Crosses 0 once at step 6.
    #[test]
    fn left_wraps_correctly() {
        let stats = process_lines(5, ["L7"]);
        assert_eq!(
            stats,
            Stats {
                value: 98,
                zero_hits: 1,
            }
        );
    }

    /// Test left rotation landing exactly on 0.
    /// Starting at 10, rotating left 10 steps lands precisely on 0.
    /// Should count as 1 zero hit.
    #[test]
    fn zero_without_wrap_counts() {
        let stats = process_lines(10, ["L10"]);
        assert_eq!(
            stats,
            Stats {
                value: 0,
                zero_hits: 1,
            }
        );
    }

    /// Test a sequence of mixed right and left rotations.
    /// 90 → R20 → 10 → L5 → 5 → R15 → 20
    /// Only the first right rotation (90→10) crosses 0 once at position 0.
    #[test]
    fn mixed_sequence_combines_counts() {
        let stats = process_lines(90, ["R20", "L5", "R15"]);
        assert_eq!(
            stats,
            Stats {
                value: 20,
                zero_hits: 1,
            }
        );
    }

    /// Full test case with the example from Advent of Code.
    /// Tests a complex sequence of 10 instructions to verify correct
    /// tracking of both final position and zero crossings.
    #[test]
    fn aoc_test() {
        let stats = process_lines(50, ["L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99",
        "R14", "L82"]);
        assert_eq!(
            stats,
            Stats{
                value: 32,
                zero_hits: 6,
            }
        );
    }
}
