use regex::Regex;

fn main() {
    let input: &str = include_str!("./input.txt");
    let output1 = part1(input);
    dbg!(output1);

    let output2 = part2(input);
    dbg!(output2);
}


fn part1(input: &str) -> String {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let mut sum = 0;

    for item in re.captures_iter(input) {
        let mut full_string: String = item[0].split("mul(").collect();
        full_string = full_string.split(")").collect();
        let mut parts = full_string.split(",");
        let first = parts.next().unwrap().parse::<i32>().unwrap();
        let second = parts.next().unwrap().parse::<i32>().unwrap();
        // let mut full_string = &item[0].split("(")
        sum += first * second;
    }
    return sum.to_string();
}

fn part2(input: &str) -> String {
    let mut enabled = true; 
    let mut total = 0;

    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex = Regex::new(r"don't\(\)").unwrap();

    for part in input.split_inclusive(|c: char| c == ')' || c == ']') {
        if do_regex.is_match(part) {
            enabled = true;
        } else if dont_regex.is_match(part) {
            enabled = false;
        } else if enabled {
            if let Some(captures) = mul_regex.captures(part) {
                let x: i32 = captures[1].parse().unwrap();
                let y: i32 = captures[2].parse().unwrap();
                total += x * y;
            }
        }
    }

    return total.to_string();
}