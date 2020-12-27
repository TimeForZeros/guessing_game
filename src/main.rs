use std::io; //std is standard library, io allows fo i/o
fn main() {
  println!("Guess the number!");
  println!("Please input your gess.alloc");

  let mut guess = String::new(); //static method of string type

  io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    println!("you guessed: {}", guess);
}
