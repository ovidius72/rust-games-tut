use crate::common::utils;
use rand::seq::IndexedRandom;

fn greet() {
    println!("Thank you for playing. Bye 👋");
}

fn print_row(row: &Vec<&str>) {
    for i in 0..row.len() {
        print!("{:?} | ", row.get(i).unwrap())
    }
    println!("\n")
}

fn check_row(row: &Vec<&str>) -> i32 {
    if row.len() != 3 {
        panic!("Invalid row");
    }
    let item1 = row.first().unwrap();
    let item2 = row.get(1).unwrap();
    let item3 = row.get(2).unwrap();
    let mut won_amount = 0;
    if item1 == item2 && item2 == item3 {
        println!("You win");
        if item1 == &"🍒" {
            won_amount = 10;
        }
        if item1 == &"🍌" {
            won_amount = 20;
        }
        if item1 == &"🌟" {
            won_amount = 30;
        }
        if item1 == &"🍉" {
            won_amount = 40;
        }
        if item1 == &"💎" {
            won_amount = 100;
        }
    }
    won_amount
}

fn spin_row() -> Vec<String> {
    let symbols = ["🍒", "🍌", "🌟", "🍉", "💎"];
    let mut extracted = vec![];
    for _ in 0..3 {
        match symbols.choose(&mut rand::rng()) {
            Some(i) => extracted.push(i.trim().to_string()),
            None => println!("Nothing"),
        }
    }
    extracted
}

pub fn run() {
    println!("***********************************************************");
    println!("****Welcome: Symbols 🍒 🍌 ⭐️ 🍉 💎 🍋 🍓 ****************");
    println!("***********************************************************");

    let mut amount = 100;

    loop {
        let mut input = String::new();
        println!("Your amount is: {}", amount);
        if amount == 0 {
            println!("You don't have enough money to play. Bye");
            println!("Enter r to restart.");
            input = utils::read_line(&mut input);
            if input == "r" {
                amount = 100;
                continue;
            }

            greet();
            break;
        }

        print!("Enter you bet or (q) to quit: ");
        input = utils::read_line(&mut input);

        if input == "q" {
            greet();
            break;
        } else {
            let bet = utils::convert_string_to_int(&input);

            if bet > amount {
                println!("Your bet is too high!");
                continue;
            }
            if bet <= 0 {
                println!("Your bet is not valid");
                continue;
            }
            let row = spin_row();
            let row_as_vec_of_str = row.iter().map(|s| &s[..]).collect();

            print_row(&row_as_vec_of_str);
            amount -= bet;
            let won_amount = check_row(&row_as_vec_of_str);
            amount += won_amount;
        }
    }
}
