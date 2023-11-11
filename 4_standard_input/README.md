## Input and Ouput
We will get input from console and ouput it.

## A program that repeats standard input

```rust
//main.rc
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
```

In this example, we have a lot of information not yet defined.
Most of them will be handled in the later lectures.
In this lecture we concentrate on input/output libraries.

### Flush
Stdout is frequently line-buffered by default. Use io::stdout().flush() to ensure the output is emitted immediately.


### Standard input
Use io::stdin().read_line(&mut variable) to read a string line. Convert the string into any type we want using String.trim().parse().expect().

### Future remarks
We didn't talk about use, let, String::new(), and expect(). They will be explained in the separate lectures in the future.

## Usage
```bash
rustc main.rc
./main
```

## Result
```bash
Input a number: 9
Your number is 9
```


## References
- https://doc.rust-lang.org/std/macro.print.html
- https://stackoverflow.com/questions/39174864/simplification-possible-in-example-using-print-and-flush
