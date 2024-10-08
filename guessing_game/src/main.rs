// mod null_check;

use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    // null_check::check_number();
    println!("Guess the number!");

    let secret_number: i32 = rand::thread_rng().gen_range(1..101);

    println!("The secret numner is: {}", secret_number);
    loop {
        println!("Please input your guess.");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to your guess.");

        // 入力が空でないかを確認
        if guess.trim().is_empty() {
            println!("You didn't enter a guess!");
            return; // プログラムを終了する
        }

        let guess: i32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
