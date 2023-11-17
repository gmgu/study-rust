## Ownership
Each value in Rust has an owner. There can only be one owner at a time. When the owner goes out of scope, the value will be dropped.

## Examples

```rust
//main.rc
fn main() {
  {
    let s = "hello";  // s is valid from here
  }  // s is no longer valid
  //println!("{}", s);  // s is not valid here

  let x = 5;  // x is stored in the stack
  let y = x;  // y is stored in the stack
  println!("x: {}, y: {}", x, y);  // no ownership changes

  let a = String::from("hello");  // a is pointer. data is stored in the heap
  println!("a: {}", a);
  let b = a;  // b is a pointer. ownership of the data is passed to a from b; a is no more valid
  println!("b: {}", b);
  // println!("a: {}", a); // a is not valid

  let a = String::from("world");
  let b = a.clone();  // copied
  println!("a: {}, b: {}", a, b);

  take_ownership(a);
  //println!("a: {}", a);  // a is not valid

  let mut a = give_ownership();
  println!("a: {}", a);

  a = take_and_return_ownership(a);
  println!("a: {}", a);
}

fn take_ownership(a: String) -> (){
}

fn give_ownership() -> String {
  let a = String::from("hi");
  return a;
}

fn take_and_return_ownership(mut a: String) -> String {
  a.push_str(" there");
  return a;
}
```

## Usage
```bash
rustc main.rc
./main
```

Ignore "unused variable" warnings.

## Result
```bash
x: 5, y: 5
a: hello
b: hello
a: world, b: world
a: hi
a: hi there
```
