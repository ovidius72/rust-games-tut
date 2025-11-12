mod common;
pub mod game;
use game::{spin_row, square_frame};

use crate::{
    common::utils::{clear, read_line},
    game::christmas_tree,
};

fn main() {
    clear();
    println!("*********************************************");
    println!("**** Welcome to the Antonio's game board ****");
    println!("*********************************************");
    println!();
    loop {
        println!("1 - Spin Row\t\t 2 - Draw Rectangles");
        println!("3 - Draw Christmas Tree");
        print!("Scegli l'App da avviare [1, 2, 3]. enter q to exit: ");
        let mut input = String::new();
        let choice = read_line(&mut input);
        match choice.trim() {
            "1" => {
                println!("**** Running Spin Row");
                clear();
                spin_row::run();
                clear();
            }
            "2" => {
                println!("**** Running Square Draw");
                clear();
                square_frame::run();
                clear();
            }
            "3" => {
                println!("**** Running Christmas Tree Drawer");
                clear();
                christmas_tree::run();
                clear();
            }
            "q" => {
                println!("Thank you for playing 👋");
                break;
            }
            _ => {
                clear();
                println!("No valid choice");
                continue;
            }
        }
    }
}
