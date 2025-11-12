pub mod utils {
    use std::io::{Write, stdin, stdout};
    pub fn convert_string_to_int(s: &str) -> i32 {
        s.trim().parse().unwrap_or(0)
    }

    pub fn read_line(s: &mut String) -> String {
        stdout().flush().unwrap();
        stdin().read_line(s).expect("Error reading user input");
        stdout().flush().unwrap();
        s.trim().to_string()
    }

    pub fn clear() {
        print!("{}[2J", 27 as char);
    }
    // pub fn read_line_as_int() -> i32 {
    //     let mut s = String::new();
    //     let input = read_line(&mut s);
    //     convert_string_to_int(&input)
    // }
}
