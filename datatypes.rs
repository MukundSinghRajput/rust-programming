// We use data types in Rust to determine the type of data associated with the variables.

/*let alphabet: char;
error: expected item, found keyword `let`
 --> datatypes.rs:3:1
  |
3 | let alphabet: char;
  | ^^^ consider using `const` or `static` instead of `let` for global variables
  
*/

const ALPHABET: &str = "pro";

fn main() {
    println!("{}", ALPHABET);
}

/*
Data Types in Rust
There are four primary data types in Rust also known as scalar types:

> Integer
> Floating-Point
> Boolean
> Character
*/

// 1. Integer Type
// In Rust, we use integer data types to store whole numbers. For example,

let number: i32 = 200;
/*
Here number is a Signed Integer Type.

Means it can store both +ve and -ve
*/

let num: u32 = 200; // Unsigned Integer Type stores only +ve integer type.
// Here, u32 specifies that the x variable can only store positive values. u specifies unsigned integer type.

// Here, we have created the number variable of type i32 (integer) and stored the value 200.

// The integer type i32 has two parts to it:

// i - specifies signed integer type (can store both positive or negative value)
// 32 - size of the data type (takes 32 bits of space in memory)

/*
Categories of Integer Data Types in Rust

Depending on the size of data, we can further classify the signed and unsigned integer type into various categories:
Size	Signed	Unsigned
8-bit	  i8	    u8
16-bit	  i16	    u16
32-bit	  i32	    u32
64-bit	  i64	    u64
128-bit	  i128	    u128
*/

// For boolean and char datatype 
// make sure to visit --> https://www.programiz.com/rust/data-types

// i32 (default type for integer variable)