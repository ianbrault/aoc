/*
** src/puzzles/aoc2024/day2.rs
*/

use super::Solution;
use crate::utils;

use log::debug;

fn deltas(report: &[i64]) -> Vec<i64> {
    report
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, x)| x - report[i - 1])
        .collect()
}

fn report_is_safe(report: &[i64]) -> bool {
    let deltas = deltas(report);
    let increasing = deltas.iter().all(|&d| d > 0 && d <= 3);
    let decreasing = deltas.iter().all(|&d| (-3..0).contains(&d));
    debug!("report: {:?}: safe: {}", report, increasing || decreasing);
    increasing || decreasing
}

fn report_is_safe_dampener(report: &[i64]) -> bool {
    let mut safe = false;
    let diff = deltas(report);
    let increasing_count = diff.iter().filter(|&&d| d > 0 && d <= 3).count();
    let decreasing_count = diff.iter().filter(|&&d| (-3..0).contains(&d)).count();
    if increasing_count == diff.len() || decreasing_count == diff.len() {
        safe = true;
    } else if increasing_count >= diff.len() - 2 {
        let bad_delta_pos = diff.iter().position(|&d| d <= 0 || d > 3).unwrap();
        // remove the item on the left side of the bad delta
        let report_a = [&report[..bad_delta_pos], &report[(bad_delta_pos + 1)..]].concat();
        let deltas_a = deltas(&report_a);
        if deltas_a.iter().all(|&d| d > 0 && d <= 3) {
            safe = true;
        }
        // remove the item on the right side of the bad delta
        let report_b = [
            &report[..(bad_delta_pos + 1)],
            &report[(bad_delta_pos + 2)..],
        ]
        .concat();
        let deltas_b = deltas(&report_b);
        if deltas_b.iter().all(|&d| d > 0 && d <= 3) {
            safe = true;
        }
    } else if decreasing_count >= diff.len() - 2 {
        let bad_delta_pos = diff.iter().position(|&d| !(-3..0).contains(&d)).unwrap();
        // remove the item on the left side of the bad delta
        let report_a = [&report[..bad_delta_pos], &report[(bad_delta_pos + 1)..]].concat();
        let deltas_a = deltas(&report_a);
        if deltas_a.iter().all(|&d| (-3..0).contains(&d)) {
            safe = true;
        }
        // remove the item on the right side of the bad delta
        let report_b = [
            &report[..(bad_delta_pos + 1)],
            &report[(bad_delta_pos + 2)..],
        ]
        .concat();
        let deltas_b = deltas(&report_b);
        if deltas_b.iter().all(|&d| (-3..0).contains(&d)) {
            safe = true;
        }
    }

    debug!("report: {:?}: safe: {}", report, safe);
    safe
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // The unusual data consists of many reports, one report per line. Each report is a list of
    // numbers called levels that are separated by spaces.
    let reports = input
        .split('\n')
        .map(|line| utils::split_and_parse::<i64>(line).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Part A: A report only counts as safe if both the levels are either all increasing or all
    // decreasing and any two adjacent levels differ by at least one and at most three. How many
    // reports are safe?
    let safe_reports = reports
        .iter()
        .filter(|report| report_is_safe(report))
        .count();
    solution.set_part_a(safe_reports);

    // Part B: The Problem Dampener is a reactor-mounted module that lets the reactor safety
    // systems tolerate a single bad level in what would otherwise be a safe report. Update your
    // analysis by using the Problem Dampener. How many reports are now safe?
    let safe_reports_dampened = reports
        .iter()
        .filter(|report| report_is_safe_dampener(report))
        .count();
    solution.set_part_b(safe_reports_dampened);

    solution
}
