# Basic program - learn the fundamentals

## Part 1: Get acquainted with cargo

_nothing special here_

## Part 2: Makefile

It is a common practice to add a Makefile to Rust projecst (and not only rust).
The Makefile is a simple way to automate the build process.

```bash
# build the project
make build
```

is a shortcut behind which the following command is hidden:

```makefile

build:
	lint 
	@cargo build
    
lint:
	@rustup component add clippy 2> /dev/null
	@cargo clippy
```

## Part 3: Variable assignment and Immutability

In Rust, variables are immutable by default. 
This means that once a value is assigned to a variable, it cannot be changed.

If you want to make a variable mutable, you need to use the `mut` keyword.

```rust
let x = 5; // immutable
x = 6; // this is **NOT** allowed

let mut y = 5; // mutable
y = 6; // this is allowed
```


