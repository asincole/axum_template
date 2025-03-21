# Axum Template

## Aim

This project serves as a starter template for building web applications using Rust and the Axum web framework. It
provides a basic, opinionated project structure and configuration to quickly get me (or you) started with my (or your)
next Axum project.

## Features

* Basic Axum project setup
* Swagger Documentation setup
* Structured directory layout
* Example configuration with `Cargo.toml` and `Justfile`
* Includes logging with `tracing`

## Technologies Used

* [Rust](https://www.rust-lang.org/)
* [Axum](https://github.com/tokio-rs/axum)
* [Tokio](https://tokio.rs/)
* [Serde](https://serde.rs/)
* [Tracing](https://github.com/tokio-rs/tracing)
* [Just](https://just.systems/)

## Prerequisites

Before you begin, ensure you have the following installed:

* [Rust](https://www.rust-lang.org/tools/install) (version 1.75 or higher)
* [Just](https://just.systems/install/) (for running commands defined in the `Justfile`)

## Getting Started

1. **Clone the repository:**

   ```bash
   git clone <repository_url>
   cd axum_template
   ```

2. **Run the project:**

   ```bash
   just dev-watch
   ```

   This will start the development server with hot reloading, using `cargo watch`. You can install cargo
   watch [here](https://crates.io/crates/cargo-watch).

## Project Structure

## Running the Application

The `Justfile` provides convenient commands for running and testing the application:

* `just dev`: Runs the application in development mode using `cargo run`.
* `just dev-watch`: Runs the application in development mode with hot reloading using `cargo watch`.
* `just test`: Runs the tests using `cargo test`.

## Testing

Run tests using the command:

   ```bash
   just test
   ```

## Publishing

To publish the project, you will need to:

1. Update the `Cargo.toml` file with the correct version and other metadata.
2. Build the project for release: `cargo build --release`.
3. Publish the crate to crates.io: `cargo publish`

## Additional Information

Feel free to customize this template to suit your specific needs. Refer to the documentation for Rust, Axum, and the
other dependencies for more information on how to use them.
