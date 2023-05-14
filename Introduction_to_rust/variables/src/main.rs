fn main() {
    // all variables in rust are immutable by default
    // rust performs type inference, so we don't have to declare the type
    let age = 31;
    let name = "Jack";

    // print the variables using println!
    println!("Name = {}, Age = {}", name, age);
    println!("Name = {0}, Age = {1}", name, age);
    println!("Name = {name}, Age = {age}");

    let mut new_age = 25;
    println!("New Age = {}", new_age);

    // declare a float constant
    // note that rust naming convention means that constants are in uppercase
    const PI: f32 = 3.14;

    println!("Value of PI = {}", PI)

}