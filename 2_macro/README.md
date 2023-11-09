## What is macro

Macros look like functions, except that their name ends with bang (i.e., '!').
Macros are expanded into source code that gets compiled with the rest of the program.

## A macro that prints given string

```rust
//main.rc
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
```

Don't use semicolon at the end of the statement in each macro body.

The arguments of a macro are prefixed by a dollar sign (i.e., '$') and type annotated with a designator.
Available designators for the argument of a macro are as follows:
- block
- expr (for expressions; e.g., 1, 1 + 5, true)
- ident (for variable/functino names)
- item
- literal (for literal constants)
- pat (pattern)
- path
- stmt (statement)
- tt (token tree)
- ty (type)
- vis (visibility qualifier)

## Usage
```bash
rustc main.rc
./main
```

## Result
```bash
Print using macro
Print using overloaded macro
```
