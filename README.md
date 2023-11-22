# Comparative Programming Languages: Rust

This repo contains several Rust exercises to get familiar with the language.
We assume you have theoretical knowledge from the lectures.
If you have questions about these exercises, contact us (in Dutch or English) at alicia.andries@kuleuven.be and ruben.mechelinck@kuleuven.be.

## Introduction & Setup
First, some important background info:
- a Rust project is called a "crate".
- the Rust compiler is "rustc" (https://doc.rust-lang.org/rustc/index.html). We most often do not invoke the compiler directly, but instead rely on Cargo.
- "Cargo" is the Rust package manager (https://doc.rust-lang.org/cargo/index.html). Cargo is the preferred way to build, run, and install crates. Cargo manages the whole build process and deals with external dependencies. It reads the (relatively easy) build configuration from the `Cargo.toml` file in the crate's root. You can find published crates at https://crates.io.
Some basic subcommands:
```shell
    #NOTICE: you will use this one for almost all exercises in this lab
    $ cargo run [--release]  #run (and build if necessary) the crate
```
```shell
    #some others
    $ cargo init  #create a new emtpy crate
    $ cargo build [--release]  #build the crate, invokes rustc automatically
    $ cargo install [<crate>]  #install a crate
```

- toolchain management is done with "rustup" (https://rust-lang.github.io/rustup/index.html). With rustup, you can install and update your Rust toolchain, install different channels of the toolchain (stable, beta, nightly), or individual components.

- **DOCUMENTATION**<br>
The Rust documentation is **incredibly good!!**
This (https://doc.rust-lang.org/std/index.html) is the root page of the documentation for the standard library.
You can easily find the docs for all data structures, functions, etc. with a simple Google search, for example: search for `rust vec` to get the docs for `std::vec::Vec` which is Rust's dynamically allocated vector type.

### Setup
To install all Rust-related components (using rustup): see https://www.rust-lang.org/tools/install.
See https://www.rust-lang.org/tools for extensions for your favourite text editor or IDE, for example, "rust-analyzer" for VS Code.

## Warmup Exercises
These small exercises each deal with a limited number of Rust concepts you should be familiar with before starting on bigger projects. Your goal is to make each exercise compile and run properly. **The Rust compiler gives very thorough error messages, suggestions, and tips. Be sure to leverage them!** These exercises are as much about getting familiar with the compiler messages, the standard library, and the documentation.
The source code for these exercises is located in the `/warmup_exercises/src/bin` directory. You can run them individually using `cargo run --bin <filename_without_extension>` (this works from any subdirectory inside the crate).

- exercise 1, 2, 3, 4: ownership, mutability, move semantics, borrowing
- exercise 5: basic compound types (structs)
- exercise 6: error handling
- exercise 7: traits
- exercise 8: polymorphism (generics with trait bounds)

## Full Exercises
The full exercises are located under `/full-exercises`.
Be sure to read the comments at the top of each exercise file! You can build and run each exercise with `cargo run`, unless stated otherwise.
The stars represent the level of difficulty.
- exercise 1: expression evaluator (basic compound types (enums & structs), pattern matching, heap allocation, error handling)
    :star::star:
- exercise 2: local storage vector (ownership, compound types, pattern matching, generics, trait bounds, slices, documentation)
    :star::star::star:
- exercise 3: GUI (polymorphism (traits, trait objects))
    :star:
- exercise 4: "Bee Movie" analyzer (iterators)
    :star:
- exercise 5: quizzer (dealing with larger projects (modules, external dependencies), file editing, command line arguments)
    :star::star::star:
- exercise 6: GUI part 2 (lifetime parameters)
    :star::star:


## Cheat Sheet
See [cheat_sheet.md](cheat_sheet.md)

## Credits
Most exercises are based on the following projects:
- https://gitlab.com/etrovub/smartnets/rustiec-101/
- https://google.github.io/comprehensive-rust/
- https://github.com/rust-lang/rustlings
- https://practice.rs
