use aoc_runner_derive::aoc;

fn split_lines(input: &str) -> (Vec<&str>, &str) {
    let line_count = input.lines().count();
    let line_length = input.len() / line_count + 1; // include the newline character

    let number_lines: Vec<&str> = (0..(line_count - 1))
        .map(|i| &input[(i * line_length)..((i + 1) * line_length - 1)])
        .collect();
    let operator_line: &str = &input[(line_count - 1) * line_length..];

    (number_lines, operator_line)
}

#[aoc(day6, part1)]
fn horizontal_parse(input: &str) -> u64 {
    let (number_lines, operator_line) = split_lines(input);

    // Hold mutable iterators that parse the number lines.
    let mut numberators: Vec<_> = number_lines
        .iter()
        .map(|nums| nums.split_whitespace().map(|n| n.parse::<u64>().unwrap()))
        .collect();

    let mut result = 0;
    for operator in operator_line.split_whitespace() {
        // Advance each number line iterator for one step per operator.
        // This is safe as long as the lines match in length.
        let numbers = numberators.iter_mut().map(|i| i.next().unwrap()).collect();

        result += apply_operator(numbers, operator);
    }
    result
}

fn apply_operator(numbers: Vec<u64>, operator: &str) -> u64 {
    match operator {
        "+" => numbers.iter().sum::<u64>(),
        "*" => numbers.iter().product::<u64>(),
        _ => panic!("Unknown operator"),
    }
}

#[aoc(day6, part2)]
fn vertical_parse(input: &str) -> u64 {
    let (number_lines, operator_line) = split_lines(input);

    let mut result = 0;

    let mut operatorator = operator_line.split_ascii_whitespace();
    let mut start_index = 0;
    loop {
        let end_index = next_space_index(&number_lines, start_index);
        let numbers = parse_vertical_numbers(&number_lines, start_index, end_index);
        result += apply_operator(numbers, operatorator.next().unwrap());

        start_index = end_index + 1;
        if end_index >= number_lines[0].len() {
            break;
        }
    }
    result
}

fn parse_vertical_numbers(number_lines: &Vec<&str>, start: usize, end: usize) -> Vec<u64> {
    let mut numbers = Vec::new();
    for column in start..end {
        let mut number = 0;
        for row in number_lines {
            let char = row.as_bytes()[column] as char;
            if char.is_ascii_digit() {
                number = number * 10 + (char as u8 - 48) as u64;
            }
        }
        numbers.push(number);
    }
    numbers
}

/// Finds the next index where all strings have a space,
/// or the end of the string if no space is found.
fn next_space_index(number_strings: &Vec<&str>, start_index: usize) -> usize {
    let mut max_index = 0;
    for s in number_strings.iter().map(|s| &s[start_index..]) {
        match s.find(' ') {
            Some(index) => max_index = max_index.max(index),
            None => return number_strings[0].len(),
        }
    }
    start_index + max_index
}
