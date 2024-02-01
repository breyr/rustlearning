// enums are used to define different "possibilities" encoded in a variable
// we can enumerate all possible variants, which is where enumeration gets its name

enum IpAddrKind {
    V4,
    V6,
}

// can have values for enum variants
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// can add methods to enums
impl IpAddrKind {
    fn who_am_i(&self) {
        // method body
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

#[derive(Debug)]
enum State {
    Pennsylvania,
}

// => separates the pattern and the code to run
// when a match runs, it will compare the value to each arm in order
// if you want multiple lines for one arm, you need {}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // state gets bound to value of Coin::Quarter(State::Pennsylvania)
            println!("State quarter from {:?}!", state);
            25
        }
    }
    // matches are exhaustive, meaning the arms' patterns must cover all possibilities
    // catch all -> _

    // only executing code if Some is met
    // if let Coin::Quarter(state) = coin {
    //     println!("State quarter from {:?}!", state);
    // } else {
    //     // do something
    // }
}

// Option
// Requesting the first item in a non-empty list you get the valued,
// Requesting the first item in an empty list, you get nothing
// Expressing this in terms of the type system, the compiler can check to make sure that you've covered all cases
// Rust doesn't have the null feature as in other languages
// Option<T> is the enum provided by the standard library
// this checks for if a value is present or not

fn main() {
    // instances of the two variants
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    // instances passing values
    let home = IpAddr::V4(127, 0, 0, 7);
    let loopback = IpAddr::V6(String::from("::1"));
}
