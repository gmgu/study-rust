fn main() {
  let s = String::from("hello");  // immutable variable
  print_func(&s); // create reference and pass the reference
  println!("{}", s);  // 

  let r1 = &s;
  let r2 = &s;  // multiple immutable reference possible
  println!("{} {}", r1, r2);

  let mut s = String::from("hi");   // mutable variable
  change(&mut s); // create mutable reference
  println!("{}", s);  // the data of s changed
  print_func(&s); // create immutable reference for mutable variable

  //let r3 = &mut s;
  //let r4 = &mut s;
  //println!("{} {}", r3, r4); // cannot use multiple mutable reference at the same time

  //let r3 = &s;
  //let r4 = &mut s;
  //println!("{} {}", r3, r4);  // cannot use mutable and immutable references at the same time

  let r3 = &s;
  println!("{}", r3);
  let r4 = &mut s;
  println!("{}", r4);   // okay to use mutable reference after using immutable reference
}

fn print_func(a: &String) -> () { // reference does not own the data
  println!("{}", a);  // use reference just like the original variable
} // the reference (not the data its pointing) is dropped

fn change(a: &mut String) -> () { // mutable reference
  a.push_str(" world!");
}
