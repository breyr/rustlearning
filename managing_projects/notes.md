# Managing Rust Projects with Packages, Crates, and Modules

__Module System__
- Packages: A Cargo feature that lets you build, test, and share crates
- Crates: A tree of modules that produces a library or executable
- Modules and use: Let you control the organization, scope, and privacy of paths
- Paths: A way of naming an item, such as a struct, function, or module

## Packages and Crates

A **crate** is the smallest amount of code that the Rust compiler considers at a time.

If you have one file in your project, the compiler considers that file to be a crate.

Two types of crates:
1. Binary
- programs that you compile to an executable that you can run
- each must have a function called main (defines what happens when the program runs)
2. Library
- do not have a main function
- do not need to be compiled
- define functionality to be shared with multiple projects

__crate root__ is a source file that the Rust compiler starts from and makes up the root module of your crate.

A **package** is a bundle of one ore more crates that provide a set of functionality. A package contains a Cargo.toml file which tells how to build the crates.

A package can contain as many binary crates, but only one library crate. It must contain at least one crate.

Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package.

Similarly, if the package directory contains src/lib.rs, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root.

A package can have multiple binary crates by placing them into src/bin, each file will be its own binary crate.

## Defining Modules to Control Scope and Privacy

**Modules Cheat Sheet**
- __Start from the crate root__
- __Declaring modules__
    - In the crate root module, you can declare new modules. For example, declaring a garden module with mod garden; The compiler will look for the module's code in these places:
        - Inlince, within the curly braces that replace the semicolon following mod garden
        - in the file src/garden.rs
        - in the file src/garden/mod.rs
- __Declaring submodules__
    - in any other file than the crate root, you can declare submodules
    - declaring mod vegetables; in src/garden.rs, the compiler will look for the submodule's code in the following places:
        - inline
        - src/garden/vegetables.rs
        - src/garden/vegetables/mod.rs
- __Paths to code in modules__
    - You can refer to code in a module from anywhere else in the same crate, as long as the privacy rules allow, using the path to the code
        - an Asparagus type in the garden vegetables module would be found at crate::garden::vegetables::Asparagus
- __Private vs public__
    - Code within a module is private from its parent modules by default. To make a module public, declare it with pub mod instead of mod.
- __The use keyword__
    - Within a scope, the use keyword creates shortcuts to items to reduce repetition of long paths.

## Grouping Related Code in Modules

Modules allow us to organize code within a crate for readability and easy reuse. They also allow up to control the privacy of items, because code within a module is private by default. Private items are not availabile for outside use, but we can choose to make modules and the items within them public, which exposes them to allow external code to use and depend on them.
