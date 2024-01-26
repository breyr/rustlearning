fn main() {
    // shadowing

    let x = 5; // immutable by default, use mut to make it mutable

    // by using let we can perform transformations on an immutable variable
    let x = x + 1;
    {
        let x = x * 3;
        println!("The value of x in the inner scope is: {}", x); // 18
    }
    println!("The value of x is: {x}"); // 6

    // let can also be used to change the type of a variable
    // _ in front of an unused variable name makes it invisible to the compiler
    let _spaces = "    ";
    let _spaces = _spaces.len(); // spaces is now an integer
                                 // if you used mut spaces, it would result in a compile time error
                                 // you can't mutate a variable's type

    // contansts are always immutable, must be type annotate, and set to a constant expression
    // constants are declared with the const keyword
    // cannot be set to the result of a function call or any other value that could only be computed at runtime
    // certain operations can be used when setting constants at compile time

    const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Variable Types

    // Integer
    /*
        Length  Signed  Unsigned
        8-bit   i8      u8
        16-bit  i16     u16
        32-bit  i32     u32
        64-bit  i64     u64
        128-bit i128    u128
        arch    isize   usize (dependent on the kind of computer your program is running on)

        Integer Overflow - rust will panic at runtime if a number exceeds the max value
    */

    // Floating-Point
    // f32 and f64
    // f64 is the default because it is roughly the same speed as f32 but more precise

    // Numeric Operations
    // addition
    // subtraction
    // multiplication
    // division (integer)
    // remainder (integer)

    // Boolean
    // bool
    // true or false

    // Character
    // char
    // single quotes

    // Compound Types

    // tuples
    // fixed length
    let _tup: (i32, f64, char) = (500, 6.4, 'c');
    // pattern matching to destructure a tuple
    let (_x, _y, _z) = _tup;
    // access a tuple element directly by using a period (.)
    let _five_hundred = _tup.0;
    // tuples without any values are called unit types

    // arrays
    // fixed length
    // elements must be the of the same type
    // useful when you want your data allocated on the stack rather than the heap
    let _a = [1, 2, 3, 4, 5];
    // arrays are useful when you want to ensure you always have a fixed number of elements
    let _a: [i32; 5] = [1, 2, 3, 4, 5]; // speficy the type, followed by the length
                                        // initialize an array with the same value for each element
    let _a = [3; 5]; // [3, 3, 3, 3, 3]
    let first = _a[0];

    // vectors
    // can grow or shrink in size
}
