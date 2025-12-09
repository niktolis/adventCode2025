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

#[allow(dead_code)]
fn max_two_digits_ordered(line: &str) -> Option <u8> {
    let bytes = line.as_bytes();
    if bytes.len() < 2 {
        return None;
    }
    let mut max_suffix_digit = -1;
    let mut best_value = -1;

    for &b in bytes.iter().rev() {
        let d  = (b - b'0') as i8;

        if max_suffix_digit != - 1 {
            let candidate = (d as i16) * 10 + (max_suffix_digit as i16);
            if candidate > best_value {
                best_value = candidate;
            }
        }

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

fn max_k_digits_ordered(line: &str, k: usize) -> Option<u128> {

    let bytes = line.as_bytes();
    let n = bytes.len();

    if k == 0 || k > n {
        return None;
    }

    let mut to_remove = n - k;
    let mut stack: Vec<u8> = Vec::with_capacity(n);

    for &b in bytes {
        if b < b'0' || b > b'9' {
            return None;
        }

        let d = b - b'0';

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

    while to_remove > 0 {
        stack.pop();
        to_remove -= 1;
    }

    let digits = &stack[..k];

    let mut value: u128= 0;
    for &d in digits {
        value = value
            .checked_mul(10)?
            .checked_add(d as u128)?;
    }
    Some(value)
}

fn calculate_total_jolts<'a, I>(lines: I, k: usize) -> u128
where
    I: IntoIterator<Item = &'a str>,
{
    let mut total_jolts: u128 = 0;
    for line in lines {
        let jolts = max_k_digits_ordered(line, k).unwrap_or(0) as u128;
        total_jolts += jolts;
    }
    total_jolts
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn aoc_test_part1_one_line() {
        let total_jolts = calculate_total_jolts(["987654321111111"], 2);
        assert_eq!(total_jolts, 98);
    }
    #[test]
    fn aoc_test_part1_multiple_lines_size2() {
        let total_jolts = calculate_total_jolts(["987654321111111", "811111111111119", "234234234234278", "818181911112111" ], 2);
        assert_eq!(total_jolts, 357);
   }
   #[test]
    fn aoc_test_part1_multiple_lines_size12() {
        let total_jolts = calculate_total_jolts(["987654321111111", "811111111111119", "234234234234278", "818181911112111" ], 12);
        assert_eq!(total_jolts, 3121910778619);
    }
}
