# ghr

[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)


Generative Helper in Rust | A command line tool to replace *help* and *man* in linux for helping developers get structured information about any tool installed locally.

## 🏃 Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. 

Clone the repository.
```bash
git clone https://github.com/d1pankarmedhi/ghr.git
```

Build the project dependencies
```bash
cargo build
```

Run project
```bash 
cargo run -- --command docker # replace docker with your tool of choice.
```

## 📎 Example


Using `ghr` to get better information on using docker cli with the help of openai.

```bash
cargo run -- --command docker
```
**Output 👇**

![preview1](https://github.com/d1pankarmedhi/ghr/assets/136924835/740ee0f4-59e6-4d6a-b622-d31825f7f0ab)

**Build and run binary**

You can run the **main.py** file or run the binary inside the **target** directory, produced after building the project. 

Use `cargo build --release` to build a smaller and optimized binary for release. 

> Run the binary, for example `./target/release/ghr --command docker`.

## 🛠 Built With

* [Rust](https://www.rust-lang.org/)
* [clap](https://docs.rs/clap/latest/clap/) 
* [OpenAI](https://openai.com/)


## 📃 License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details


