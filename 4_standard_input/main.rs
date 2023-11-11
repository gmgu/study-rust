use std::io;
use std::io::Write;
// identically, we can "use std::io::{self, Write}

fn main() {
  let mut line = String::new();  // use mut for mutable variable. (variables are immutable by default.)

  print!("Input a number: ");
  io::stdout().flush().expect("Unexpected flush error");

  io::stdin().read_line(&mut line).expect("Failed to read a line");
  let number: i32 = line.trim().parse().expect("Input is not a number");  // type casting
  println!("Your number is {number}"); 
}
