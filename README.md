# lust-for-rust

Practicing & Learning Rust.

### Installation

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Update rustup with

```shell
rustup update
```

DON'T FORGET TO RESTART THE SHELL AFTER THE INSTALLATION!

### How to run

* Create Directory with `cargo new project_name`
* On each directory, run main.rs file with `cargo run`

**or**

* Use `rustc` to  produce a `hello` binary, and run `hello` binary file

```shell
rustc hello.rs
./hello
```

### How to download dependencies from crates

First, write dependencies on cargo.toml which is manifest file for the project. This is like manifest.json for node.js.

```shell
cargo build
```



## What I like about Rust

* Error direction given by console log
* Code is simple compare to the other systemetic programming languages
* Specific guide: [**Rust Programming Book** | 2018 and thereafter](https://doc.rust-lang.org/book/foreword.html)
* `cargo doc --open` feature for describing dependencies in the project
* Package pamagement system `cargo` exists, whereas C or C++ doesn't provide



## References

### Best dictionaries

* [Rust installation guide](https://www.rust-lang.org/learn/get-started)
* [**Rust Programming Book** | 2018 and thereafter](https://doc.rust-lang.org/book/foreword.html)
* [**Given code examples**](https://doc.rust-lang.org/rust-by-example/)

### Problem Solving Materials
* [Rustlings for exercise](https://exercism.io/tracks/rust/exercises)
* [Rust Exercises for beginners - exercism](https://exercism.io/tracks/rust)

### Project Materials

* [Rust Trending Github Project](https://github.com/trending/rust)
* [Python and Rust 1 to 1 Syntax Comparison | 2018](https://www.youtube.com/watch?v=0Yox95Uxhak)
  * [PDF Slide](https://uc338bc702cd83091b9c4f5f747d.dl.dropboxusercontent.com/cd/0/inline2/A6LKQWH1rxWAXB_8Mjshv10fLpAQ3ZMsM8BqjoQXNm7dbaj4vnSApxZU6FvtOmy4QrvqVPoRsJL7JoXBGNHAgGIcFwmAHQ0X9ACSE3NqEpixhRNuiswD6M3LlvmiFZaF9QHnP1rOz8dOsQgENw0v1k-6zXOYEkYpsGh4Iqa1XECLcMg_mpU4X14_q4OHlhdQjvJExcMpK0R4Lr0rwjNu3A-NHxu5S6olcPvPDdUeSbeApgmNXY4OW2YlluHYHVfS2SEm0lwxVOP8-wYkD_spBkjqP8jXcFgs9uOutcna3VoRCPepY_vkl3HG_xGmM0-7wNPSTOpNycX6EqhXt4cUO_P1/file#)

### Difficult Materials

* [RustPython Video 1 ](https://www.youtube.com/watch?v=YMmio0JHy_Y) and [RustPython Video 2](https://www.youtube.com/watch?v=nJDY9ASuiLc)
* [Rust for python programmers 2015](https://lucumr.pocoo.org/2015/5/27/rust-for-pythonistas/)



## Python keyword to Rust keyword

|         | Python |        Rust        |
| :-----: | :----: | :----------------: |
| string  |  str   |       String       |
| boolean |  bool  |        bool        |
| integer |  int   | i32, i64, u32, u16 |
|  float  | float  |      f32, f64      |



## Rust Keywords

[Source](https://doc.rust-lang.org/book/appendix-01-keywords.html)

- `as` - perform primitive casting, disambiguate the specific trait containing an item, or rename items in `use` and `extern crate` statements
- `async` - return a `Future` instead of blocking the current thread
- `await` - suspend execution until the result of a `Future` is ready
- `break` - exit a loop immediately
- `const` - define constant items or constant raw pointers
- `continue` - continue to the next loop iteration
- `crate` - link an external crate or a macro variable representing the crate in which the macro is defined
- `dyn` - dynamic dispatch to a trait object
- `else` - fallback for `if` and `if let` control flow constructs
- `enum` - define an enumeration
- `extern` - link an external crate, function, or variable
- `false` - Boolean false literal
- `fn` - define a function or the function pointer type
- `for` - loop over items from an iterator, implement a trait, or specify a higher-ranked lifetime
- `if` - branch based on the result of a conditional expression
- `impl` - implement inherent or trait functionality
- `in` - part of `for` loop syntax
- `let` - bind a variable
- `loop` - loop unconditionally
- `match` - match a value to patterns
- `mod` - define a module
- `move` - make a closure take ownership of all its captures
- `mut` - denote mutability in references, raw pointers, or pattern bindings
- `pub` - denote public visibility in struct fields, `impl` blocks, or modules
- `ref` - bind by reference
- `return` - return from function
- `Self` - a type alias for the type we are defining or implementing
- `self` - method subject or current module
- `static` - global variable or lifetime lasting the entire program execution
- `struct` - define a structure
- `super` - parent module of the current module
- `trait` - define a trait
- `true` - Boolean true literal
- `type` - define a type alias or associated type
- `union` - define a [union](https://doc.rust-lang.org/reference/items/unions.html) and is only a keyword when used in a union declaration
- `unsafe` - denote unsafe code, functions, traits, or implementations
- `use` - bring symbols into scope
- `where` - denote clauses that constrain a type
- `while` - loop conditionally based on the result of an expression