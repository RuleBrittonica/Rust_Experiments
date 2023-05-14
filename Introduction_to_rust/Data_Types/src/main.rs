fn main() {
    // Integer type - i signifies signed integer
    let number: i32 = 200;
    println!("i32 = {}", number);

    // unsigned integer - u signifies unsigned integer
    let unsigned_number: u32 = 400;
    println!("u32 = {}", unsigned_number);

    // options for number size - works for both signed and unsigned integers
    // number = bit allocated
    let number_8bit: i8 = 127;
    let number_16bit: i16 = 32767;
    let number_32bit: i32 = 2147483647;
    let number_64bit: i64 = 9223372036854775807;
    let number_128bit: i128 = 170141183460469231731687303715884105727;
    println!("i8 = {}", number_8bit);
    println!("i16 = {}", number_16bit);
    println!("i32 = {}", number_32bit);
    println!("i64 = {}", number_64bit);

    // floating point numbers
    let float_number: f32 = 3.14;
    let large_float_number: f64 = 3.141592653589793238462643383;
    println!("f32 = 3.14");
    println!("f64 = 3.141592653589793238462643383");

    // boolean type
    let flag1: bool = false;
    let flag2: bool = true;
    println!("flag1 = {}", flag1);
    println!("flag2 = {}", flag2);

    // character type
    let character: char = 'a';
    println!("character = {}", character);

    // rust type casting
    let decimal: f64 = 65.4321;
    let integer: u8 = decimal as u8;
    println!("decimal = {}", decimal);
    println!("integer = {}", integer);

    let character_2: char = 'z';
    let integer_2: u8 = character_2 as u8;
    println!("character_2 = {}", character_2);
}
