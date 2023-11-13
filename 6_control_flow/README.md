## Control Flow
We will learn how to use if, while, and for statments.

## Examples

```rust
//main.rc
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
```

### if
No parentheses for the condition. Must use brackets. No implicit type conversion from numbers to boolean.

### loop
Infinite loop. Can be labeled, and can break the labeled loop in the nested loop.

### while
No parentheses for the condition. Must use brackets.

### for
Use from..to for representing [from, to).

## Usage
```bash
rustc main.rc
./main

```

## Result
```bash
10 is greater than 5
b is 5
c = 1
c = 2
c = 3
d = 9
e and f are 1 and 1, respectively
g = 2
item = 9
item = 3
item = 6
i = 1
i = 2
i = 3
k = 3
k = 2
k = 1

```
