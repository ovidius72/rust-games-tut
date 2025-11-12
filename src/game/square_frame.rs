use crate::common::utils::{self, convert_string_to_int};
pub fn run() {
    println!("**********************************************");
    println!("************* SQUARE FRAME *******************");
    println!("**********************************************");
    loop {
        let mut input = String::new();
        println!("Enter a number. q return the main menu: ");
        let input = utils::read_line(&mut input);
        if input == "q" {
            break;
        }
        let n = convert_string_to_int(&input);
        for i in 0..n {
            for j in 0..n {
                if j == 0 || j == n - 1 {
                    if i == 0 || i == n - 1 {
                        print!(" + ")
                    } else {
                        print!(" | ")
                    }
                } else {
                    print!(" - ")
                }
            }
            println!("\n");
        }
    }
}
