fn main() {
  let x = 5;  // variables are immutable by default
  //x = 6;  // you cannot change the value of x
  let mut y = 3;  // add 'mut' in front of the variable name to make it mutable
  println!("X and Y are {} and {}, respectively", x, y);

  y = 4;  // you can change the value of y
  println!("X and Y are {} and {}, respectively", x, y);

  println!("Z is {Z}");

  let a = 1;
  println!("The first a = {a}");

  // shadowing
  let a = a + 8;
  println!("The second a = {a}");

  {
    let a = 3;
    println!("The third a = {a}");
  }

  println!("The second a after the scope of the third a = {a}");


  // shadowing vs mutable (this is for demonstration; ignore the unused variable warning)
  let b = "   ";
  let b = b.len();

  let mut c = "   ";
  //c = c.len();  // error
}


const Z: i32 = 9;  // the name of constants should be upper cased

//const Z: i32 = 8;  // error
