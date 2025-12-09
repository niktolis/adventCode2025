use std::env;

// Advent of Code 2025 Day 2 - URL for fetching puzzle input
const INPUT_URL: &str = "https://adventofcode.com/2025/day/2/input";

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Parse command-line argument to determine validation mode
    // Accepts "atleast", "at-least", or "at_least" for AtLeastDouble mode
    let mode = parse_mode(std::env::args().nth(1).as_deref());

    // Retrieve session cookie from environment variable for AOC authentication
    let session = env::var("AOC_SESSION")
        .map_err(|_| "AOC_SESSION environment variable is not set")?;
    
    // Fetch puzzle input from Advent of Code using authenticated session
    let body = ureq::get(INPUT_URL)
        .header("Cookie", &format!("session={session}"))
        .call()?
        .into_body()
        .read_to_string()?;

    // Process all lines and sum invalid IDs based on selected mode
    let sum = sum_of_invalid_ids(body.lines(), mode);

    println!("Sum of invalid IDs: {}", sum);

    Ok(())
}

/// Defines validation modes for detecting invalid ID patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InvalidMode {
    /// Invalid if the ID is exactly two halves repeated (e.g., 5555, 123123)
    ExactDouble,
    /// Invalid if the ID repeats a pattern 2+ times (e.g., 5555, 123123, 123123123)
    AtLeastDouble,
}

/// Parses command-line argument to determine validation mode.
/// 
/// Defaults to `ExactDouble` if no argument or unrecognized argument provided.
fn parse_mode(arg: Option<&str>) -> InvalidMode {
    match arg {
        Some("atleast") | Some("at-least") | Some("at_least") => InvalidMode::AtLeastDouble,
        _ => InvalidMode::ExactDouble,
    }
}

/// Represents an inclusive range of ID numbers to validate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    start: u64,
    end: u64,
}

/// Parses a single range from a string in the format "start-end".
/// 
/// Returns `None` if the format is invalid or numbers can't be parsed.
/// 
/// Example: "55-56" -> Some(Range { start: 55, end: 56 })
fn parse_range(part: &str) -> Option<Range> {
    let mut bounds = part.trim().splitn(2, '-');
    let start_str = bounds.next()?.trim();
    let end_str = bounds.next()?.trim();
    let start = start_str.parse().ok()?;
    let end = end_str.parse().ok()?;

    Some(Range { start, end })
}

/// Parses a comma-separated line into an iterator of ranges.
/// 
/// Skips empty parts and logs warnings for invalid range formats.
/// 
/// Example: "11-22, 95-115" yields Range{11,22} then Range{95,115}
fn ranges(line: &str) -> impl Iterator<Item = Range> + '_ {
    line.split(',').filter_map(|part| {
        let part = part.trim();
        if part.is_empty() {
            None
        } else {
            match parse_range(part) {
                Some(range) => Some(range),
                None => {
                    eprintln!("Warning: could not parse range: {part}");
                    None
                }
            }
        }
    })
}

/// Checks if a string is exactly two repeated halves.
/// 
/// Returns true only when the string length is even and the first half
/// equals the second half.
/// 
/// Examples:
/// - "5555" -> true ("55" + "55")
/// - "123123" -> true ("123" + "123")
/// - "123123123" -> false (3 repetitions, not exactly 2)
/// - "111" -> false (odd length)
fn is_repeating_pattern(s: &str) -> bool {
    // Only true when the string is exactly two repeated halves.
    if s.len() % 2 != 0 {
        return false;
    }

    let mid = s.len() / 2;
    &s[..mid] == &s[mid..]
}

/// Checks if a string contains a pattern repeated at least twice.
/// 
/// Tests all possible pattern sizes that could divide the string length evenly.
/// Returns true if any pattern of size 1 to len/2 repeats 2 or more times.
/// 
/// Examples:
/// - "5555" -> true (pattern "55" repeats 2 times, or "5" repeats 4 times)
/// - "123123" -> true (pattern "123" repeats 2 times)
/// - "123123123" -> true (pattern "123" repeats 3 times)
/// - "111" -> true (pattern "1" repeats 3 times)
/// - "1234" -> false (no repeating pattern)
fn is_repeating_at_least_twice(s: &str) -> bool {
    let len = s.len();
    
    // Try each possible pattern size from 1 to len/2
    for size in 1..=len / 2 {
        // Pattern size must evenly divide total length
        if len % size != 0 {
            continue;
        }
        
        let repeats = len / size;
        if repeats < 2 {
            continue;
        }
        
        // Check if all chunks match the first segment
        let segment = &s[..size];
        if s.as_bytes()
            .chunks(size)
            .all(|chunk| chunk == segment.as_bytes())
        {
            return true;
        }
    }
    false
}

/// Determines if a number is invalid based on the validation mode.
/// 
/// Converts the number to a string and checks for repeating patterns.
fn is_invalid(n: u64, mode: InvalidMode) -> bool {
    let s = n.to_string();
    match mode {
        InvalidMode::ExactDouble => is_repeating_pattern(&s),
        InvalidMode::AtLeastDouble => is_repeating_at_least_twice(&s),
    }
}

/// Sums all invalid numbers within an inclusive range.
/// 
/// Iterates through [start, end] and sums numbers that match the invalid pattern.
/// Returns 0 if start > end (with a warning).
fn sum_invalid_in_range(range: Range, mode: InvalidMode) -> u64 {
    if range.start > range.end {
        eprintln!("Warning: start greater than end in range: {:?}", range);
        return 0;
    }

    (range.start..=range.end)
        .filter(|&n| is_invalid(n, mode))
        .sum()
}

/// Calculates the total sum of invalid IDs across all ranges in all lines.
/// 
/// Each line may contain multiple comma-separated ranges. This function:
/// 1. Parses each line into ranges
/// 2. Sums invalid IDs within each range
/// 3. Accumulates the total using saturating addition to prevent overflow
/// 
/// # Arguments
/// * `lines` - Iterator of input lines, each containing comma-separated ranges
/// * `mode` - Validation mode (ExactDouble or AtLeastDouble)
fn sum_of_invalid_ids<'a, I>(lines: I, mode: InvalidMode) -> u64
where
    I: IntoIterator<Item = &'a str>,
{
    let mut sum: u64 = 0;
    for line in lines {
        for range in ranges(line) {
            sum = sum.saturating_add(sum_invalid_in_range(range, mode));
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test basic repeating digit pattern.
    /// Range 55-56 contains only 55 ("55" = "5" + "5"), which is invalid.
    #[test]
    fn repeating_digits_invalid() {
        let invalid_id_sum = sum_of_invalid_ids(["55-56"], InvalidMode::ExactDouble);
        assert_eq!(invalid_id_sum, 55);
    }

    /// Test repeating multi-digit chunk.
    /// 123123 = "123" + "123" (exact double), so it's invalid.
    #[test]
    fn repeating_chunk_invalid() {
        let invalid_id_sum = sum_of_invalid_ids(["123123-123123"], InvalidMode::ExactDouble);
        assert_eq!(invalid_id_sum, 123123);
    }

    /// Test that triple repetition is NOT invalid in ExactDouble mode.
    /// 123123123 has 3 repetitions of "123", not exactly 2, so it's valid.
    #[test]
    fn triple_repetition_is_valid() {
        let invalid_id_sum = sum_of_invalid_ids(["123123123-123123123"], InvalidMode::ExactDouble);
        assert_eq!(invalid_id_sum, 0);
    }

    /// Test odd-length repeating digit.
    /// "111" has odd length so can't be split into two equal halves - valid.
    #[test]
    fn odd_length_same_digit_is_valid() {
        let invalid_id_sum = sum_of_invalid_ids(["111-111"], InvalidMode::ExactDouble);
        assert_eq!(invalid_id_sum, 0);
    }

    /// Test multiple comma-separated ranges.
    /// Range 1-2 has no invalid IDs, range 55-56 has 55, total = 55.
    #[test]
    fn multiple_ranges_count_combines() {
        let invalid_id_sum = sum_of_invalid_ids(["1-2, 55-56"], InvalidMode::ExactDouble);
        assert_eq!(invalid_id_sum, 55);
    }
    
    /// Full test case for Part 1 with the example from Advent of Code.
    /// Tests multiple complex ranges in ExactDouble mode.
    #[test]
    fn aoc_test_part1() {
        let invalid_id_sum = sum_of_invalid_ids(["11-22,95-115,998-1012,1188511880-1188511890,
        222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,
        824824821-824824827,2121212118-2121212124"], InvalidMode::ExactDouble);
        assert_eq!(invalid_id_sum, 1227775554);
    }
    
    /// Full test case for Part 2 with the example from Advent of Code.
    /// Same ranges as Part 1 but using AtLeastDouble mode (2+ repetitions valid).
    #[test]
    fn aoc_test_part2() {
        let invalid_id_sum = sum_of_invalid_ids(["11-22,95-115,998-1012,1188511880-1188511890,
        222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,
        824824821-824824827,2121212118-2121212124"], InvalidMode::AtLeastDouble);
        assert_eq!(invalid_id_sum, 4174379265);
    }

    /// Verify that triple repetition IS invalid in AtLeastDouble mode.
    /// 123123123 has pattern "123" repeated 3 times (≥2), so it's invalid.
    #[test]
    fn triple_repetition_becomes_invalid_in_at_least_mode() {
        let invalid_id_sum = sum_of_invalid_ids(["123123123-123123123"], InvalidMode::AtLeastDouble);
        assert_eq!(invalid_id_sum, 123123123);
    }
}
