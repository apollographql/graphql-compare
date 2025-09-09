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

## Response Shape

We compare two queries by performing a subset test of their normalized response
shape representation. Response shape is similar to response value, but instead
of a concrete value returned by the underlying resolve we record response field
definition which will be invoked by the resolver, i.e. given a query

```graphql
{
  x(arg: 1)
}
```

We can represent it as

```
{
  x → x(arg: 1) on { Query }
}
```

### Aliases

Since we are mapping responses, we capture aliases as our response key

```graphql
{
  y: x(arg: -1)
}
```

```
{
  y → x(arg: -1) on { Query }
}
```

### Type Conditions

For fields on an abstract type, there might be multiple possible definitions,
depending on their type conditions.

```graphql
{
  x {
    ... on T {
      t
      ... on U {
        u
      }
    }
    ... on S {
      t
      s
    }
  }

}
```

In order to simplify the comparison we always explicitly capture the type
conditions for each field and field sub selections are captured as sub-response
shapes. If there are multiple nested conditions, we capture the condition as
an intersection between all possible types.

```
{
  x → x(arg: 1) on { Query } {
    t → t on { T, S }
    u -> u on T ∩ U
    s → s on { S }
  }
}
```

### Boolean Conditions

`@include` and `@skip` directive information is captured as boolean conditions.

```graphql
query($v1: Boolean!, $v2: Boolean!, $v3: Boolean!) {
  ... @include(if: $v1) {
    x(arg: 0) @include(if: $v2)
  }
  x1: x(arg: 1) @skip(if: $v2)
  x2: x(arg: 2) @include(if: $v1) @skip(if: $v2)
  x2: x(arg: 2) @include(if: $v1) @skip(if: $v3)
}
```

Multiple conditions are captured conjunctively (AND). If we have multiple variants
of a field with different conditions, we capture those disjunctively (OR).

```
{
  x → x(arg: 0) if v1 /\ v2
  x1 → x(arg: 1) if ¬v2
  x2 → x(arg: 2) if (v1 /\ ¬v2) \/ (v1 /\ ¬v3)
}
```

## Licensing

Source code in this repository is covered by the Elastic License 2.0. The
default throughout the repository is a license under the Elastic License 2.0,
unless a file header or a license file in a subdirectory specifies another
license. [See the LICENSE](./LICENSE) for the full license text.