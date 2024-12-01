
fn main() {
    let input: &str = include_str!("./input.txt");
    let output1 = part1(input);
    dbg!(output1);

    let output2 = part2(input);
    dbg!(output2);
}

fn part1(input: &str) -> String {
    let full_input = input.lines();
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in full_input {
        let mut parts = line.split_whitespace();
        if let (Some(left), Some(right)) = (parts.next(), parts.next()) {
            left_list.push(left.parse::<i32>().unwrap());
            right_list.push(right.parse::<i32>().unwrap());
        }
    }

    left_list.sort();
    right_list.sort();

    let result = std::iter::zip(left_list, right_list)
        .map(|pair| pair.0.abs_diff(pair.1))
        .reduce(|acc, n| acc + n)
        .unwrap();

    return result.to_string();
}

fn part2(input: &str) -> String {
    let mut left_list = Vec::new();
    let mut right_list: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        if let (Some(left), Some(right)) = (parts.next(), parts.next()) {
            left_list.push(left.parse::<i32>().unwrap());

            let right_value = right.parse::<i32>().unwrap_or_else(|_| {
                eprintln!("Invalid right value: {}", right);
                0
            });
            *right_list.entry(right_value).or_insert(0) += 1;
        }
    }

    let result = left_list
        .iter()
        .map(|val| {
            val * right_list.get(val).unwrap_or(&0)
        })
        .sum::<i32>();

    return result.to_string();
}
