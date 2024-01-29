fn main() {
    // s is not valid
    let s = "hello"; // s is valid

    // do stuff with s

    let mut s2 = String::from("hello");
    s2.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s2); // This will print `hello world!`
} // s is no longer valid
