fn main() {
    let number = 7;

    // condition must be a bool: if number is not a bool, it will not compile
    // use else if to add more conditions
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // using if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // both arms of the if must return the same type
    println!("The value of number is: {}", number);

    // loops
    // assigning a value as a result of a loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 15 {
            break counter; // this sets let result = counter;
        }
    };

    println!("The result is {}", result);

    // break and continue apply to the innermost loops
    // loops can be named

    let mut count = 0;
    'counting_up: loop {
        println!("counting up");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // break inner loop
            }
            if count == 2 {
                break 'counting_up; // break outer loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    // while
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("a[{}] = {}", index, a[index]);
        index += 1;
    }

    // for
    for element in a {
        println!("the value is: {}", element);
    }
}
