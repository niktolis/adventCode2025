use std::env;

const INPUT_URL: &str = "https://adventofcode.com/2025/day/3/input";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = env::var("AOC_SESSION")
        .map_err(|_| "AOC_SESSION environment variable is not set")?;

    let body = ureq::get(INPUT_URL)
        .header("Cookie", &format!("session={session}"))
        .call()?
        .into_body()
        .read_to_string()?;

    let total_jolts = calculate_total_jolts(body.lines(), 12);

    println!("Total jolts: {}", total_jolts);

    Ok(())
    
}

/// Finds the maximum 2-digit number from a string where digits must be in order.
/// 
/// This is a specialized version for k=2. It scans right-to-left, tracking the
/// maximum digit seen so far in the suffix. For each digit, it forms a 2-digit
/// number with the max suffix digit and keeps track of the best value found.
/// 
/// Example: "987654321111111" -> 98 (9 followed by 8)
#[allow(dead_code)]
fn max_two_digits_ordered(line: &str) -> Option <u8> {
    let bytes = line.as_bytes();
    if bytes.len() < 2 {
        return None;
    }
    let mut max_suffix_digit = -1;  // Track the largest digit seen to the right
    let mut best_value = -1;         // Best 2-digit value found so far

    // Scan from right to left
    for &b in bytes.iter().rev() {
        let d  = (b - b'0') as i8;

        // If we have a suffix digit, form a 2-digit number
        if max_suffix_digit != - 1 {
            let candidate = (d as i16) * 10 + (max_suffix_digit as i16);
            if candidate > best_value {
                best_value = candidate;
            }
        }

        // Update the maximum digit seen in the suffix
        if d > max_suffix_digit {
            max_suffix_digit = d;
        }
    }

    if best_value == -1 {
        None
    } else {
        Some(best_value as u8)
    }
}

/// Finds the maximum k-digit number from a string of digits while preserving order.
/// 
/// Uses a greedy algorithm with a monotonic stack to select k digits that form
/// the largest possible number. The algorithm works by:
/// 1. Processing digits left-to-right
/// 2. Removing smaller digits from the stack if a larger digit appears (when budget allows)
/// 3. Ensuring exactly k digits remain
/// 
/// Example: max_k_digits_ordered("987654321111111", 12) -> 987654321111
///          We remove the three smallest trailing '1's to keep 12 digits
/// 
/// Time: O(n), Space: O(n) where n is the string length
fn max_k_digits_ordered(line: &str, k: usize) -> Option<u128> {

    let bytes = line.as_bytes();
    let n = bytes.len();

    // Edge cases: can't form k digits if k is invalid or exceeds length
    if k == 0 || k > n {
        return None;
    }

    let mut to_remove = n - k;  // How many digits we must discard
    let mut stack: Vec<u8> = Vec::with_capacity(n);

    // Process each digit left-to-right
    for &b in bytes {
        // Validate input is all digits
        if b < b'0' || b > b'9' {
            return None;
        }

        let d = b - b'0';

        // Greedy removal: pop smaller digits when we see a larger one
        // This maintains a monotonic decreasing stack for optimal selection
        while let Some(&last) = stack.last() {
            if to_remove > 0 && last < d {
                stack.pop();
                to_remove -= 1;
            } else {
                break;
            }
        }
        stack.push(d);
    }

    // Remove any excess digits from the end (smallest values)
    while to_remove > 0 {
        stack.pop();
        to_remove -= 1;
    }

    // Take exactly k digits from the stack
    let digits = &stack[..k];

    // Convert digit array to u128 number with overflow checking
    let mut value: u128= 0;
    for &d in digits {
        value = value
            .checked_mul(10)?
            .checked_add(d as u128)?;
    }
    Some(value)
}

/// Calculates the sum of maximum k-digit values across all input lines.
/// 
/// Each line is processed independently to find its maximum k-digit ordered number,
/// then all values are summed. Lines that fail to produce a valid k-digit number
/// contribute 0 to the total.
/// 
/// # Arguments
/// * `lines` - Iterator of string slices, one per puzzle input line
/// * `k` - Number of digits to select from each line
fn calculate_total_jolts<'a, I>(lines: I, k: usize) -> u128
where
    I: IntoIterator<Item = &'a str>,
{
    let mut total_jolts: u128 = 0;
    for line in lines {
        // Extract max k-digit value from this line, default to 0 on failure
        let jolts = max_k_digits_ordered(line, k).unwrap_or(0) as u128;
        total_jolts += jolts;
    }
    total_jolts
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test single line with k=2: "987654321111111" -> 98
    /// Selects '9' and '8' (first two digits in descending order)
    #[test]
    fn aoc_test_part1_one_line() {
        let total_jolts = calculate_total_jolts(["987654321111111"], 2);
        assert_eq!(total_jolts, 98);
    }
    
    /// Test multiple lines with k=2:
    /// Line 1: "987654321111111" -> 98
    /// Line 2: "811111111111119" -> 89 (8 and 9)
    /// Line 3: "234234234234278" -> 88 (7 and 8)
    /// Line 4: "818181911112111" -> 99 (both 9s, but taken as first 9)
    /// Total: 98 + 89 + 78 + 92 = 357
    #[test]
    fn aoc_test_part1_multiple_lines_size2() {
        let total_jolts = calculate_total_jolts(["987654321111111", "811111111111119", "234234234234278", "818181911112111" ], 2);
        assert_eq!(total_jolts, 357);
   }
   
    /// Test multiple lines with k=12 (selecting 12 digits from 15-digit strings)
    /// Validates the greedy algorithm works for larger k values
    #[test]
    fn aoc_test_part1_multiple_lines_size12() {
        let total_jolts = calculate_total_jolts(["987654321111111", "811111111111119", "234234234234278", "818181911112111" ], 12);
        assert_eq!(total_jolts, 3121910778619);
    }
}
