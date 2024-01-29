fn main() {
    println!("Hello, world!");

    another_function(5, 'h');

    // Statements and Expressions
    // Statements are instructions that perform some action and do not return a value
    // Expressions evaluate to a resulting value

    let y = 6; // statement
               // cannot assign a let statement to another variable
               // let x = (let y = 6); // error: expected expression, found statement (`let`)
               // let y = {
               //     let x = 3;
               //     x + 1 // expression, must not end with a semicolon (return value that y binds to)
               // }
    let y = five();
    println!("The value of y is: {}", y);
}

// rust code uses snake case as the conventional style for function and variable names
// function signatures must declare the type of each parameter
fn another_function(x: i32, unit_label: char) {
    println!("The measurement is {x}{unit_label}");
}
// Functions with return values
// return values are not named, but are declared after an arrow (->), these are necessary
fn five() -> i32 {
    5 // this is an expression, so it does not need a semicolon
      // if you add a semicolon, it becomes a statement and will not return a value
}

// statements don’t evaluate to a value, which is expressed by (), the unit type.
// Therefore, nothing is returned, which contradicts the function definition and results in an error.
