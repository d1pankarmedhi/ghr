# ghr

[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)


Generative Helper in Rust | GHR | - A command line tool to help developers generate boilerplate code templates.

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

Make sure to set your openai key as environment variable
```bash
export OPENAI_API_KEY=<api-key>
```

Run project
```bash 
cargo run 
```

## 📎 Example


Generating **Dockerfile** template for a python fastapi server using **ghr**.

```bash
cargo run
```
### Output

![preview](https://github.com/d1pankarmedhi/ghr/assets/136924835/a6cecf3d-6685-407a-94e8-300d429ea9d5)


### Build and run Binary

You can run the **main.py** file or run the binary inside the **target** directory, produced after building the project. 

Use `cargo build --release` to build a smaller and optimized binary for release. 

### Todo
- [x] OpenAI response processing
- [x] File saving
- [ ] Error handling
- [ ] Support for code template


## 🛠 Built With

* [Rust](https://www.rust-lang.org/)
* [clap](https://docs.rs/clap/latest/clap/) 
* [OpenAI](https://openai.com/)


## 📃 License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details


