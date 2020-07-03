# lust-for-rust

Practicing & Learning Rust.

### Installation

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

or update rustup with

```shell
rustup update
```

DON'T FORGET TO RESTART THE SHELL AFTER THE INSTALLATION!

### How to download dependencies from crates

First, write dependencies on cargo.toml which is manifest file for the project. This is like manifest.json for node.js.

```shell
cargo build
```

### How to run

Run main.rs file with `cargo run`

```shell
cargo --version
cargo run
```

**or**

Use `rustc` to  produce a `hello` binary, and run `hello` binary file

```shell
rustc hello.rs
./hello
```



## Python keyword to Rust keyword

|         | Python |        Rust        |
| :-----: | :----: | :----------------: |
| string  |  str   |       String       |
| boolean |  bool  |        bool        |
| integer |  int   | i32, i64, u32, u16 |
|  float  | float  |      f32, f64      |



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