macro_rules! print_str {
  ($str: literal) => {
    println!($str)
  }; // semicolon between overloads
  ($str1: literal, $str2: literal) => {
    println!("{} {}", $str1, $str2)
  } // semicolon is optional for the last one
}

fn main() {
  print_str!("Print using macro");
  print_str!("Print using", "overloaded macro");
}
