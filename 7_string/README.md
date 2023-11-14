## String
We study string before learning ownership.

## Examples

```rust
//main.rc
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
```

The length of string literals can be determined in the compile time, so string literals are stored in the stack.

A variable-length strings should be stored in the heap. Strings can be allocated using String::from method.

## Usage
```bash
rustc main.rc
./main

```

## Result
```bash
hello world!
hello
hello rust
hello rust!
length of str = 11
bytes h = 104
bytes e = 101
```
