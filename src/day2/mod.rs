use std::fs;

fn get_input() -> Vec<String> {
    fs::read_to_string("src/day2/input.txt")
        .expect("file not found")
        .split("\n")
        .filter(|inst| !inst.is_empty())
        .map(|inst| inst.trim().to_string())
        .collect()
}

fn part_a() {
    let input = get_input();

    let mut horizontal_pos = 0u32;
    let mut depth = 0u32;

    for i in input {
        let (command, value) = parse_instruction(&i);

        match command {
            "forward" => horizontal_pos = horizontal_pos + value,
            "down" => depth = depth + value,
            "up" => depth = depth - value,
            _ => println!("command doesn't match any case {}", command),
        }
    }

    println!(
        "Day 2::Part A::Horizontal Position::{}, Depth::{}, Total::{}",
        horizontal_pos,
        depth,
        horizontal_pos * depth
    );
}

fn part_b() {
    let input = get_input();

    let mut horizontal_pos = 0u32;
    let mut depth = 0u32;
    let mut aim = 0u32;

    for i in input {
        let (command, value) = parse_instruction(&i);

        println!("{} {}", command, value);

        match command {
            "down" => {
                aim = aim + value;
            }
            "up" => {
                aim = aim - value;
            }
            "forward" => {
                horizontal_pos = horizontal_pos + value;
                depth = depth + (aim * value);
            }
            _ => println!("command doesn't match any case {}", command),
        }
    }

    println!(
        "Day 2::Part B::Horizontal Position::{}, Depth::{}, Total::{}",
        horizontal_pos,
        depth,
        horizontal_pos * depth
    );
}

fn parse_instruction(instruction: &str) -> (&str, u32) {
    let mut split = instruction.split(' ');
    let command: &str = split.next().expect("unable to parse the instruction");
    let value: u32 = split
        .next()
        .expect(&format!("unable to parse the value {:?}", instruction))
        .parse::<u32>()
        .expect(&format!("faild to parse the value {:?}", instruction));

    (command, value)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_input() {
        super::part_a();
        super::part_b();
    }
}
