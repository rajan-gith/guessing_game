use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){

  println!("Guessing number game");
  println!("Guess the number!");
  
  let secret_number = rand::thread_rng()
  .gen_range(1..=100);

  println!("The secret no is: {secret_number}");

  loop {

    println!("Please input your guess.");

    let mut guess = String::new();
  
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");
  
    let guess: u32 = match guess.trim()
                          .parse() {
                            Ok(num) => num,
                            Err(_) => continue,
                          };
  
    println!("You guessed: {guess}");
  
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Lower number"),
      Ordering::Greater => println!("Higher number"),
      Ordering::Equal =>{
        println!("Hurray");
        break;
      }
    };
  }
}