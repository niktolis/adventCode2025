use std::env;
const INPUT_URL: &str = "https://adventofcode.com/2025/day/1/input";

enum LineStart {
    Right,
    Left,
    Other,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let session = env::var("AOC_SESSION")
        .map_err(|_| "AOC_SESSION environment variable is not set")?;
    let body = ureq::get(INPUT_URL)
        .header("Cookie", &format!("session={session}"))
        .call()?
        .into_body()
        .read_to_string()?;

    let stats = process_lines(50, body.lines());

    println!("Times dial pointed at 0: {}", stats.zero_hits);
    println!("Final value: {}", stats.value);

    Ok(())
}

fn classify_line(line: &str) -> LineStart {
    match line.as_bytes().first().copied() {
        Some(b'R') => LineStart::Right,
        Some(b'L') => LineStart::Left,
        _ => LineStart::Other,
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Stats {
    value: u32,
    zero_hits: u32,
}

fn process_lines<'a, I>(start: u32, lines: I) -> Stats
where
    I: IntoIterator<Item = &'a str>,
{
    let mut value = start % 100;
    let mut zero_hits = 0;

    for line in lines {
        match classify_line(line) {
            LineStart::Right => {
                if let Some(rest) = line.strip_prefix('R') {
                    if let Ok(delta) = rest.trim().parse::<u32>() {
                        zero_hits += zero_hits_right(value, delta);
                        value = ((value as u64 + delta as u64) % 100) as u32;
                    } else {
                        eprintln!("Warning: invalid number after R in line: {line}");
                    }
                }
            }
            LineStart::Left => {
                if let Some(rest) = line.strip_prefix('L') {
                    if let Ok(delta) = rest.trim().parse::<u32>() {
                        zero_hits += zero_hits_left(value, delta);
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

fn zero_hits_right(start: u32, delta: u32) -> u32 {
    ((start as u64 + delta as u64) / 100) as u32
}

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
