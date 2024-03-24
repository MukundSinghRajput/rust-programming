fn main() {
    let x = 10; // declare a variable 
    // print x = 10
    println!("x = {}", x);
    extra();
    bio();
    bio2();
    bio3()
}

fn extra() {
    /*
    Define x = 20 then print x 
    */
    let x = 20;
    println!("x = {}", x)
    /*
    Here, {} is a placeholder which is replaced by the value of the 
    variable after the comma. That's why we get 20 as output instead of {} 
    */
}

fn bio() {
    let name = "Mukund";
    let age = 16;
    println!("Name = {} \nAge = {}", name, age)
    /*
    However, we can also specify the numbering for placeholders to print variables in different order. 
    Let's see another example with new function bio2*/
}

fn bio2() {
    let name = "Mukund";
    let age = 16;
    println!("Age = {1}\nName = {0}", name, age)
    /*
    We can also use the variable names directly inside the placeholder.
    */
}

fn bio3() {
    let name = "Mukund";
    let age = 16;
    println!("Age = {age}\nName = {name}");
}

//Note: In the Rust ecosystem, line comments are preferred over block comments.
//Comments are also useful for temporarily disabling chunks of code.