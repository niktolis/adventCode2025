use std::env;

const INPUT_URL: &str = "https://adventofcode.com/2025/day/4/input";

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let session = env::var("AOC_SESSION")
        .map_err(|_| "AOC_SESSION environment variable is not set")?;

    let body = ureq::get(INPUT_URL)
        .header("Cookie", &format!("session={session}"))
        .call()?
        .into_body()
        .read_to_string()?;

    println!("{}", body);

    Ok(())

} 

/// All 8 neighbor directions as (dr, dc):
///   (-1,-1) (-1,0) (-1,1)
///   ( 0,-1)        ( 0,1)
///   ( 1,-1) ( 1,0) ( 1,1)

const NEIGHBORS: &[(isize, isize)] = &[
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1)
];

#[allow(dead_code)]
fn process_grid(input: &str) -> u8 {

    let mut count = 0;

    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    count
}
    


