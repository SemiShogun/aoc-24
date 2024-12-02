
fn main() {
    let input: &str = include_str!("./input.txt");
    let output1 = part1(input);
    dbg!(output1);

    let output2 = part2(input);
    dbg!(output2);
}

fn is_safe(report: &[i32]) -> bool {
    if report.len() < 2 {
        return false;
    }

    let is_increasing = report.windows(2).all(|pair| pair[1] > pair[0] && pair[1] - pair[0] <= 3);
    let is_decreasing = report.windows(2).all(|pair| pair[1] < pair[0] && pair[0] - pair[1] <= 3);

    is_increasing || is_decreasing
}

fn dampener_check(report: &[i32]) -> bool {

    if is_safe(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut recheck_report = report.to_vec();
        recheck_report.remove(i);
        if is_safe(&recheck_report) {
            return true;
        }
    }
    
    false
}

fn part1(input: &str) -> String {
    let reports: Vec<Vec<i32>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    return reports
        .iter()
        .filter(|report| is_safe(report))
        .count()
        .to_string();
}

fn part2(input: &str) -> String {
    let reports: Vec<Vec<i32>> = input
    .lines()
    .filter(|line| !line.is_empty())
    .map(|line| {
        line.split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect()
    })
    .collect();

    return reports
        .iter()
        .filter(|report| dampener_check(report))
        .count()
        .to_string();
}