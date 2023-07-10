// #!#![allow(unused)]
// #![allow(dead_code)]

use std::io; // import the crate library

fn get_string() -> io::Result<String> {
    let mut buffer = String::new();
    
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer)
}

fn main() -> io::Result<()> {
    let mut user_input = String::new(); // variable to store the user input
    let stdin = io::stdin(); // get the stdin here
    stdin.read_line(&mut user_input)?; // read the user input

    println!("{user_input}"); // and here print the input

    Ok(())


}
