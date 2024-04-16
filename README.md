# Comfy Language (WIP)

Comfy is a programming language that aims to have a very similar syntax to Rust, but with some additional features and a different import system. The goal is to create a language that is both statically and dynamically typed, with no borrow checking and support for generics without lifetime parameters. It is also planned to have structs with methods, enums and pattern matching. Comfy will be compiled to C++ and will be able to call C++ code.

## Features (in development)

- Statically typed
- Type inference
- Import system
- No borrow checking
- C++ support
- Translation into C++

## Project Structure

The project is structured as follows:

- `compiler` - The Comfy compiler
- `parser` - The Comfy parser
- `types` - The Comfy types for both the compiler and parser
- `utils` - Utilities for the Comfy compiler and parser

Each directory has its own README with more information.

## Setup

You'll need to have Rust and clang++ installed. Once you have Rust installed, you can clone the repository and run `cargo build`.

Parser and compiler have their own CLI.

## License

Comfy is licensed under the MIT license.
