# candid-gen

![GitHub release (latest by date)](https://img.shields.io/github/v/release/cwnt-io/candid-gen)
![GitHub](https://img.shields.io/github/license/cwnt-io/candid-gen)
<!-- ![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/cwnt-io/candid-gen/rust.yml) -->

<!-- toc -->

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
  - [Prerequisites](#prerequisites)
  - [Installing](#installing)
- [Usage](#usage)
  - [Command Line Interface](#command-line-interface)
  - [Arguments](#arguments)
  - [Options](#options)
  - [Examples](#examples)
- [Configuration](#configuration)
  - [Environment Setup](#environment-setup)
- [Contributing](#contributing)
  - [Development](#development)
- [License](#license)
- [Authors](#authors)
- [Acknowledgements](#acknowledgements)
- [Contact](#contact)
- [Links](#links)

<!-- tocstop -->

## Overview

`candid-gen` is a CLI tool that automates the generation of Candid interface files from Rust canisters for Internet Computer (IC) projects. It simplifies the process of creating `.did` files, ensuring that your Rust canisters are properly documented and ready for deployment on the IC.

## Features

- Automatically generates Candid interface files from Rust canisters.
- Supports specifying individual canisters or generating files for all canisters in a project.
- Ensures that all necessary tools and targets are installed and available.

## Installation

### Prerequisites

- Rust and Cargo
- `rustup` with `wasm32-unknown-unknown` target installed
- `candid-extractor`

### Installing

To install `candid-gen`, you can clone the repository and build the project:

```sh
git clone https://github.com/cwnt-io/candid-gen.git
cd candid-gen
cargo install --path .
```

## Usage

### Command Line Interface

```sh
candid-gen [CANISTERS_NAMES]...
```

### Arguments

- `[CANISTERS_NAMES]...`: Specify one or more canister names to generate Candid files. Each canister name should be provided as a separate argument. If no canister name is passed, this will generate the Candid files for every rust canister of the project.

### Options

- `-h`, `--help`: Print help information
- `-V`, `--version`: Print version information

### Examples

Generate Candid files for all Rust canisters in the project:

```sh
candid-gen
```

Generate Candid files for specific canisters:

```sh
candid-gen canister1
candid-gen canister1 canister2
```

## Configuration

### Environment Setup

Ensure that the following tools are installed and available in your PATH:

- `rustup`
- `cargo`
- `candid-extractor`

Also, make sure the `wasm32-unknown-unknown` target is installed:

```sh
rustup target add wasm32-unknown-unknown
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you would like to contribute to `candid-gen`.

### Development

To set up a development environment:

1. Clone the repository:
    ```sh
    git clone https://github.com/cwnt-io/candid-gen.git
    cd candid-gen
    ```

2. Install dependencies:
    ```sh
    cargo build
    ```

3. Run tests:
    ```sh
    cargo test
    ```

## License

This project is licensed under the terms of the MIT license. See the [LICENSE](LICENSE) file for details.

## Authors

- Gustavo Basso - [gubasso@cwnt.io](mailto:gubasso@cwnt.io)
- Ismael Pamplona - [isma@cwnt.io](mailto:isma@cwnt.io)

## Acknowledgements

- Inspired by the needs of the Internet Computer community.
- Built with Rust, Clap, and other open-source tools.

## Contact

For any inquiries, please reach out to the authors via email or open an issue on GitHub.

## Links

- [Repository](https://github.com/cwnt-io/candid-gen)
