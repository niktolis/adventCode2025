const INPUT_URL: &str = "https://adventofcode.com/2025/day/2/input";
const SESSION: &str = "53616c7465645f5fb314d25ef781c6cdf54d85608b12da3d864aa197b303ad4582aeac4a20fb1eacc7e68e5ad1898334efc4fbcfa7316fb6d26ac2bb57f250c9";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mode = parse_mode(std::env::args().nth(1).as_deref());

    let body = ureq::get(INPUT_URL)
        .set("Cookie", &format!("session={SESSION}"))
        .call()?
        .into_string()?;

    let sum = sum_of_invalid_ids(body.lines(), mode);

    println!("Sum of invalid IDs: {}", sum);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InvalidMode {
    ExactDouble,
    AtLeastDouble,
}

fn parse_mode(arg: Option<&str>) -> InvalidMode {
    match arg {
        Some("atleast") | Some("at-least") | Some("at_least") => InvalidMode::AtLeastDouble,
        _ => InvalidMode::ExactDouble,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    start: u64,
    end: u64,
}

fn parse_range(part: &str) -> Option<Range> {
    let mut bounds = part.trim().splitn(2, '-');
    let start_str = bounds.next()?.trim();
    let end_str = bounds.next()?.trim();
    let start = start_str.parse().ok()?;
    let end = end_str.parse().ok()?;

    Some(Range { start, end })
}

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

fn is_repeating_pattern(s: &str) -> bool {
    // Only true when the string is exactly two repeated halves.
    if s.len() % 2 != 0 {
        return false;
    }

    let mid = s.len() / 2;
    &s[..mid] == &s[mid..]
}

fn is_repeating_at_least_twice(s: &str) -> bool {
    let len = s.len();
    for size in 1..=len / 2 {
        if len % size != 0 {
            continue;
        }
        let repeats = len / size;
        if repeats < 2 {
            continue;
        }
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

fn is_invalid(n: u64, mode: InvalidMode) -> bool {
    let s = n.to_string();
    match mode {
        InvalidMode::ExactDouble => is_repeating_pattern(&s),
        InvalidMode::AtLeastDouble => is_repeating_at_least_twice(&s),
    }
}

fn sum_invalid_in_range(range: Range, mode: InvalidMode) -> u64 {
    if range.start > range.end {
        eprintln!("Warning: start greater than end in range: {:?}", range);
        return 0;
    }

    (range.start..=range.end)
        .filter(|&n| is_invalid(n, mode))
        .sum()
}

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

    #[test]
    fn repeating_digits_invalid() {
        let invalid_id_sum = sum_of_invalid_ids(["55-56"], InvalidMode::ExactDouble);
        assert_eq!(invalid_id_sum, 55);
    }

    #[test]
    fn repeating_chunk_invalid() {
        let invalid_id_sum = sum_of_invalid_ids(["123123-123123"], InvalidMode::ExactDouble);
        assert_eq!(invalid_id_sum, 123123);
    }

    #[test]
    fn triple_repetition_is_valid() {
        let invalid_id_sum = sum_of_invalid_ids(["123123123-123123123"], InvalidMode::ExactDouble);
        assert_eq!(invalid_id_sum, 0);
    }

    #[test]
    fn odd_length_same_digit_is_valid() {
        let invalid_id_sum = sum_of_invalid_ids(["111-111"], InvalidMode::ExactDouble);
        assert_eq!(invalid_id_sum, 0);
    }

    #[test]
    fn multiple_ranges_count_combines() {
        let invalid_id_sum = sum_of_invalid_ids(["1-2, 55-56"], InvalidMode::ExactDouble);
        assert_eq!(invalid_id_sum, 55);
    }
    
    #[test]
    fn aoc_test_part1() {
        let invalid_id_sum = sum_of_invalid_ids(["11-22,95-115,998-1012,1188511880-1188511890,
        222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,
        824824821-824824827,2121212118-2121212124"], InvalidMode::ExactDouble);
        assert_eq!(invalid_id_sum, 1227775554);
    }
    
     #[test]
    fn aoc_test_part2() {
        let invalid_id_sum = sum_of_invalid_ids(["11-22,95-115,998-1012,1188511880-1188511890,
        222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,
        824824821-824824827,2121212118-2121212124"], InvalidMode::AtLeastDouble);
        assert_eq!(invalid_id_sum, 4174379265);
    }

    #[test]
    fn triple_repetition_becomes_invalid_in_at_least_mode() {
        let invalid_id_sum = sum_of_invalid_ids(["123123123-123123123"], InvalidMode::AtLeastDouble);
        assert_eq!(invalid_id_sum, 123123123);
    }
}
