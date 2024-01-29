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
