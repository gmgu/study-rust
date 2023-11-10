fn add(x: u32, y: u32) -> u32 {  // arguments are type annotated, and the type of return value is specified
  x + y  // the last expression in the functino weill be used as return value
}

fn mul(x: u32, y: u32) -> u32 {
  return x * y;  // we can use return statement
}

fn main() {
  println!("Add: {}", add(4, 5));
  println!("Mul: {}", mul(3, 3));
  plt(9);
  plt2(9);
}

// function can be declared below its call
fn plt(x: u32) {  // function with no return type
  print!("printing x: {}\n", x);
}

fn plt2(x: u32) -> () {  // function with no return type actually returns the unit type '()'
  print!("printing x: {}\n", x);
}
