# Reflection Questions

1. What is the purpose of the `match` control flow statement in Rust?
   How does it differ from using multiple `if` statements or `if-else` chains?

> The match control flow statement in Rust is used for pattern matching. It allows you to compare a value against a
> series of patterns and execute code based on which pattern matches.
>
> Here's how it differs from using multiple if statements or if-else chains:
>
> * __Exhaustiveness:__ match ensures exhaustiveness, meaning you must handle all possible cases. If you don't cover all
    possibilities, Rust will give you a compile-time error.
>
>
>* __Pattern matching:__ match enables more complex pattern matching, including matching against specific values,
   ranges, structs, enums, and more.
>
>
>* __Clarity and readability:__ match can often lead to more concise and readable code, especially when dealing with
   multiple conditions.
>
>Overall, match is preferred in Rust when you have multiple conditions to check and want to ensure exhaustiveness and
> clarity in your code.

2. What is a struct in Rust, and what purpose does it serve? How is it different from other data types in the language?

> In Rust a structure (short `struct`) s a complex data type that groups together related variables (fields) under one
> name. This allows you to organize data into a meaningful group, making it easy to manage and use. Structs are one of
> the
> core data structures in Rust and are used extensively to create custom data types.
>
> __Purpose:__
>
> - Code and logic organization
> - Reusability
> - Extensibility (structs can be extended)

3. Why is the `#[derive(Debug)]` attribute used for the structs? What benefit does it provide?

> The #[derive(Debug)] attribute in Rust is used with structs (and enums) to automatically generate an implementation of
> the std::fmt::Debug trait for the type. This trait allows you to format your type using the {:?} formatter in
> strings—most commonly within the println! or format! macros. This is especially useful for debugging purposes, as it
> lets you easily inspect the values of instances of your types.
