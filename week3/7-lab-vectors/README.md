# Vector Lab

## Objective
The objective of this lab is to understand how to retrieve values from vectors in Rust. By the end of the
lab, you should be able to access vector elements by index, retrieve the first and last values, and handle potential
errors when retrieving values.

## Instructions:
- Open the lab which will place you in the `src` directory to explore the code you will be working with.

- Uncomment each section of code one by one to explore different methods of retrieving values from vectors. Each section
corresponds to a specific operation and is commented out to avoid compilation errors.

- Save the file in your project directory.

- In your project directory, open a terminal or command prompt.

- Run the command `cargo run` to compile and execute the program.

- Observe the output in the console. The program demonstrates various methods of retrieving values from vectors, such as
accessing values by index, retrieving the first and last values, and using pattern matching.

## Reflection Questions:
- What are the different methods of retrieving values from a vector in Rust? How do they differ in terms of error handling
and potential panics?

- In the provided code, what would happen if you uncommented and executed the
line `println!("The third value in the vector is: {}", third_value);`? How would the program output change?

## Challenge Questions:
- Modify the code to handle the case when the vector is empty. Uncomment and complete the code inside the `match`
statement in the `main` function to print a specific message when the vector is empty.

- Extend the program by implementing a function that takes a vector as input and returns the sum of all its elements.
Invoke this function with the `vec` variable in the `main` function and print the sum.

- Remember to test your modifications by running the program and observing the output. Experiment with different scenarios
and vector operations to deepen your understanding of retrieving values from vectors in Rust.

