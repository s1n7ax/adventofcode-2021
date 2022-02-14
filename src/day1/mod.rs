use std::fs;

fn get_input() -> String {
    fs::read_to_string("src/day1/input.txt").expect("file not found")
}

pub fn part_a() {
    let window_sum_list: Vec<u32> = get_input()
        .trim()
        .split('\n')
        .map(|depth| depth.parse::<u32>().unwrap())
        .collect();

    let mut increase_count: i32 = 0;

    for i in 1..window_sum_list.len() {
        if window_sum_list[i - 1] < window_sum_list[i] {
            increase_count = increase_count + 1;
        }
    }

    println!("Day 1::Part A::{}", increase_count)
}

pub fn part_b() {
    let window_sum_list: Vec<u32> = get_input()
        .trim()
        .split('\n')
        .map(|depth| depth.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .windows(3)
        .map(|window| window.iter().sum())
        .collect();

    let mut increase_count: i32 = 0;

    for idx in 1..window_sum_list.len() {
        if window_sum_list[idx - 1] < window_sum_list[idx] {
            increase_count = increase_count + 1;
        }
    }

    println!("Day 1::Part B::{}", increase_count)
}

#[cfg(test)]
mod test {

    use super::{part_a, part_b};

    #[test]
    fn test() {
        part_a();
        part_b();
    }
}
