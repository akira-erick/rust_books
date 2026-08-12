# Rust Books CRUD

A simple command-line CRUD application for managing books, built with Rust.

The application allows you to create, update, delete, display, list, and sort books. Books are stored in memory while the application is running.

## Requirements

* [Rust](https://www.rust-lang.org/tools/install)
* Cargo (included with Rust)

## Running the Project

The Rust project is located in the `rust_books` directory.

From the project root, run:

```bash
cd rust_books
cargo run
```

Alternatively, you can run it directly with:

```bash
cargo run --manifest-path rust_books/Cargo.toml
```

## Available Commands

When the application starts, you will see a menu similar to:

```text
CRUD de Livros

1 - Inserir livro
2 - Alterar livro
3 - Apagar livro
4 - Exibir livro
5 - Listar livros
0 - Sair

Escolha uma opção:
```

Choose an option and follow the instructions in the terminal.

## Development

Check the project without running it:

```bash
cd rust_books
cargo check
```

Build the project:

```bash
cd rust_books
cargo build
```

Run the optimized version:

```bash
cd rust_books
cargo run --release
```

## Notes

The books are stored **in memory**, so all data is lost when the application exits.
