# Learn Rust

## Introduction

My self practice following the book [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html).

## Notes

### Installation

Please follow the instructions in [Installation](https://doc.rust-lang.org/book/ch01-01-installation.html).

### Compiling and Running

- Before running a Rust program, you must compile it using the Rust compiler by entering the ```rustc``` command and passing it the name of your source file, like this:

```code
rustc main.rs
```

- After compiling successfully, Rust outputs a binary executable:
  - Windows: ```main.exe``` + ```.pdb``` file (contains debugging information)
  - Other platforms: ```main```
- From here, you can run the ```main.exe``` or ```main``` file, like this:

```code
.\main.exe # on Windows
```

```code
./main   # on other platforms
```

### Cargo

- We can create a project using ```cargo new [project_name]```:

```code
cargo new hello_cargo
cd hello_cargo
```

- We can build a project using ```cargo build```.

- We can build and run a project in one step using ```cargo run```.

- We can build a project **without producing a binary** to check for errors and make sure it compiles using ```cargo check```.

- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the ```target/debug``` directory.

- Cargo commands are the same no matter which operating system you’re working on.

- We can compile the project with optimizations for release using ```cargo build --release``` to create an executable in ```target/release``` instead of ```target/debug```.

### Variables and Mutability

- By default, variables are immutable.
- ```let``` variables can only be used in a function.
- We can make variables immutable by adding ```mut``` in front of the variable names (also intent to future readers of the code that other parts of the code may change this variable's value).

#### Constant

- Cannot use ```mut``` with constant.
- Constants are declared using the ```const``` keyword, and the data type **must be** annotated.
- Constants may be set **only** to a constant expression.
- Rust’s naming convention for constants is to use all uppercase with underscores between words.
- Constants are valid for the entire time a program runs, within the scope they were declared in.

#### Shadowing

- It is possible to declare a new variable with the same name as a previous variable (the second variable is what the compiler will see when the name of the variable is used).
- We can shadow a variable by using the same variable’s name and repeating the use of the let keyword:

```code
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```

- We’ll get a compile-time error if we accidentally try to reassign to this variable without using the ```let``` keyword.
- By using ```let```:
  - We can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
  - When we use the let keyword again, we can change the type of the value but reuse the same name.
