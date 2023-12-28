fn main() {
    let input = include_str!("Day1Puzzle1.txt");
    let mut sum = 0;
    for line in input.lines() {
        let numbers = get_numbers(line);
        match numbers {
            Some((first, last)) => {
                sum += first*10 + last;
                // println!("Line: {} First: {} Last: {} Sum: {} Running Total: {}", line, first, last, first*10 + last, sum);
            },
            None => println!("No numbers found in the string"),
        }
    }
    println!("Sum: {}", sum);
}

fn get_numbers(line: &str) -> Option<(i32, i32)> {
    let mut first = None;
    let mut last = None;

    for (i, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            let number = c.to_digit(10).unwrap() as i32;
            if first.is_none() {
                first = Some((i, number));
            } 
            last = Some((i, number));
        }
    }

    match (first, last) {
        (Some((_, first)), Some((_, last))) => Some((first, last)),
        _ => None,
    }
}