use std::env;
use std::collections::VecDeque;

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

#[derive(Debug, Clone)]
struct Stats {
    out: Grid,
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
            let stats = process_grid_single(&grid);
            println!("\n Final Grid with removed accessible rolls");
            print_grid(&stats.out);
            println!("\nSINGLE: total removed = {}", stats.total_removed);
            Ok(())
        }
        "multi" => {
            println!("Running MULTI pass ...");
            let stats = process_grid_multi(&grid);
            println!("\n Final Grid with removed accessible rolls");
            print_grid(&stats.out);
            println!("\nMULTI: passes = {}, total removed = {}",stats.passes, stats.total_removed);
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

fn count_adjacent_rolls(grid : &Grid, r: usize, c: usize) -> u8 {
    
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

fn process_grid_single(grid: &Grid) -> Stats {

    let mut total_removed: usize = 0;
    let passes = 0usize;

    let rows = grid.len() as usize;
    let cols = grid[0].len() as usize;

    // This will hold a marking of accessibility:
    // 'x' = accessible '@'
    // '@' = non-accessible '@'
    // '.' = empty

    let mut out: Grid = vec![vec!['.'; cols as usize]; rows as usize];

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != '@' {
                continue;
            }

            let adj_rolls = count_adjacent_rolls(&grid, r, c);

            if adj_rolls < 4 {
                out[r][c] = 'x';
                total_removed += 1;
            } else {
                out[r][c] = '@';
            }
        }
    }
    
    Stats {
        out,
        passes,
        total_removed
    }
}

fn process_grid_multi(grid: &Grid) -> Stats {
    
    let mut out = grid.clone();
    let mut total_removed = 0usize;
    let mut passes = 0usize;
    
    let rows = out.len();
    let cols = out[0].len();

    // degree[r][c] = how many rolls neighbors cell (r,c) currently has
    let mut degree =  vec![vec![0u8; cols]; rows];

    // 1) compute initial degrees using the shared count_adjacent_rolls

    for r in 0..rows {
        for c in 0..cols {
            if out[r][c] == '@' {
                degree[r][c] = count_adjacent_rolls(&out, r, c);
            }
        }
    }

    // 2) initial queue: all cells with '@' and degree < 4
    let mut queue = VecDeque::new();
    let mut in_queue = vec![vec![false; cols]; rows];
    
    for r in 0..rows {
        for c in 0..cols {
            if out[r][c] == '@' && degree[r][c] < 4 {
                queue.push_back((r,c));
                in_queue[r][c] = true;
            }
        }
    }

    // 3) process in passes
    while !queue.is_empty() {
        passes += 1;
        let mut removed_this_wave = 0usize;

        let layer_size = queue.len();
        for _ in 0..layer_size {
            let (r,c) = queue.pop_front().unwrap();
            in_queue[r][c] = false;

            if out[r][c] != '@' {
                continue; // it might have been removed already
            }

            // remove this roll
            out[r][c] = 'x';
            total_removed += 1;
            removed_this_wave += 1;

            // update neighbors' degrees
            for (dr,dc) in NEIGHBORS {
                let nr = r as isize + dr;
                let nc = c as isize + dc;

                if nr < 0 || nr >= rows as isize || nc < 0 || nc >= cols as isize {
                    continue;
                }
                let (ur, uc) = (nr as usize, nc as usize);

                if out[ur][uc] != '@' {
                    continue;
                }

                if degree[ur][uc] > 0 {
                    degree[ur][uc] -= 1;
                }

                if degree[ur][uc] < 4 && !in_queue[ur][uc] {
                    queue.push_back((ur, uc));
                    in_queue[ur][uc] = true;
                }
            }
        }
        println!("Pass {passes}: removed {removed_this_wave} rolls");
    }

    Stats {
            out,
            passes,
            total_removed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to create a grid from a string representation
    fn grid_from_str(s: &str) -> Grid {
        s.lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect())
            .collect()
    }

    /// Helper to count '@' symbols in a grid
    fn count_rolls(grid: &Grid) -> usize {
        grid.iter()
            .flat_map(|row| row.iter())
            .filter(|&&c| c == '@')
            .count()
    }

    /// Helper to count 'x' symbols (removed rolls) in a grid
    fn count_removed(grid: &Grid) -> usize {
        grid.iter()
            .flat_map(|row| row.iter())
            .filter(|&&c| c == 'x')
            .count()
    }

    #[test]
    fn test_empty_grid() {
        let grid = grid_from_str("...\n...\n...");
        
        let stats_single = process_grid_single(&grid);
        assert_eq!(stats_single.total_removed, 0);
        assert_eq!(count_removed(&stats_single.out), 0);
        
        let stats_multi = process_grid_multi(&grid);
        assert_eq!(stats_multi.total_removed, 0);
        assert_eq!(stats_multi.passes, 0);
    }

    #[test]
    fn test_single_roll() {
        let grid = grid_from_str("...\n.@.\n...");
        
        // Single roll has 0 neighbors, should be removed
        let stats_single = process_grid_single(&grid);
        assert_eq!(stats_single.total_removed, 1);
        assert_eq!(count_rolls(&stats_single.out), 0);
        
        let stats_multi = process_grid_multi(&grid);
        assert_eq!(stats_multi.total_removed, 1);
        assert_eq!(stats_multi.passes, 1);
    }

    #[test]
    fn test_two_by_two_grid() {
        // 2x2 grid: each cell has exactly 3 neighbors
        let grid = grid_from_str("@@\n@@");
        
        // All should be removed in single pass (each has 3 < 4 neighbors)
        let stats_single = process_grid_single(&grid);
        assert_eq!(stats_single.total_removed, 4);
        assert_eq!(count_rolls(&stats_single.out), 0);
        
        // Multi-pass should also remove all, but might take multiple passes
        let stats_multi = process_grid_multi(&grid);
        assert_eq!(stats_multi.total_removed, 4);
        assert!(stats_multi.passes > 0);
    }

    #[test]
    fn test_three_by_three_all_rolls() {
        // 3x3 grid of all rolls:
        // Corners have 3 neighbors, edges have 5, center has 8
        let grid = grid_from_str("@@@\n@@@\n@@@");
        
        // Single pass: only corners removed (3 < 4)
        let stats_single = process_grid_single(&grid);
        assert_eq!(stats_single.total_removed, 4); // 4 corners
        assert_eq!(count_rolls(&stats_single.out), 5); // center + 4 edges remain
        
        // Multi-pass: all should eventually be removed
        let stats_multi = process_grid_multi(&grid);
        assert_eq!(stats_multi.total_removed, 9);
        assert_eq!(count_rolls(&stats_multi.out), 0);
        assert!(stats_multi.passes > 1); // Should take multiple passes
    }

    #[test]
    fn test_single_vs_multi_difference() {
        // Pattern where single and multi give different results
        // Cross pattern: center has 4 neighbors (not removed in single)
        // but edges have only 1 neighbor (removed in single)
        let grid = grid_from_str(".@.\n@@@\n.@.");
        
        // Single: removes 4 edge cells (each has 1 neighbor), center remains
        let stats_single = process_grid_single(&grid);
        assert_eq!(stats_single.total_removed, 4);
        assert_eq!(count_rolls(&stats_single.out), 1); // center remains
        
        // Multi: after edges removed, center has 0 neighbors, gets removed too
        let stats_multi = process_grid_multi(&grid);
        assert_eq!(stats_multi.total_removed, 5);
        assert_eq!(count_rolls(&stats_multi.out), 0);
        assert_eq!(stats_multi.passes, 2); // Two passes needed
    }

    #[test]
    fn test_isolated_groups() {
        // Two separate groups of rolls
        let grid = grid_from_str("@@...@@\n@@...@@");
        
        // Each cell has 3 neighbors, all removed
        let stats_single = process_grid_single(&grid);
        assert_eq!(stats_single.total_removed, 8);
        
        let stats_multi = process_grid_multi(&grid);
        assert_eq!(stats_multi.total_removed, 8);
    }

    #[test]
    fn test_stable_configuration() {
        // 4x4 grid: corners have 3, edges have 5, 4 interior cells have 8
        let grid = grid_from_str("@@@@\n@@@@\n@@@@\n@@@@");
        
        // Single: removes corners (3 < 4), 12 remain
        let stats_single = process_grid_single(&grid);
        assert_eq!(stats_single.total_removed, 4); // 4 corners only
        assert_eq!(count_rolls(&stats_single.out), 12);
        
        // Multi: also only removes corners, then structure stabilizes
        // After removing corners, edges have 4 neighbors (stable), interior has 7
        let stats_multi = process_grid_multi(&grid);
        assert_eq!(stats_multi.total_removed, 4); // Same as single
        assert_eq!(stats_multi.passes, 1);
        assert_eq!(count_rolls(&stats_multi.out), 12); // Same 12 remain
    }

    #[test]
    fn test_count_adjacent_rolls() {
        let grid = grid_from_str("@@@\n@@@\n@@@");
        
        // Center cell should have 8 neighbors
        assert_eq!(count_adjacent_rolls(&grid, 1, 1), 8);
        
        // Corner should have 3 neighbors
        assert_eq!(count_adjacent_rolls(&grid, 0, 0), 3);
        
        // Edge should have 5 neighbors
        assert_eq!(count_adjacent_rolls(&grid, 0, 1), 5);
    }

    #[test]
    fn test_count_adjacent_with_gaps() {
        let grid = grid_from_str("@.@\n.@.\n@.@");
        
        // Center has 4 diagonal neighbors
        assert_eq!(count_adjacent_rolls(&grid, 1, 1), 4);
        
        // Corners have 1 neighbor each
        assert_eq!(count_adjacent_rolls(&grid, 0, 0), 1);
        assert_eq!(count_adjacent_rolls(&grid, 0, 2), 1);
    }

    #[test]
    fn test_boundary_cells() {
        // Test cells on boundaries
        let grid = grid_from_str("@\n@");
        
        // Each has 1 neighbor
        let stats_single = process_grid_single(&grid);
        assert_eq!(stats_single.total_removed, 2);
        
        let stats_multi = process_grid_multi(&grid);
        assert_eq!(stats_multi.total_removed, 2);
    }

    #[test]
    fn test_line_of_rolls() {
        // Horizontal line
        let grid = grid_from_str("@@@@@");
        
        // Ends have 1 neighbor, middle ones have 2 - all < 4
        let stats_single = process_grid_single(&grid);
        assert_eq!(stats_single.total_removed, 5); // All removed
        
        // Multi: all cells start with < 4 neighbors, so all queued initially
        let stats_multi = process_grid_multi(&grid);
        assert_eq!(stats_multi.total_removed, 5);
        assert_eq!(stats_multi.passes, 1); // All removed in first pass
    }

    #[test]
    fn test_multi_pass_cascading() {
        // Pattern designed to test cascading removal
        // Square with hole in middle
        let grid = grid_from_str("@@@@@\n@...@\n@...@\n@...@\n@@@@@");
        
        // Single: corners have 3, some edges have fewer
        let stats_single = process_grid_single(&grid);
        assert!(stats_single.total_removed > 0);
        
        // Multi: should remove everything, but all in one pass since
        // all cells with < 4 neighbors are found initially
        let stats_multi = process_grid_multi(&grid);
        assert_eq!(stats_multi.total_removed, 16); // All rolls removed
        assert_eq!(stats_multi.passes, 1); // All removed in first pass
    }

    #[test]
    fn test_stable_core_pattern() {
        // 5x5 grid: corners have 3 neighbors (removed), but after removal
        // edge cells have exactly 4 neighbors (stable), preventing further cascading
        // This demonstrates a pattern where multi-pass doesn't remove everything
        let grid = grid_from_str("@@@@@\n@@@@@\n@@@@@\n@@@@@\n@@@@@");
        
        // Single: only corners removed (3 < 4)
        let stats_single = process_grid_single(&grid);
        assert_eq!(stats_single.total_removed, 4); // 4 corners
        assert!(count_rolls(&stats_single.out) > 0);
        
        // Multi: only corners removed in pass 1, then remaining cells are stable
        // After removing corners, edge cells have 4 neighbors (not < 4), so they remain
        let stats_multi = process_grid_multi(&grid);
        assert_eq!(stats_multi.total_removed, 4); // Only corners, same as single
        assert_eq!(stats_multi.passes, 1); // Only one pass needed
        assert_eq!(count_rolls(&stats_multi.out), 21); // 25 - 4 = 21 remain
    }

    #[test]
    fn test_aoc_pattern_single_vs_multi() {
        // Complex real-world pattern with mixed densities
        let input = "..@@.@@@@.\n\
                     @@@.@.@.@@\n\
                     @@@@@.@.@@\n\
                     @.@@@@..@.\n\
                     @@.@@@@.@@\n\
                     .@@@@@@@.@\n\
                     .@.@.@.@@@\n\
                     @.@@@.@@@@\n\
                     .@@@@@@@@.\n\
                     @.@.@@@.@.";
        
        let grid = grid_from_str(input);
        
       
        // Test single pass
        let stats_single = process_grid_single(&grid);
        assert_eq!(stats_single.total_removed, 13);       
        
        // Test multi pass
        let stats_multi = process_grid_multi(&grid);
        assert_eq!(stats_multi.total_removed, 43);
   
    }
}


