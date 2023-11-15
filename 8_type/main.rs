fn main() {
  let a1: i8 = -9;    // signed 8-bit integer
  let a2: u8 = 9;     // unsigned 8-bit integer
  let a3: i16 = -90;  // signed 16-bit integer
  let a4: u16 = 90;   // unsigned 16-bit integer
  let a5: i32 = -900; // signed 32-bit integer
  let a6: u32 = 900;  // unsigned 32-bit integer
  let a7: i64 = -9000; // signed 64-bit integer
  let a8: u64 = 9000; // unsigned 64-bit integer
  let a9: i64 = -90000; // signed 128-bit integer
  let a10: u64 = 90000; // unsigned 128-bit integer
  let a11: isize = -9;  // signed, size depends on the architecture of the computer your program is running on
  let a12: usize = 9;  // unsigned, size depends on the architecture of the computer your program is running on


  let b1: i32 = 1_000;  // 1000, decimal
  let b2: i32 = 0xff;   // hex
  let b3: i32 = 0o77;   // octal
  let b4: i32 = 0b1111; // binary
  let b5: u8 = b'A';    // Byte (u8 only)
  let b6 = 1_000i32;    // type suffix

  let c1: f32 = 3.14;     // 32 bits
  let c2: f64 = 3.141592; // 64 bits
  let c3 = 9.0;           // f64 default

  let d1 = true;          //booleans are one byte in size
  let d2: bool = false;

  let e1 = 'n';           // single quotes
  let e2: char = 'i';     // four bytes in size and represents a Unicode Scaler Value

  let f1 = (-9, 9.0);
  let f2: (i32, f64) = (-9, 9.0);
  let (x, y) = f1;
  let z = f2.0; // -9
  let w = f2.1; // 9.0

  let g1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
  let g2: [i32; 5] = [1, 2, 3, 4, 5];
  let g3 = [3; 5];  // = [3, 3, 3, 3, 3]. [initial value; size]
  let g4 = g3[1];  // 1-th value of g3
}
