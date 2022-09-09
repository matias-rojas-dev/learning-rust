fn main() {
    // Numbers
    let number: u32 = 29;
    println!("The number {}", number);

    let age: u8 = 255;
    println!("The age is {}", age);

    // floats
    let number_64 = 4.0; // compiler infers the value to use the default type f64
    let number_32: f32 = 5.0; // type f32 specified via annotation

    println!("The number_64:  {}", number_64);
    println!("The number_32 {}", number_32);

    // Addition, Subtraction, and Multiplication
    println!(
        "1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}",
        1u32 + 2,
        8i32 - 5,
        15 * 3
    );

    // Integer and Floating point division
    println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);

    // chars
    let uppercase_s = 'S';
    let lowercase_f = 'f';
    let smiley_face = '😃';
    println!("{}, {}, {}", uppercase_s, lowercase_f, smiley_face);

    // strings
    // Specify the data type "char"
    let character_1: char = 'S';
    let character_2: char = 'f';

    // Compiler interprets a single item in quotations as the "char" data type
    let smiley_face = '😃';

    // Compiler interprets a series of items in quotations as a "str" data type and creates a "&str" reference
    let string_1 = "miley ";

    // Specify the data type "str" with the reference syntax "&str"
    let string_2: &str = "ace";

    println!(
        "{} is a {}{}{}{}.",
        smiley_face, character_1, string_1, character_2, string_2
    );
}
