use std::env;

const INPUT_URL: &str = "https://adventofcode.com/2025/day/5/input";


/// Inclusive interval [start, end]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Interval {
    start: i64,
    end: i64
}

/// Parse the input format:
/// - First section: lines of "a-b" ranges
/// Then blank line as separator
/// - Second section one number per line
fn parse_input(input: &str) -> (Vec<Interval>, Vec<i64>) {
    let mut ranges: Vec<Interval> = Vec::new();
    let mut numbers: Vec<i64> = Vec::new();

    // Once we hit the empty line, we switch from reading ranges to reading numbers
    let mut in_numbers = false;

    for raw in input.lines() {
        let line = raw.trim();

        // Blank line separates the two sections
        if line.is_empty() {
            in_numbers = true;
            continue;
        }

        if !in_numbers {
            // Expect "a-b"
            let (a,b) = line
                .split_once('-')
                .unwrap_or_else(|| panic!("Bad range line '{line}', expected a-b"));

            let mut start: i64 = a.trim().parse().expect("Bad range start");
            let mut end: i64 = b.trim().parse().expect("Bad range end");

            // Normalize in case a > b
            if start > end {
                std::mem::swap(&mut start, &mut end);
            }

            ranges.push(Interval { start, end });
        } else {
            // Expect a single integer
            numbers.push(line.parse().expect("Bad number"));
        }
    }

    (ranges, numbers)
}

/// Merge ranges so that the result is:
/// - sorted by start
/// - non-overlapping
/// - inclusive-merged (touching intervals are merged too)
/// 
/// Example:
/// [3,5] + [10,14] + [12,18] + [16,20]
/// sorts to [3,5], [10,14] [12, 18], [16,20]
/// merges to [3,5], [10,20]
fn merge_intervals(mut v: Vec<Interval>) -> Vec<Interval> {
    
    // Sort by start, then end (stable enough for merging)
    v.sort_by(|a, b| a.start.cmp(&b.start).then_with(|| a.end.cmp(&b.end)));

    let mut merged: Vec<Interval> = Vec::with_capacity(v.len());

    for it in v {
        if let Some(last) = merged.last_mut() {
            // Because intervals are inclusive, we merge if:
            // - overlapping: it.start <= last.end
            // - or directly adjacent: it.start == last.end + 1
            //
            // Use saturating_add(1) to avoid overflow at i64::MAX.
            if it.start <= last.end.saturating_add(1) {
                // Extend the current merged interval if needed
                if it.end > last.end {
                    last.end = it.end;
                }
                continue; // merged into `last`
            }
        }
        // Disjoint interval: start a new merged block
        merged.push(it);
    }
    merged
}

/// Check if x belongs to any merged interval.
/// Merged intervals are sorted by start and disjoint.
/// 
/// We do binary search for the first interval with start > x.
/// Then the candidate is the interval just before that (idx-1)
/// because it has the largest start <= x.
fn contains(merged: &[Interval], x: i64) -> bool {
    // partition_point returns the first index where predicate is false.
    // Here predicate is: interval.start <= x
    // So idx = number of intervals with start <= x.
    let idx = merged.partition_point(|it| it.start <= x);

    if idx == 0 {
        // All intervals.start > x, so x can't be inside any interval.
        return false;
    }

    // Candidate interval: last one with start <= x
    let it = merged[idx - 1];
    x <= it.end
}


/// Counts the total number of integers contained in all merged intervals.
/// 
/// For each inclusive interval [start, end], the count of integers is:
/// (end - start + 1)
/// 
/// This is computed as (end - start) + 1 to handle the inclusive boundaries.
/// 
/// # Arguments
/// * `merged` - Slice of non-overlapping, sorted intervals
/// 
/// # Returns
/// Total count of all integers across all intervals
/// 
/// # Example
/// Intervals [3,5] and [10,14] contain:
/// - [3,5]: 3 integers (3, 4, 5)
/// - [10,14]: 5 integers (10, 11, 12, 13, 14)
/// - Total: 8 integers
fn count_interval_members(merged: &[Interval]) -> u64 {

    let mut count = 0;

    for it in merged {
        // For inclusive interval [start, end], count = (end - start) + 1
        count += (it.end - it.start) as u64;
        count += 1
    }

    count
}

/// Process input to find the amount of numbers belonging to a range
/// return the value.
fn process_input_part1(input: &str) -> u64 {
    let(ranges, numbers) = parse_input(input);
    let merged = merge_intervals(ranges);

    let mut count = 0;
    for x in numbers {
        if contains(&merged, x) {
            count += 1;
        }
    }
    count
}

fn process_input_part2(input: &str) -> u64 {
    let (ranges, _numbers) = parse_input(input);
    let merged = merge_intervals(ranges);

    let count = count_interval_members(&merged);

    count
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let session = env::var("AOC_SESSION")
        .map_err(|_| "AOC_SESSION environment variable is not set")?;

    let body = ureq::get(INPUT_URL)
        .header("Cookie", &format!("session={session}"))
        .call()?
        .into_body()
        .read_to_string()?;


    let count = process_input_part1(&body);

    println!("{}", count);

    let total = process_input_part2(&body);

    println!("{}", total);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_test_part1() {

         let input = "\
         3-5
         10-14
         16-20
         12-18

         1
         5
         8
         11
         17
         32
         ";

     assert_eq!(process_input_part1(input), 3);
    
    }
    
    #[test]
    fn aoc_test_part2() {
       let input = "\
         3-5
         10-14
         16-20
         12-18
         ";

        assert_eq!(process_input_part2(input), 14)

 
    }
}    
