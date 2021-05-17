//! A trigonometric calculator for calculating the sine, cosign, and tangent of a triangle, with the vertex opposite the smallest side as a reference point.
//!
//! To do that, first this program asks if you want to input the two legs, or a leg and the hypotenuse of the triangle in question.
//! If the user chose to input two legs, it'll find which one is shorter and take note of that for the next step.
//! Then, the program will use the Pythagorean Theorem to calculate the third size, and order the three sides by: hypotenuse, longer side, shorter side.
//! Finally, the program will output the three trigonometric ratios in reference to the smallest side, by dividing the different sides in the proper combinations.

mod triangle;
mod util;

use triangle::Triangle;
use util::*;

use std::io::Write;

enum InputChoice {
    TwoLegs,
    LegAndHypotenuse,
}

/// This function is given a decision on which sides to read from stdin.
/// It then proceeds to ask for two values from stdin, either two legs or a leg and the hypotenuse.
fn get_lengths(choice: InputChoice) -> Triangle {
    match choice {
        // Later we will calculate which input is bigger.
        InputChoice::LegAndHypotenuse => {
            print_flush!("Length of Hypotenuse (or a leg): ");
            let leg_a = read_f64();
            print_flush!("Length of a Leg (or the hypotenuse if the prior was a leg): ");
            let leg_b = read_f64();
            Triangle::from_hypotenuse(leg_a, leg_b)
        }
        InputChoice::TwoLegs => {
            print_flush!("Length of Leg 1: ");
            let leg_a = read_f64();
            print_flush!("Length of Leg 2: ");
            let leg_b = read_f64();
            Triangle::from_legs(leg_a, leg_b)
        }
    }
}

fn main() {
    print_flush!("Is this `1` two legs or `2` a leg and a hypotenuse: ");
    let triangle: Triangle = match &read_line().as_str()[..1] {
        "1" => get_lengths(InputChoice::TwoLegs),
        "2" => get_lengths(InputChoice::LegAndHypotenuse),
        _ => {
            panic!("Invalid answer for first prompt.");
        }
    };
    todo!();
}
