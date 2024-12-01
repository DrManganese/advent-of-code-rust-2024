use adv_code_2024::start_day;
use anyhow::{Ok, Result};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::sorted;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

#[allow(clippy::items_after_statements)]
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u32> {
        let (left, right): (Vec<u32>, Vec<u32>) = reader
            .lines()
            .map_while(Result::ok)
            .map(|line| {
                let (l, r) = line.split_once("   ").unwrap();
                (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap())
            })
            .unzip();
        let sum = sorted(left)
            .zip(sorted(right))
            .map(|(l, r)| l.abs_diff(r))
            .sum::<u32>();

        Ok(sum)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u32> {
        let (left, right): (Vec<u32>, Vec<u32>) = reader
            .lines()
            .map_while(Result::ok)
            .map(|line| {
                let (l, r) = line.split_once("   ").unwrap();
                (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap())
            })
            .unzip();
        let right_occurence_map = right.iter().fold(HashMap::new(), |mut acc, num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        });
        let similarity = left
            .iter()
            .map(|l| right_occurence_map.get(l).unwrap_or(&0) * l)
            .sum::<u32>();
        Ok(similarity)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    //endregion

    Ok(())
}
