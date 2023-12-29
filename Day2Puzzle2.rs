fn main() {
    let input = include_str!("Day2Puzzle1.txt");
    let mut sum = 0;
    for line in input.lines() {
/*
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
 */
        let mut red_min = 0;
        let mut green_min = 0;
        let mut blue_min = 0;
        for split in line.split(": ") {
            if split.starts_with("Game ") {
                continue;
            } else {
                for draw_split in split.split("; ") {
                    let mut red_count = 0;
                    let mut green_count = 0;
                    let mut blue_count = 0;
                    for draw in draw_split.split(", ") {
                        if draw.contains("red") {
                            match draw[0..draw.len()-4].parse() {
                                Ok(count) => red_count = count,
                                Err(_) => {
                                    println!("Error parsing red count");
                                }
                            }
                        }
                        if draw.contains("green") {
                            match draw[0..draw.len()-6].parse() {
                                Ok(count) => green_count = count,
                                Err(_) => {
                                    println!("Error parsing green count");
                                }
                            }
                        }
                        if draw.contains("blue") {
                            match draw[0..draw.len()-5].parse() {
                                Ok(count) => blue_count = count,
                                Err(_) => {
                                    println!("Error parsing blue count");
                                }
                            }
                        }
                    }
                    if red_min < red_count {
                        red_min = red_count;
                    }
                    if blue_min < blue_count {
                        blue_min = blue_count;
                    }
                    if green_min < green_count {
                        green_min = green_count;
                    }
                }
            } 
        }
        sum+= red_min * blue_min * green_min;
    }
    println!("Sum: {}", sum);
}
