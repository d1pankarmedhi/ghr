# ghr

[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)


A command line tool to replace *help* and *man* in linux for helping developers get structured information about any tool installed locally.

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

Using `gmr` to get better information on using docker cli with the help of openai.

```bash
cargo run -- --command docker
```
**Output 👇**
![preview1](https://github.com/d1pankarmedhi/ghr/assets/136924835/740ee0f4-59e6-4d6a-b622-d31825f7f0ab)

<!-- 
```text
Here are some user-friendly examples to help you understand what the Docker tool does:

1. Creating and running a new container from an image:
   - Command: docker run [OPTIONS] IMAGE
   - Example: docker run -d -p 8080:80 nginx

2. Executing a command in a running container:
   - Command: docker exec [OPTIONS] CONTAINER COMMAND
   - Example: docker exec -it my_container bash

3. Listing containers:
   - Command: docker ps [OPTIONS]
   - Example: docker ps -a

4. Building an image from a Dockerfile:
   - Command: docker build [OPTIONS] PATH
   - Example: docker build -t my_image .

5. Downloading an image from a registry:
   - Command: docker pull [OPTIONS] IMAGE[:TAG]
   - Example: docker pull ubuntu:latest

6. Uploading an image to a registry:
   - Command: docker push [OPTIONS] NAME[:TAG]
   - Example: docker push my_registry/my_image:latest

7. Listing images:
   - Command: docker images [OPTIONS]
   - Example: docker images

8. Logging in to a registry:
   - Command: docker login [OPTIONS] [SERVER]
   - Example: docker login my_registry.com

9. Logging out from a registry:
   - Command: docker logout [SERVER]
   - Example: docker logout my_registry.com

10. Searching Docker Hub for images:
    - Command: docker search [OPTIONS] TERM
    - Example: docker search mysql

These are just a few examples of the commands you can use with Docker. For more information and a complete list of commands, you can visit the official Docker documentation at https://docs.docker.com/go/guides/.
``` -->


## 🛠 Built With

* [Rust](https://www.rust-lang.org/)
* [clap](https://docs.rs/clap/latest/clap/) - a crate to create quick cli tools



## 📃 License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details


