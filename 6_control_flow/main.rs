fn main() {
  let a = 10;

  if a < 5 {  // no parentheses
    println!("{a} is smaller than 5");
  }
  else if a > 5 {  // condition must be boolean
    println!("{a} is greater than 5");
  }
  else {
    println!("{a} is equal to 5");
  }

  let b = if a != 5 {5} else {9};
  println!("b is {b}");

  let mut c = 0;
  loop {
    c += 1;
    println!("c = {c}");
    if c > 2 {
      break;
    }
  }

  let d = loop {
    break 9;
  };  // semicolon assigns the value to d
  println!("d = {d}");

  let mut e = 0;
  let mut f = 0;
  'outer_loop: loop {  // one "'"
    e += 1;
    loop {
      f += 1;
      break 'outer_loop  // one "'"
    }
  }
  println!("e and f are {e} and {f}, respectively");

  let mut g = 0;
  while g < 2 {
    g += 1;
  }
  println!("g = {g}");

  let l = [9, 3, 6];
  for item in l {
    println!("item = {item}");
  }

  for i in 1..4 {  // [from, to)
    println!("i = {i}");
  }

  for k in (1..4).rev() {
    println!("k = {k}");
  }
}
