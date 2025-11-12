use crate::common::utils::{convert_string_to_int, read_line};

pub fn run() {
    loop {
        print!("Enter a number from 1 to 100 (or q to exit): ");
        let mut input = String::new();
        let input = read_line(&mut input);
        if input == "q" {
            break;
        }
        let n = convert_string_to_int(&input);
        if !(0..=100).contains(&n) {
            println!("The entered number is not valid.");
            continue;
        }

        let tot_row = n * 2;
        for col in 0..=(n) {
            for row in 0..=tot_row {
                let half = tot_row / 2;
                let mut symbol = if row == half { "|" } else { " " };
                let right_offset = half + col + 1;
                let left_offset = half - col;
                if (left_offset..half).contains(&row) || (half + 1..right_offset).contains(&row) {
                    symbol = "*"
                }
                print!("{}", symbol);
            }
            println!()
        }
    }
}
