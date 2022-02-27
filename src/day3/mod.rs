use std::fs;

fn get_input() -> Vec<Vec<i8>> {
    fs::read_to_string("src/day3/input.txt")
        .expect("file not found")
        .split("\n")
        .filter(|inst| !inst.is_empty())
        .map(|inst| {
            inst.trim()
                .chars()
                .map(|bit| bit.to_digit(2).expect("Unable to convert to number") as i8)
                .collect::<Vec<i8>>()
        })
        .collect()
}

fn part_a() -> i32 {
    let input = get_input();
    let input_count = input.len();

    let mut ones_in_each_column: Vec<i32> = vec![0; 12];
    let mut mode_in_columns: Vec<i8> = vec![];

    // calculating all the 1s (only 1s) in each column
    for row in input {
        for (index, &value) in row.iter().enumerate() {
            if value > 0 {
                ones_in_each_column[index] += 1;
            }
        }
    }

    let half = (input_count / 2) as i32;

    // if the calculated value is more than the half of the input record count, then there are more
    // 1s than 0s
    for tot in ones_in_each_column {
        mode_in_columns.push(if tot > half { 1 } else { 0 });
    }

    let gamma_rate_base_2: i16 = mode_in_columns
        .into_iter()
        .fold(0, |acc, digit| {
            (acc << 1) + digit as i16
        });

    let gamma_rate = gamma_rate_base_2;
    let epsilon_rate = !gamma_rate & (1i16 << 12i16).wrapping_sub(1);

    gamma_rate as i32 * epsilon_rate as i32
}

#[cfg(test)]
mod test {

    #[test]
    fn test_input() {
        assert!(super::part_a() == 2967914i32);
    }
}
