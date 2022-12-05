Integration tests

# tokio::test

`tokio::test` is the testing equivalent of `tokio::main`.
It also spares you from having to specify the `#[test]` attribute.

You can inspect what code gets generated using
`cargo expand --test health_check` (<- name of the test file)

# nextest
optional better test runner
see [cargo-nextest](https://nexte.st/).

`$> cargo nextest run`

