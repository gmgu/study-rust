fn main() {
  let s = "hello world!";  // string literals are immutable and stored in stack
  println!("{s}");

  let mut str = String::from("hello");  // Strings can be mutable and stored in heap
  println!("{str}");
  str.push_str(" rust");  // push a string
  println!("{str}");
  str.push('!');  // push a character
  println!("{str}");

  println!("length of str = {}", str.len());

  println!("bytes {} = {}", str.chars().nth(0).unwrap(), str.as_bytes()[0]);
  println!("bytes {} = {}", str.chars().nth(1).expect("Index out of bound"), str.as_bytes()[1]);
}
