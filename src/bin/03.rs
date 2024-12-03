use adv_code_2024::start_day;
use anyhow::{Ok, Result};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use regex::{Captures, Match, Regex};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const TEST2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

#[allow(clippy::items_after_statements)]
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn extract_and_do_mul(captures: Captures) -> u32 {
        captures["first"].parse::<u32>().unwrap() * captures["second"].parse::<u32>().unwrap()
    }
    
    fn part1<R: BufRead>(reader: R) -> Result<u32> {
        let mul_match = Regex::new(r"mul\((?<first>\d{1,3}),(?<second>\d{1,3})\)")?;
        let sum: u32 = reader
            .lines()
            .map_while(Result::ok)
            .map(|line| mul_match.captures_iter(&line).map(extract_and_do_mul).sum::<u32>())
            .sum();

        Ok(sum)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u32> {
        // Match either do(), don't() or mul(first,second)
        let regex = Regex::new(r"(?:(do\(\)|don't\(\))|mul\((\d{1,3}),(\d{1,3})\))").unwrap();

        // This was tricky, I first assumed that each row in the input was to be handled separately,
        // and then each row's results would need to be summed. That's not the case. A don't on the
        // previous line still applies on mul()s on the second one.
        let data = reader.lines().map_while(Result::ok).join("");

        // Basically a for loop where we update 'enabled' when we see do or dont, and then add
        // the result of the mul to the sum if 'enabled' is true
        let sum = regex.captures_iter(&data)
            .fold((true, 0), |(enabled, sum), capture| {
                if let Some(do_dont) =  capture.get(1) {
                    match do_dont.as_str() {
                        "do()" => (true, sum),
                        "don't()" => (false, sum),
                        _ => unreachable!()
                        }
                } else if enabled {
                    let first = capture[2].parse::<u32>().unwrap();
                    let second = capture[3].parse::<u32>().unwrap();
                    let new_sum = sum + first * second;
                    (enabled, new_sum)
                } else {
                    (enabled, sum)
                }
            }).1;

        Ok(sum)
    }
    
    assert_eq!(48, part2(BufReader::new(TEST2.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    //endregion

    Ok(())
}
