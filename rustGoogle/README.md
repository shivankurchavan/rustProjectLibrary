```rust
fn main() {
    let x: i32 = 10;
    println!(" x : {x}");
    x = 20;
    println!(" x : {x}");
}

```
Error : 
```
(shiv㉿kali)-[~/…/2024/rust/RustProjectLibrary/rustGoogle]
└─$ cargo run
   Compiling rustGoogle v0.1.0 (/home/shiv/Desktop/2024/rust/RustProjectLibrary/rustGoogle)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x: i32 = 10;
  |         - first assignment to `x`
3 |     println!(" x : {x}");
4 |     x = 20;
  |     ^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut x: i32 = 10;
  |         +++

For more information about this error, try `rustc --explain E0384`.
error: could not compile `rustGoogle` (bin "rustGoogle") due to 1 previous error

```
so we will assign `mut` to the varialbe to specify that it is mutable.


# Built-in Types and Literal Values


|                           | **Types**                                | **Literals**                   |
|---------------------------|-------------------------------------------|---------------------------------|
| Signed integers           | `i8`, `i16`, `i32`, `i64`, `i128`, `isize`| `-10`, `0`, `1_000`, `123_i64` |
| Unsigned integers         | `u8`, `u16`, `u32`, `u64`, `u128`, `usize`| `0`, `123`, `10_u16`           |
| Floating point numbers    | `f32`, `f64`                              | `3.14`, `-10.0e20`, `2_f32`    |
| Unicode scalar values     | `char`                                    | `'a'`, `'α'`, `'∞'`            |
| Booleans                  | `bool`                                    | `true`, `false`                |




### The types have widths as follows:

`iN`, `uN`, and `fN` are N bits wide,

`isize` and `usize` are the width of a pointer,

`char` is 32 bits wide

`bool` is 8 bits wide.

`i` - signed interger 

`u` - unsigned integer 


# Arithmatic operation

sab same
+
-
*
/
^
