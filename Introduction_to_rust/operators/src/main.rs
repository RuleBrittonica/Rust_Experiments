fn main() {
    let a = 20;
    let b = 2;

    // add two variables using + operator
    let x = a + b;
    println!("{} + {} = {}", a, b, x);

    // subtract two variables using - operator
    let y = a - b;
    println!("{} - {} = {}", a, b, y);

    // multiply two variables using * operator
    let z = a * b;
    println!("{} * {} = {}", a, b, z);

    let dividend = 21;
    let divisor = 8;

    // arithmetic division using / operator with integers
    let division = dividend / divisor;

    println!("{} / {} = {}", dividend, divisor, division);

    // to get actual floating point values, we need to use floats - we could
    // explicitly type this, or we can just let the compiler infer the type

    let dividend_float = 21.0;
    let divisor_float = 8.0;

    // arithmetic division using / operator with floats
    let division_float = dividend_float / divisor_float;
    println!("{} / {} = {}", dividend_float, divisor_float, division_float);

    let dividend_modulus = 21;
    let divisor_modulus = 8;

    // arithmetic remainder using % operator
    let remainder = dividend_modulus % divisor_modulus;

    println!("{} % {} = {}", dividend_modulus, divisor_modulus, remainder);

    let mut x = 1;

    // compound assignment operators
    x += 3;
    println!("x = {}", x);

    let x = 7;
    let y = 3;

    // use of comparison operators
    let c = x > y;
    let d = x < y;
    let e = x == y;

    println!("{} >= {} is {}", x, y, c);
    println!("{} <= {} is {}", x, y, d);
    println!("{} == {} is {}", x, y, e);

    let a = true;
    let b = false;

    // logical AND operation
    let c = a && b;

    // logical OR operation
    let d = a || b;

    // logical NOT operation
    let e = !a;

    println!("{} && {} = {}", a, b, c);
    println!("{} || {} = {}", a, b, d);
    println!("!{} = {}", a, e);
}
