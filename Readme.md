# Semantic Comparison of GraphQL Queries

This CLI tool compares two GraphQL queries that are valid in the given schema and determines if

* One is a subset of another, meaning the former's response shape will always be included in the
  latter's response shape in all cases.
* Or, they are equivalent (since they are mutually a subset of each other).
* Or, there are no subset relationships between them.

The schema is necessary since the semantics of queries are defined by their schema.


## Build

This tool is written in Rust based on the `apollo-compiler` and `apollo-federation` crates. Follow [this instructions](https://www.rust-lang.org/tools/install) to install Rust. Then, execute the following command to build the tool:

```
cargo build
```

## Usage

```
cargo run -- <schema> <query1> <query2>
```

The result of comparison will be printed to stdout.
