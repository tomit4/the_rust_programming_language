## Writing Automated Tests

The following includes related shell commands to this section of the book

**Project Setup:**

```sh
cargo new adder --lib && cd adder
```

**Running Cargo's Test Runner:**

```sh
cargo test
```

**Telling Cargo's Test Runner How Many Threads To Run:**

```sh
cargo test -- --test-threads=1
```

**Tell Cargo's Test Runner To Show println! Statements Properly In Test Output:**

```sh
cargo test -- --show-output
```

**Run only ONE test function called `one_hundred`:**

```sh
cargo test one_hundred
```

**Run all tests that start with `add`:**

NOTE: You CANNOT run multiple tests from the command line, only one argument can
be passed to `cargo test`. The following will NOT work:

```sh
cargo test add_one add_two
```

Instead, you can choose to run all tests that start with `add` like so:

```sh
cargo test add
```

**Ignoring tests:**

Sometimes certain tests should be ignored during regular testing, but run during
specific situations. For this, rust provides the `#[ignore]` macro, that tells
cargo to not run the test function that follows, see `./tests_03/` for a
demonstration of a test that sleeps for 1 hour as a demonstration.

```rust
#[test]
#[ignore]
fn expensive_test() {
    // ... code that takes an hour to run
}
```

NOTE that if you want to ONLY run tests with the `#[ignore]` macro, you can pass
the `--ignored` flag to the cargo test runner like so:

```sh
cargo test -- --ignored
```

### Integration Tests

`cargo test` can be utilized for both unit and integration tests. Unlike unit
tests, there is no need to use the `#[cfg(test)]` macro, as Cargo knows that any
tests within the `./tests` directory are tests to be run. Specifically, these
tests are <em>Integration Tests</em>, mean to work in conjunction with other
tests. This is unlike <em>Unit Tests</em>, which are meant to be run in
isolation of each other. See `./tests_05` for an example

You can run specific files within your `tests` directory by invoking them by
name within the `cargo test` command like so:

```sh
cargo test --test integration_test
```

Where `integration_test` specifically refers to running
`./tests/integration_test.rs`.

**Integration Test Common Conventions:**

Sometimes you need to run some sort of setup function or have other code that
your integration tests need to see, but is not part of your actual program.
Cargo knows of a common convention where within your `tests` directory, you make
a subdirectory called `common` and a `mod.rs` file is instantiated within it. By
using the `mod` keyword within your integration test code, you can call any
functions associated within this `common/mod.rs` file at any point during your
test. See `./tests_05` for an example.

```rust
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, Adder::add_two());
}
```
