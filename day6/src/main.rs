use std::env;

const INPUT_URL: &str = "https://adventofcode.com/2025/day/6/input";

/// Returns non-empty lines (trimming only for emptiness; keeps original spacing).
#[inline]
fn non_empty_lines<'a>(input: &'a str) -> Vec<&'a str> {
    input.lines().filter(|l| !l.trim().is_empty()).collect()
}

/// Parse operator tokens from a whitespace-separated line (`+` or `*`)
#[inline]
fn parse_ops_tokens(line: &str) -> Vec<u8> {
    line.split_whitespace()
        .map(|t| {
            let b = t.as_bytes()[0];
            debug_assert!(b == b'+' || b == b'*');
            b
        })
        .collect()
}

/// Fast integer scanner over a byte slice, collecting all unsigned ints.
/// (AoC inputs are well-formed; we keep this tight.)
#[inline]
fn parse_u128_ws(bytes: &[u8], out: &mut Vec<u128>) {
    out.clear();
    let mut i = 0usize;
    while i < bytes.len() {
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }
        let mut v: u128 = 0;
        while i < bytes.len() {
            let c = bytes[i];
            if !c.is_ascii_digit() {
                break;
            }
            v = v * 10 + (c - b'0') as u128;
            i += 1;
        }
        out.push(v);
        while i < bytes.len() && !bytes[i].is_ascii_digit() {
            i += 1;
        }
    }
}

/// Generic block splitterL returns contiguous [start, end) ranges of non-separator columns.
#[inline]
fn split_blocks<F>(width: usize, mut is_sep: F) -> Vec<(usize, usize)>
where
    F: FnMut(usize) -> bool,
{
    let mut blocks = Vec::new();
    let mut c = 0usize;
    while c < width {
        while c < width && is_sep(c) {
            c += 1;
        }
        if c >= width {
            break;
        }
        let start = c;
        while c < width && !is_sep(c) {
            c += 1;
        }
        blocks.push((start, c));
    }
    blocks
}

/// Process input for AoC challenge day 6 part 1
/// 
/// Input format:
/// - N lines of numbers (whitespace separated)
/// - last line contains N-ary operators: '+' or '*', also whitepspace separated
/// 
/// Each column is one "problem": combine all numbers in that column using the operator
/// Then sum all column results
/// 
fn process_input_part1(input: &str) -> u128 {
    // Keep non-empty lines (trailing newline is common).
    let mut lines: Vec<&str> = non_empty_lines(input);
    assert!(!lines.is_empty(), "empty input");

    // Last line = operators
    let op_line = lines.pop().unwrap();
    let ops: Vec<u8> = parse_ops_tokens(op_line);
    let cols = ops.len();
    assert!(cols > 0, "no operators found");

    // Column accumulators; initialized based on op
    let mut acc: Vec<u128> = Vec::with_capacity(cols);
    acc.resize(cols, 0);
    for (i, &op) in ops.iter().enumerate() {
        acc[i] = if op == b'+' { 0 } else { 1 };
    }

    let mut tmp_nums: Vec<u128> = Vec::new();
    
    // Previous lines = operand rows
    for (r, line) in lines.iter().enumerate() {
       parse_u128_ws(line.as_bytes(), &mut tmp_nums);
       if tmp_nums.len() != cols {
            panic!(
                "row {} has {} numbers but operator row has {}",
                r,
                tmp_nums.len(),
                cols
            );
        }
        for i in 0..cols {
            let v = tmp_nums[i];
            if ops[i] == b'+' {
                acc[i] += v;
            } else {
                acc[i] *= v;
            }
        }
    }

    acc.into_iter().sum()

}

/// Process input for AoC challenge day 6 part 2
/// 
/// Input format:
/// - N lines of numbers (whitespace separated)
/// - last line contains N-ary operators: '+' or '*', also whitepspace separated
/// 
/// - interpret input as fixed-width grid
/// - split into blocks by "all-space columns"
/// - for each block, each character-column with digits is one operant (top -> bottom)
/// - operator is in the bottom row somewhere within the block
/// 
fn process_input_part2(input: &str) -> u128 {
    let mut lines = non_empty_lines(input);
    assert!(lines.len() >= 2, "need number rows + operator row");

    let op_line = lines.pop().unwrap();
    let num_lines = lines;

    // Compute width and pad all rows to the same width to allow O(1) indexing.
    let width = std::iter::once(op_line.len())
        .chain(num_lines.iter().map(|l| l.len()))
        .max()
        .unwrap();

    #[inline]
    fn pad_to_width(s: &str, width: usize) -> Vec<u8> {
        let mut v = s.as_bytes().to_vec();
        v.resize(width, b' ');
        v
    }

    let op_row = pad_to_width(op_line, width);
    let rows: Vec<Vec<u8>> = num_lines
        .iter()
        .map(|l| pad_to_width(l, width))
        .collect();

    // Column is separator if its spaces in every row including op row.
    let is_sep = |c: usize| -> bool {
        if op_row[c] != b' ' {
            return false;
        }
        for r in &rows {
            if r[c] != b' ' {
                return false;
            }
        }
        true
    };

    // Split into contiguous non-seprator blocks [start, end)
    let blocks = split_blocks(width, is_sep);
    
    let mut total = 0;

    for (start, end) in blocks {
        // find operator within this block
        let mut op: u8 = 0;
        for c in start..end {
            let ch = op_row[c];
            if ch == b'+' || ch == b'*' {
                op = ch;
                break;
            }
        }
        assert!(op == b'+' || op == b'*', "no operator in block");

        //fold operands on the fly (avoid storing operands Vec)
        let mut block_acc: u128 = if op == b'+' { 0 } else { 1 };
        
        eprintln!("Block [{}, {}): op={}", start, end, op as char);

        for c in start..end {
            // Build number from digits in this column, top->bottom, skipping spaces
            let mut have_digit = false;
            let mut val = 0;

            for r in &rows {
                let ch = r[c];
                if ch.is_ascii_digit() {
                    have_digit = true;
                    val = val * 10 + (ch - b'0') as u128;
                } else {
                    // Only spaces expected in the grid area
                    debug_assert!(ch== b' ')
                }
            }
            if have_digit {
                eprintln!("  col {}: val={}", c, val);
                if op == b'+' {
                    block_acc += val;
                } else {
                    block_acc *= val;
                    
                }
            }
        }
        eprintln!("  block_acc={}", block_acc);
        total += block_acc;
    }

    total
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let mode = args.next().unwrap_or_else(|| "part1".to_string());

    let session = env::var("AOC_SESSION")
        .map_err(|_| "AOC_SESSION environment variable is not set")?;

    let body = ureq::get(INPUT_URL)
        .header("Cookie", &format!("session={session}"))
        .call()?
        .into_body()
        .read_to_string()?;

   match mode.as_str() {
        "part1" | "1" => {
           let grand_total = process_input_part1(&body);
           println!("Part1: Grand total is: {}", grand_total);
           Ok(())
        },
        "part2" | "2" => {
           let grand_total = process_input_part2(&body);
           println!("Part2: Grand total is: {}", grand_total);
           Ok(())
        },
        _ => {
            return Err(format!(
                "Invalid mode '{mode}'. Use 'part1' or 'part2'."
            )
            .into())

        }
    }   
   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_test_part1() {
        let input = "\
123 328  51 64
45 64  387 23
6 98  215 314
*   +   *   +
";
        assert_eq!(process_input_part1(input), 4277556)
    }

    #[test]
    fn aoc_test_part2() {
       let input = "\
123  328   51   64
 45  64   387   23
  6  98   215  314
  *   +     *    +
";
        assert_eq!(process_input_part2(input), 3263827)
    }
}
