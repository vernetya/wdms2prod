# linter
**clippy** 
```
rustup component add clippy
cargo clippy
cargo clippy -- -D warnings
```

specific warning can be muted using `#[allow(clippy::lint_name)]`


# Formatting
**rustfmt**  
relies on `rustfmt`

# vulnerabilities
**cargo-audit**  
`cargo audit`

# *[optional]* expand macros
**cargo-expand**

`cargo expand`