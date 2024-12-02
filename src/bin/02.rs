use adv_code_2024::start_day;
use anyhow::{Ok, Result};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::sorted;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

#[allow(clippy::items_after_statements)]
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    #[derive(PartialEq, Debug)]
    enum Direction {
        Increasing,
        Decreasing,
        Unknown,
    }

    fn delta_is_safe(delta: i32, direction: &Direction) -> bool {
        if delta == 0 || delta > 3 || delta < -3 {
            false
        } else {
            match direction {
                Direction::Increasing if delta < 0 => false,
                Direction::Decreasing if delta > 0 => false,
                _ => true
            }
        }
    }

    fn report_is_safe(report: &Vec<i32>, skip: Option<usize>) -> bool {
        let mut direction = Direction::Unknown;
        
        let mut skipped = report.clone();
        if let Some(i) = skip {
            skipped.remove(i);
        }
        for i in 1..skipped.len() {
            let num = skipped[i];
            let last = skipped[i - 1];
            let delta = num - last;

            if delta_is_safe(delta, &direction) {
                if direction == Direction::Unknown {
                    if delta > 0 {
                        direction = Direction::Increasing;
                    } else {
                        direction = Direction::Decreasing;
                    }
                }
            } else {
                return false;
            }
        }
        true
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let reports = reader
            .lines()
            .map_while(Result::ok)
            .map(|line| {
                line.split(" ").map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>()
            }).filter(|report| report_is_safe(report, None)).count();

        Ok(reports)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (safe_reports, maybe_unsafe_reports): (Vec<Vec<i32>>, Vec<Vec<i32>>) = reader
            .lines()
            .map_while(Result::ok)
            .map(|line| {
                line.split(" ").map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>()
            }).partition(|report| report_is_safe(report, None));
    
        let safe_unsafe_reports = maybe_unsafe_reports
            .iter()
            // Try to remove one number from a report until it is safe, or we reach the end
            .filter_map(|report| (0..report.len()).find(|i| report_is_safe(report, Some(*i))));
        
        Ok(safe_reports.len() + safe_unsafe_reports.count())
    }
    
    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    //endregion

    Ok(())
}
