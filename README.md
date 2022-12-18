# Learn Rust
## Introduction
My self practice following the book [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html).

## Notes

### Installation
Please follow the instructions in [Installation](https://doc.rust-lang.org/book/ch01-01-installation.html).
### Compiling and Running
- Before running a Rust program, you must compile it using the Rust compiler by entering the ```rustc``` command and passing it the name of your source file, like this:

```code
$ rustc main.rs
```
- After compiling successfully, Rust outputs a binary executable:
    - Windows: ```main.exe``` + ```.pdb``` file (contains debugging information)
    - Other platforms: ```main```
- From here, you can run the ```main.exe``` or ```main``` file, like this:
```code
.\main.exe # on Windows
```
```code
$ ./main   # on other platforms
```
### Cargo
- We can create a project using ```cargo new [project_name]```:
```code
$ cargo new hello_cargo
$ cd hello_cargo
```
- We can build a project using ```cargo build```.
- We can build and run a project in one step using ```cargo run```.
- We can build a project **without producing a binary** to check for errors and make sure it compiles using ```cargo check```.
- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the ```target/debug``` directory.
- Cargo commands are the same no matter which operating system you’re working on.
- We can compile the project with optimizations for release using ```cargo build --release``` to create an executable in ```target/release``` instead of ```target/debug```.