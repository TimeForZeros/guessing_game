use rand::Rng; // random and random number generator
use std::cmp::Ordering;
use std::io; //std is standard library, io allows fo i/o
fn main() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1, 101);

  println!("The secret number is: {}", secret_number);
  loop {
    println!("Please input your guess.");

    let mut guess = String::new(); //static method of string type

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("you're a failure at life!");
        continue;
      },
    };

    println!("you guessed: {}", guess);

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}
