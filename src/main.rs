mod triangle;
use triangle::Triangle;
use std::{io::Write, str::FromStr};

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

fn read_f64() -> f64 {
    f64::from_str(read_line().trim()).expect("Not a Number.")
}

enum InputChoice {
    TwoLegs,
    LegAndHypotenuse,
}

fn get_lengths(choice: InputChoice) -> Triangle {
    match choice {
        InputChoice::TwoLegs => {
            print!("Length of Leg 1: ");
            std::io::stdout().flush().unwrap();
            let leg_a = read_f64();
            print!("Length of Leg 2: ");
            std::io::stdout().flush().unwrap();
            let leg_b = read_f64();
            Triangle::from_legs(leg_a, leg_b)
        },
        InputChoice::LegAndHypotenuse => {
            print!("Length of Leg: ");
            std::io::stdout().flush().unwrap();
            let leg_a = read_f64();
            print!("Length of Hypotenuse: ");
            std::io::stdout().flush().unwrap();
            let leg_b = read_f64();
            Triangle::from_hypotenuse(leg_a, leg_b)
        },
    }
}

fn main() {
    println!("Is this `1` two legs or `2` a leg and a hypotenuse");
    let triangle: Triangle = match &read_line().as_str()[..1] {
        "1" => {
            get_lengths(InputChoice::TwoLegs)
        },
        "2" => {
            get_lengths(InputChoice::LegAndHypotenuse)
        },
        _ => {
            panic!("Invalid answer for first prompt.");
        }
    };
}
