fn main() {
    // s is not valid
    let s = String::from("hello"); // s is valid

    let len = calc_length(&s);
    println!("{len}");

    // do stuff with s
    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    // println!("{}", s);

    let x = {
        let y = 5;
        y + 1
    };

    let x = 5;
    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it’s okay to still
                   // use x afterward
    println!("{x}");

    let mut s2 = String::from("hello");
    s2.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s2); // This will print `hello world!`
} // s is no longer valid

fn takes_ownership(str: String) {
    println!("{}", str);
} // str is out of scope and 'drop' is called, freeing memory

fn makes_copy(int: i32) {
    // int comes into scope
    println!("{}", int);
} // int goes out of scope, but nothing happens because its a copy

fn calc_length(s: &String) -> usize {
    s.len()
} // here s goes out of scope, but because it doesn't have ownership, its not dropped

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        // iter() returns each element in a collection
        if item == b' ' {
            // b' ' is a byte literal
            return &s[0..i]; // return a slice of the string from 0 to i
        }
    }

    &s[..]
}
