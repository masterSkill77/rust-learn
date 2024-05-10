// use std::io;
// use std::io;
// fn main() {
//     println!("Guess the number!");
//     println!("Please input your guess.");
//     let mut guess = String::new();
//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");
//     println!("You guessed: {}", guess);
// }
// const BAR: &str = "14";

// fn main() {
//     const FOO: char = 'a';
//     // let FOO: &str = "a";
//     /*
//      * There is also const
//      * Double quotes is for string, simple is for char
//      */
//     println!("{} and {}", FOO, BAR);
// }

// fn main() {
// const _VALUE_FOUR_FLOAT: f32 = 4.8;
// Shadowing variable immutable
// Redeclarer le meme variable avec le mot clé let
//Peut changer de type en route

// Les tuples
// let tuples: (i32, i32) = (5000, 5000);

// let (_tuple_0, _tuple_1) = tuples;

// println!(
//     "Value const : {} and value of tuple[0] : {}",
//     VALUE_FOUR_FLOAT, _tuple_0
// );

//     const _DAYS_OF_WEEK: [&str; 7] = [
//         "Monday",
//         "Tuesday",
//         "Wednesday",
//         "Thursday",
//         "Friday",
//         "Saturday",
//         "Sunday",
//     ];

//     let students_notes: [i32; 7] = [10; 7];

//     greetings("Klay");

//     let return_value: i32 = addition(12, 25);

//     println!("{} ", students_notes[0] as f64 / return_value as f64);
// }

// fn greetings(name: &str) {
//     println!("Hello {}", name);
// }

// fn addition(a: i32, b: i32) -> i32 {
//     a + b
// }

use std::ops::Range;

fn main() {
    // let mut guess_age: String = String::new();

    // println!("Quel âge avez-vous ?");

    // io::stdin()
    //     .read_line(&mut guess_age)
    //     .expect("Veuillez remplir l'âge");

    // let age: u64 = guess_age.trim().parse().expect("Not a number");
    // if age > 18 && age <= 60 {
    //     println!("Votre âge est {}, hahaha, vous êtes majeur", age);
    // } else if age > 60 {
    //     println!("Vous avez besoin de prendre votre retraite")
    // } else {
    //     println!("Votre âge est {}, hahaha, va jouer ailleurs", age);
    // }

    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("The result is {}", result);
    let tuples: Range<i8> = 1..5;

    for element in tuples.rev() {
        println!("Countdown {}", element);
    }
}
