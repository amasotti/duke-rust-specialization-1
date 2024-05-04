# Reflection Questions


1. What is the purpose of the `match` control flow statement in Rust? 
How does it differ from using multiple `if` statements or `if-else` chains?

> The match control flow statement in Rust is used for pattern matching. It allows you to compare a value against a series of patterns and execute code based on which pattern matches.
>
> Here's how it differs from using multiple if statements or if-else chains:
>
> * __Exhaustiveness:__ match ensures exhaustiveness, meaning you must handle all possible cases. If you don't cover all possibilities, Rust will give you a compile-time error.
>
> 
>* __Pattern matching:__ match enables more complex pattern matching, including matching against specific values, ranges, structs, enums, and more.
>
> 
>* __Clarity and readability:__ match can often lead to more concise and readable code, especially when dealing with multiple conditions.
>
>Overall, match is preferred in Rust when you have multiple conditions to check and want to ensure exhaustiveness and clarity in your code.
