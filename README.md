# candid-gen

A lightweight CLI tool for generating Candid interface definitions from Rust canisters in Internet Computer (IC) projects.

## Behaviour expected

- args:
  - none: if no arg is passed, create a candid for every rust canister in the project
  - N: receive list of canisters to build
    - string or '.'
- validade input each list item
- build a vec of canisters (validated)
- create the candid for each one
