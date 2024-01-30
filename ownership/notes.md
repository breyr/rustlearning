# Ownership

Memory is managed through a system of ownership with a set of rules that the compiler checks at compile time. None of the ownership features slow down your program while it’s running.

## The Stack and The Heap

Both the stack and the heap are parts of memory available to your code to use at runtime, but they are structured differently.

### The Stack

- Last In, First Out (LIFO)
- All data stored on the stack must have a known, fixed size
- Data with an unknown size at compile time, or a size that might change, must be stored on the heap

### The Heap

- less organized
- when you put data on the heap, you request a certain amount of space
- memory allocator finds an empty spot in the heap, marks it as in use, and returns a pointer
- because the pointer to the heap is a known, fixed size, it can be stored on the stack

### Speed Differences

- accessing data in the heap is slower -> have to follow a pointer
- contemporary processors are faster if they jump around less in memory
- a function's parameters, and local variables are push onto the stack
- when the function is over, those values are popped off the stack

## Ownership Rules

1. Each value in Rust has an __owner__
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped

### The String Type

- String type is used as an example because it does not have a known, fixed size at compile time
- string literals are immutable

#### Memory and Allocation

- string literals, since we know the contents at compile time, the text is hardcoded into the final executable
- string literals are fast and efficient, but that is only because of their immutability
- String::from requests the memory needed
- in Rust, the memory is automatically returned once the variable that owns it goes out of scope
- a special function called __drop__ frees the memory

#### Variables and Data Interacting with Move

```rust
let x = 5;
let y = x;
```
Here in this example, we are binding the value 5 to x, then making a copy of the value x and binding it to y. This is what happens because integers have a known fixed size and are pushed onto the stack

```rust
let s1 = String::from("hello");
let s2 = s1;
```
A String is made up of three parts:
1. a pointer to the memory
2. a length
3. a capacity

This group of data is stored on the stack, but the string contents are stored on the heap.

The length is how much memory, in bytes, the String is currently using. The capacity is the total amount of memory in bytes that the String has recieved from the allocator.

When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack. We do not copy the data on the heap that the pointer refers to.

This is because it will be expensive to copy heap data if the size is large.

In the above example, both s1 and s2 point to the same memory location. If both s1 and s2 go out of scope, they will both try to free the same memory. This is known as a __double free__ error. However, Rust considers s1 to no longer be valid after let s2 = s1.

This concept of copying pointers is a __shallow copy__, but because Rust invalidates the first variable, it is known as a __move__

#### Clone

If we do want a deep copy, which copies both heap and stack data, we can use the __clone__ method.

```rust
let s1 = String::from("Hello");
let s2 = s1.clone();
```

#### Stack-Only Data: Copy

Rust has a special annotation called the __Copy__ trait that we can place on types that are stored on the stack. If a type has this trait, variables that use it do not move, but rather are trivially copied.

Rust won't let us annotate a type with the __Copy__ trait if the type, or any of its parts, has implemented the __Drop__ trait.

The following implement Copy:

- All the integer types, such as u32
- The Boolean type, bool, with values true and false
- All the floating point types, such as f64
- The character type, char
- Tuples, if they only contain types that also implement Copy

#### Ownership and Functions

Passing a variable to a function will move or copy, just as asignment does.
(Example in main.rs)

#### Return Values and Scope

Returning values can also transfer ownership. Assigning a value to another variable will transfer ownership (move). When a variable that include data on the heap goes out of scope, the value will be dropped unless ownership of the data has been moved to another variable.

Refereces allow you to refer to the value of a variable without taking ownership of it.

## References and Borrowing

A __reference__ is like a pointer, but is guarenteed to point to a valid value of a particular type for the life of that reference.

To pass a reference to a function, we use the __&__ symbol. This symbol must also be used when declaring a parameters type.

This act is called reference __borrowing__. You cannot modify a borrowed value, by default. This is because the value is immutable by default.

### Mutable References

To allow a reference to be mutable, we must use the __&mut__ syntax.

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(str: &mut String) {
    str.push_str(", world");
}
```

There is one restriction. If you have a mutable reference to a value, you can have no other references to that value. This restriction can prevent data races at compile time.

There is also a similar rule. You cannot have a mutable reference while you have an immutable one to the same value.

### Dangling References

A __dangling pointer__ is a pointer that references a location in memory that may have been given to someone else. Rust compiler prevents this.

## Slices

Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.

A slice is a kind of reference, so it does not have ownership.

A __string slice__ is a reference to part of a String:

```rust
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];

// starting from beginning
let slice = &s[0..5]; // or
let slice = &s[..5];

// until the end
let len = s.len();
let slice = &s[3..len]; // or
let slice = &s[3..];

// whole string
let slice = &s[0..len]; // or
let slice = &s[..];
```

### String Slices as Parameters

&str references are string slices. These are more general than string literals.
It's size is fixed at compile time.

```rust
fn first_word(s: &String) -> &str {

}

// or
fn first_word(s: &str) -> &str {

}
```

### Other Slices

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```
