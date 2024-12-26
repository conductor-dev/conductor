# Conductor

Conductor is a modular node-based framwork for data acquisition, written in Rust and designed to be used for real-time applications.

## Package Structure

The project is structured as a workspace. The members can be found in the `crates` directory.

- `conductor-core`: contains the core functionality of Conductor, including all APIs and building blocks for implementing nodes and pipelines.

- `conductor-nodes`: is a library of commonly used nodes.

- `conductor`: is the main crate and re-exports the other crates.

## Development Setup

Our code style uses the default `rustfmt` and `clippy` settings and an opinionated `.editorconfig`. It is enforced by GitHub Actions.

### Prerequisites

The only prerequisite for installing and/or using this library is [Rust](https://www.rust-lang.org/tools/install). We also provide a `Makefile` for some common commands.

### Useful commands:

- `make build`: builds the library.

- `make clean`: cleans up the project and removes the build directory.

- `make format`: formats all rust files using `rustfmt`.

- `make lint`: checks if all the default `clippy` linting rules are fulfilled. Also fails if there are any warnings.

- `make test`: runs all unit and documentation tests.
