// Code that gets a guess from the user and prints it

use std::io; //import io library (one standard library std)
use rand::Rng; //define methos that random number generators implement
use std::cmp::Ordering; //"Ordering" enum type that has the variants "Less", "Greater", and "Equal".

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng(). //use the random number generator that is local to the current execution thread & is seeded by the OS
                        gen_range(1..=100); //take a range expression as an argument and generates a random number in the range between 1 and 100
    
    //println!("The secret number is: {secret_number}");
    
    loop 
    {
        println!("Please input your guess (between 1 and 100).");
    
        //we use the "let" statement to create the variable.
        //in Rust, variables are immutable by default.
        //to make a variable mutable, we add "mut" before the variable name.
        //"String" is a string type (a growable, UTF-8 encoded bit of text) provided by std.
        //:: syntax indicates that new is an associated function of the "String" type.
        let mut guess = String::new();
        
        io::stdin() //return an instance of "std::io::Stdin" (standard input handler)
            .read_line(&mut guess) //method on stdin, & indicates that this argument is a reference. Return a "Results" value.
            .expect("Failed to read line"); //handle potential failure (skip this function will produce a warning in compilation)
        
        //shadow the previous value of "guess" with a new one. 
        //: syntax tells Rust we'll annotate the variable's type we want, that is an unsigned 32-bit integer.
        //A "match" expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern
        let guess: u32 = match guess.trim() //eliminate any whitespace at the beginning and end.
                              .parse() //convert a string to another type.

                              {
                                Ok(num) => num,
                                Err(_) => continue, //"_" is a catchall value, saying we want to match all Err values, no matter what information they have inside them
                               };
    
        // The "{}" set of curly brackets is a placeholder
        //1st way: put the variable name directly inside the brackets
        println!("You guessed: {guess}");
        //2nd way: place empty brackets in the format string, then follow it with a comma-separated list of expressions
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) { //compare two values and can be called on anything that can be compared
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}