//This program reads your name through stdin and greets you
//A name can only contain alphabetical characters and cannot be empty

//TODO fix the compiler errors and have a look at the warnings

//This program uses `panic!()` to deal with names that do not comply with the name format
//Using `panic!()` is a quick-and-dirty way to do error handling, however, it has the obvious drawback that it is all-or-nothing: you cannot recover from it (in general)

//TODO change to program so it doesn't panic on malformatted names, and handle the compiler warnings
//We have provided an error type for properly reporting all errors that `get_username` might generate
//Change `get_username` so it returns a `Result<String, MyError>` and handle the errors in `main` (an IOError should quit the program, but after an InvalidName error it should repeat the question to the user)
//hint: have a look at `Result`s `or_else` function, and the `?` operator

use std::f32::consts::E;
use std::io;
use std::io::BufRead;
use std::io::Write;

#[derive(Debug)]  // This will automatically implement the Debug trait for MyError
enum MyError {
    InvalidName,
    IOError(std::io::Error),
}

fn get_username() -> Result<String, MyError> {
    print!("Username: ");
    io::stdout().flush(); //flush() is necessary because the print! macro does not automatically flush the output

    let mut input = String::new();
    io::stdin().lock().read_line(&mut input);
    input = input.trim().to_string(); //have a look at the docs to see what `trim` does --> removes leading and trailing whitespaces

    for c in input.chars() {
        if !char::is_alphabetic(c) {
            return Err(MyError::InvalidName);
            panic!("that's not a valid name, try again");
        }
    }

    if input.is_empty() {
        return Err(MyError::InvalidName);
        panic!("that's not a valid name, try again");
        
    }

    Ok(input)

}

fn main() {
    let mut name = get_username();
    
    // Repeat asking for the username while the error is `MyError::InvalidName`
    while let Err(MyError::InvalidName) = name {
        println!("Invalid name, please try again.");
        name = get_username();
    }

    // Check if we have a valid username
    if let Ok(valid_name) = name {
        println!("Hello {valid_name}!");
    } else {
        println!("An error occurred: {:?}", name.unwrap_err());
    }
}