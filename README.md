# Unsafe Rust Transmute and Undefined Behavior Example

This project demonstrates advanced (and highly unsafe) Rust programming techniques, including custom transmute implementation and deliberate undefined behavior (UB). This is just a small subset of what is possible with this trick. Checkout the original repo for more ~~ab~~use-cases:

https://github.com/Speykious/cve-rs

**WARNING**: This code is for educational purposes only and should not be used in production. It intentionally invokes undefined behavior and :rotating_light:**WILL**:rotating_light: lead to unpredictable results.

## Contents

- `src/lib.rs`: Main library file containing the `transmute` function and `the_answer` function.
- `src/used_after_drop.rs`: Helper module with functions for extending lifetimes. This is where the main trickery happens

## Key Features

1. Custom `transmute` implementation using enum trickery.
2. `the_answer` function that modifies a value through a shared reference to show undefined behavior.
3. Lifetime extension tricks in `used_after_drop.rs`.

## Running Tests

Tests are provided to demonstrate the undefined behavior:

```
cargo test
```

Note: Results may vary between debug and release builds due to the nature of UB.

## Safety Notice

This code deliberately breaks Rust's safety guarantees. It should not be used as a reference for writing safe Rust code.
