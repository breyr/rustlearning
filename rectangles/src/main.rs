// Example program that utilizes structs and implementing methods
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Methods are similar to functions, but are defined
// within the context of a struct and their first parameter
// is always &self
// impl (implementation)
// all functions defined in impl are called - associated functions
// we can have associated functions that don't have &self - not methods
// associate functions that aren't methods are often contstructors
// new as a keyword isn't built into the language
// it is valid to have multiple impl blocks
impl Rectangle {
    // &self is short for self: &self
    // reference because we don't want to own the data, just read it
    // if we wanted to mutate the instance, we would use &mut self
    // this method acts on a pointer to an instance of Rectangle
    // Rust has automatic referencing and dereferencing, so we can use
    // dot notation to access the method without using &
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.width > other_rect.width && self.height > other_rect.height
    }
    // to call this function, use the :: syntax
    // Self is an alias for the type defined after impl
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let r1 = Rectangle {
        width: 30,
        height: 50,
    };
    let r2 = Rectangle {
        width: 10,
        height: 40,
    };
    let r3 = Rectangle {
        width: 60,
        height: 45,
    };
    // sqaure is namespaced by the struct
    // the :: syntax is used for both associated functions and namespaces created by modules
    let s1 = Rectangle::square(5);
    // borrowing since we want main to be the owner after method call
    println!("Can rect1 hold rect2? {}", r1.can_hold(&r2));
    println!("Can rect1 hold rect3? {}", r1.can_hold(&r3));

    // the trait `std::fmt::Display` is not implemented for `Rectangle`
    // note: in format strings you may be able to use `{:?}`
    // (or {:#?} for pretty-print)
    // :? tells prinln! that we want to use a debug trait
    // add #[derive(Debug)] to struct or impl Debug for Rectangle
    // println!("rect is {:?}", r1);
    // can also print debug info using dbg! macro
    // dbg! prints to the standard error console stream
    dbg!(&r1);
    println!(
        "The area of the rectangle is {} square pixels.",
        // area(&r1) // pass reference to r1
        r1.area()
    );
}

// function definition doesn't state that these numbers are related
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }
//
// definition with tuples
// this definition is better, but remains cryptic
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }
//
// definition with structs (not a method)
// would be better to tie to Rectangle type
// fn area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }
