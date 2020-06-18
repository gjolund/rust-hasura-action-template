OpenFaaS Rust Hasura Action template
=============================================

An OpenFaaS Hasura Action Template written in Rust.

## Installation

```sh
$ faas template pull https://github.com/austinrivas/rust-hasura-action-template
$ faas new --list
Languages available as templates:
- rust-hasura-action
```

## Create Function

```sh
faas new <name> --lang rust-hasura-action
```

## Usage

This template provides a thin wrapper around the [Rust Actix Server](https://actix.rs/). The wrapper implementation closely mirrors the [server docs](https://actix.rs/docs/server/).

### [Handler](template/rust-hasura-action/function/src/lib.rs)
### [Wrapper](template/rust-hasura-action/main/src/main.rs)

## Extras

### Example Function

A working example of this function template can be found [here](https://github.com/austinrivas/openfaas_rust-hasura-action).

### Testing

```sh
cargo test --manifest-path ./template/rust-hasura-action/main/Cargo.toml
cargo test --manifest-path ./template/rust-hasura-action/function/Cargo.toml
```

### Format

```sh
cargo fmt --manifest-path ./template/rust-hasura-action/main/Cargo.toml
cargo fmt --manifest-path ./template/rust-hasura-action/function/Cargo.toml
```

### Linting

```sh
cargo clippy --manifest-path ./template/rust-hasura-action/main/Cargo.toml
cargo clippy --manifest-path ./template/rust-hasura-action/function/Cargo.toml
```

### [rust-actix-builder](https://hub.docker.com/r/austinrivas/rust-actix-builder/dockerfile)

A builder Docker image that pre-compiles the included dependencies of the `rust-hasura-action` template.

Pre-compiling dependencies in this manner speeds up function builds by a significant amount.

### [openfaas-binary-runner](https://hub.docker.com/r/austinrivas/openfaas-binary-runner/dockerfile)

A runner Docker alpine image that is optimized to run OpenFaaS binaries with minimal dependencies.

### [rust-hasura-action-okteto](https://hub.docker.com/r/austinrivas/rust-hasura-action-okteto/dockerfile)

Based on `rust-actix-builder` this image is designed to run `rust-hasura-action` OpenFaaS functions on the [Okteto](https://okteto.com/) remote development platform. It includes pre-compiled base dependencies and additional configuration to optimize it for okteto. For additional information on how to configure a function for okteto see [openfaas_rust-hasura-action](https://github.com/austinrivas/openfaas_rust-hasura-action).

### CI / CD

This repo includes [two github actions](.github/workflows) to test / lint / format code in the main wrapper and function.
