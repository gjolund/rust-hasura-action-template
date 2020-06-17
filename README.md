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

## Testing

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

## Usage

This template provides a thin wrapper around the [Rust Actix Server](https://actix.rs/). The wrapper implementation closely mirrors the [example server](https://github.com/seanmonstar/warp#example).

### [Example Handler](template/rust-warp/function/src/lib.rs)
### [Wrapper](template/rust-warp/main/src/main.rs)

## Example Function

A working example of this function template can be found [here](https://github.com/austinrivas/openfaas_rust-hasura-action).

## Extras

### [rust-actix-builder](https://hub.docker.com/r/austinrivas/rust-actix-builder/dockerfile)

A builder Docker image that pre-compiles the included dependencies of the `rust-hasura-action` template.

Pre-compiling dependencies in this manner speeds up function builds by a significant amount.

### [rust-actix-runner](https://hub.docker.com/r/austinrivas/rust-actix-runner/dockerfile)

A runner Docker alpine image that is optimized to run OpenFaaS rust binaries with minimal dependencies.

### [rust-hasura-action-okteto](https://hub.docker.com/r/austinrivas/rust-hasura-action-okteto/dockerfile)

Based on `rust-actix-builder` this image is designed to run `rust-hasura-action` OpenFaaS functions on the [Okteto](https://okteto.com/) remote development platform. It includes pre-compiled base dependencies and additional configuration to optimize it for okteto. For additional information on how to configure a function for okteto see [openfaas_rust-hasura-action](https://github.com/austinrivas/openfaas_rust-hasura-action).

### CI / CD

This repo includes [two github actions](.github/workflows) to test / lint / format code in the main wrapper and function.
