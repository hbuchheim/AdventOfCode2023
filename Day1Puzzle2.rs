use std::iter::successors;

fn main() {
    let input = include_str!("Day1Puzzle1.txt");
    let mut sum = 0;
    for line in input.lines() {
        let numbers = get_numbers(line);
        match numbers {
            Some((first, last)) => {
                sum += first*10 + last;
            },
            None => println!("No numbers found in the string"),
        }
    }
    println!("Sum: {}", sum);
}

fn get_numbers(line: &str) -> Option<(i32, i32)> {
    let mut first = None;
    let mut last = None;
    let converted_line = convert_text_numbers(line);

    for (i, c) in converted_line.chars().enumerate() {
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

fn convert_text_numbers(line: &str) -> String {
    let mut result = String::new();
    for i in 0..=line.len()-1 {
        let c = line.chars().nth(i);
        match c {
            Some(char) => {
                if char.is_digit(10) {
                    result.push(char);
                } else {
                    for number in 1..=20 {
                        let number_text = encode(number);
                        if line[i..].starts_with(&number_text) {
                            result.push_str(&number.to_string());
                            break;
                        }
                    }
                }
            },
            None => {
                // Character is not valid (index out of bounds)
                println!("Index {} is out of bounds for the string", i);
            }
        }
    }
    result
}

fn encode(num: u64) -> String {
    let ones: [&str; 20] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen",
    ];
    let tens: [&str; 10] = [
        "zero", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    match num {
        0..=19 => ones[num as usize].to_string(),
        20..=99 => {
            let upper = (num / 10) as usize;
            let lower = num % 10;
            format!("{}-{}", tens[upper], encode(lower))
        },
        100..=999 => format!("{} hundred {}", encode(num / 100), encode(num % 100)),
        _ => {
            let (div, order) = successors(Some(1u64), |v| v.checked_mul(1000))
                .zip(ORDERS.iter())
                .find(|&(e, _)| e > num / 1000)
                .unwrap();
            format!("{} {} {}", encode(num / (div * 1000)), order, encode(num % (div * 1000)))
        }
    }
}

const ORDERS: [&str; 3] = ["thousand", "million", "billion"];