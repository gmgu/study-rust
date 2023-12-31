## What is function
Functions get (possibly empty) input arguments and return (possible empty) values.

## A macro that prints given string

```rust
//main.rc
fn add(x: u32, y: u32) -> u32 {  // arguments are type annotated, and the type of return value is specified
  x + y  // the last expression in the function will be used as return value
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
```

Functions are declared using the 'fn' keyword. If the function returns a value, the return type must be specified after an arrow '->'.

## Usage
```bash
rustc main.rc
./main
```

## Result
```bash
Add: 9
Mul: 9
printing x: 9
printing x: 9
```
