use std::env;

const INPUT_URL: &str = "https://adventofcode.com/2025/day/4/input";

/// All 8 neighbor directions as (dr, dc):
///   (-1,-1) (-1,0) (-1,1)
///   ( 0,-1)        ( 0,1)
///   ( 1,-1) ( 1,0) ( 1,1)

const NEIGHBORS: &[(isize, isize)] = &[
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1)
];

type Grid = Vec<Vec<char>>;

#[derive(Debug, Clone, Copy)]
struct Stats {
    passes : usize, // how many "waves" happened until no more rolls are accessible
    total_removed: usize, // how many rolls were removed in total
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Mode selection: single or multi pass (default: single)
   let mode = env::args().nth(1).unwrap_or_else(|| "single".to_string());

    let session = env::var("AOC_SESSION")
        .map_err(|_| "AOC_SESSION environment variable is not set")?;

    let body = ureq::get(INPUT_URL)
        .header("Cookie", &format!("session={session}"))
        .call()?
        .into_body()
        .read_to_string()?;

    let grid = process_input_grid(&body);

    println!("=== Original grid ===");
    print_grid(&grid);
    println!();

    match mode.as_str() {
        "single" => {
            println!("Running SINGLE pass marking ...");
            let total_removed = process_grid_single(grid);
            Ok(())
        }
        "multi" => {
            println!("Running MULTI pass ...");
            let mut grid_multi = grid.clone();
            let stats = process_grid_multi(grid_multi);
            Ok(())
        }
        _ => {
            Err(Box::from(format!("Unknown mode: '{}'", mode)))
        }
    }

}

fn process_input_grid(s: &str) -> Grid {

    s.lines().map(|line| line.chars().collect()).collect()
    
}

fn print_grid(grid: &Grid) {
    for row in grid {
        let line: String = row.iter().collect();
        println!("{line}")
    }
}

fn count_adjacent_rolls(grid : &Vec<Vec<char>>, r: usize, c: usize) -> u8 {
    
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    let r = r as isize;
    let c = c as isize;

    let mut adj_rolls = 0u8;

    for (dr, dc) in NEIGHBORS {
        let nr = r + dr;
        let nc = c + dc;

        if nr < 0 || nr >= rows || nc < 0 || nc >= cols {
            continue;
        }

        let (ur, uc) = (nr as usize, nc as usize);

        if grid[ur][uc] == '@' {
            adj_rolls += 1;
        }
    }

    adj_rolls
}

fn process_grid_single(grid: Grid) -> u32 {

    let mut count: u32 = 0;

    let rows = grid.len() as usize;
    let cols = grid[0].len() as usize;

    // This will hold a marking of accessibility:
    // 'x' = accessible '@'
    // '@' = non-accessible '@'
    // '.' = empty

    let mut out = vec![vec!['.'; cols as usize]; rows as usize];

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != '@' {
                continue;
            }

            let adj_rolls = count_adjacent_rolls(&grid, r, c);

            if adj_rolls < 4 {
                out[r][c] = 'x';
                count += 1;
            } else {
                out[r][c] = '@';
            }
        }
    }

   // println!("{:?}",grid); 

    count
}

fn process_grid_multi(grid: Grid) -> Stats {
    
    let mut total_removed = 0usize;
    let mut passes = 0usize;

    Stats {
            passes,
            total_removed
    }
}


