use std::io;    // std > io module

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();  // mutable string variable
    // String::new -> return a new instance of a String
    // String is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.
    // :: -> static methods

    // if hadn't put use std::io,
    // can use like this --> std::io::stdin
    io::stdin().read_line(&mut guess)   // & > call by reference
        .expect("Failed to read line");

    // let _result = io::stdin().read_line(&mut guess);
    // println!("{}", _result);

    println!("You guessed: {}", guess);
}
