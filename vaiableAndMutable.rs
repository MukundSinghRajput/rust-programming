/*
By default, Rust variables are immutable, which means we cannot change 
the value of a variable once it is defined. 

To solve this problem, Rust allows us to create mutable variables.
*/

fn main() {
    /*
    We use the mut keyword before the variable
    name to create a mutable variable. 

    Mutable variable --> a variable whose value can be changed 
    */
    let mut x = 10;
    println!("{x}");
    x = 20;
    println!("{x}");
    constant();
}

/*
Rules for Naming Variables in Rust
We can use any names as variable names, however, there are some rules we should follow:

1. Rust is a case sensitive language. Hence, lowercase variables and uppercase variables are different. For example,

age is different from AGE

name is different from Name

2. Variables must start with either a letter or an underscore. For example,

let age = 31;     	// valid and good practice
let _age = 31;    	// valid variable 
let 1age = 31;    // inavlid variable

3. Variable names can only contain letters, digits and an underscore character. For example,

let age1 = 31;        // valid variable
let age_num = 31;     // valid variable
let s@lary = 52352;   // invalid variable

4. Use underscore if we need to use two words as variable names. For example,

let first name = "Jack";    // invalid variable
let first_name = "Jack";    // valid variable
let first-name = "Jack";    // invalid variable

Note: Always try to give meaningful names to your variables. For example, name, age, number are better names than n, ag, nm.
*/

/*
Rust Constants
A constant is a special type of variable whose value cannot be changed. We use the const keyword to create constants in Rust.
*/

fn constant() {
    /*
    Note: As per Rust's naming convention, we use uppercase for the name of constants.
    */
    const NUM: i32 = 34; // i --> stands for integer
    println!("{}", NUM);
}