## Variables
Some remarkable characteristics of the variables in Rust is as follows:
- Variables are immutable by default
- To make a variable mutable, use 'mut' in front of the variable's name
- Both immutable and mutable variables can be redeclared
- Constant variables need type and value when declared
- Constant variables cannot be redeclared
- The name of constant variables should be upper cased and words are connected by underscores.
- The value of a constant variable must be computable in compile time.

## Examples

```rust
//main.rc
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
```

## Usage
```bash
rustc main.rc
./main

```
Ignore the following three warnings.
- warning: unused variable: `b`
- warning: unused variable: `c`
- warning: variable does not need to be mutable

## Result
```bash
X and Y are 5 and 3, respectively
X and Y are 5 and 4, respectively
Z is 9
The first a = 1
The second a = 9
The third a = 3
The second a after the scope of the third a = 9
```
