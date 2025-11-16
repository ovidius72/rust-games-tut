mod common;
pub mod game;

use game::{spin_row, square_frame};

use crate::{
    common::utils::{clear, read_line},
    game::christmas_tree,
};

#[allow(dead_code)]
fn play_with_let() {
    fn get_a_result() -> Result<bool, String> {
        let rnd = rand::random_range(0..100);
        println!("**** GET_RESULT: rnd num is: {rnd}");
        if rnd < 50 {
            return Ok(true);
        }
        Err("GET_RESULT: Error generating number".to_string())
    }

    fn get_an_option() -> Option<bool> {
        let rnd = rand::random_range(0..100);
        println!("GET_OPTION: **** rnd num is: {rnd}");
        if rnd < 50 {
            return Some(true);
        }
        None
    }
    // ***********************
    // LET - ELSE
    // ***********************
    // check with option.
    // let Some(b) = get_an_option() else {
    //     println!("Option KO");
    //     panic!("Option KO");
    //     // panic!("Error reading number")
    // };
    // println!("Option ok: received: {b}");

    // // u64::from_str_radix("22", 0);
    // // check with result.
    // let Ok(o) = get_a_result() else {
    //     println!("Result KO");
    //     panic!("Result KO");
    // };
    // println!("Result OK received {o}");

    // ***********************
    // IF - LET
    // ***********************
    let res1 = if let Some(x) = get_an_option() {
        println!("Ok in let-if with result {x}");
        x
    } else {
        println!("Option KO in let-if.");
        false
    };

    let res2 = if let Ok(o) = get_a_result() {
        println!("OK from result with {o}");
        o
    } else {
        println!("Result KO in let-if.");
        false
    };
    let res3 = get_an_option().unwrap_or_else(|| {
        println!("Error unwrapping Option returning a default value");
        false
    });
    let res4 = get_a_result().unwrap_or_else(|_| {
        println!("Error unwrapping result. returning a default value");
        false
    });

    println!("************************************************");
    println!("Option Result: {res1} \nResult result: {res2}");
    println!("************************************************");
    println!("Option Result: {res3} \nResult result: {res4}");
    println!("************************************************");

    println!("End of the program");
}

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
