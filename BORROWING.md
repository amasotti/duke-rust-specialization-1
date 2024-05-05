# Rust Ownership concept

Rust most famous, interesting and unique feature is its ownership concept. 
It is a way to manage memory in Rust. 
It is a set of rules that the compiler checks at compile time. The rules are:

1. Each value in Rust has a variable that’s called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.


Now take this simple Rust code:

~~~rust
fn main() {
	let s = String::from("A wonderful test string");
	own_string(s);
	
	println!("The string is no longer available here: {}", s);
}


fn own_string(s: String) {
	println!("I own this string: {}", s);
}
~~~

This code will not compile. The error message is:

~~~sh 
   Compiling proj v0.1.0 (/home/toni/half-personal/rust/rust_spec_coursera/rust-specialization-course1-foundamentals/week2/3-functions/proj)
error[E0382]: borrow of moved value: `s`
 --> src/borrowing_demo.rs:5:60
  |
2 |     let s = String::from("A wonderful test string");
  |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
3 |     own_string(s);
  |                - value moved here
4 |     
5 |     println!("The string is no longer available here: {}", s);
  |                                                            ^ value borrowed here after move
  |
note: consider changing this parameter type in function `own_string` to borrow instead if owning the value isn't necessary
 --> src/borrowing_demo.rs:9:18
  |
9 | fn own_string(s: String) {
  |    ----------    ^^^^^^ this parameter takes ownership of the value
  |    |
  |    in this function
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
3 |     own_string(s.clone());
  |                 ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `proj` (bin "proj") due to 1 previous error
~~~

The compiler in Rust is very friendly and is saying us exactly what's happening under the hood:

1. We create a string `s` in the main function.
2. We pass `s` to the `own_string` function. This is a `move` operation. The `own_string` function now owns the string.
Interesting enough, the compiler tells us an important detail:
> move occurs because `s` has type `String`, which does not implement the `Copy` trait

Indeed we wouldn't have a problem with integers or booleans. The reason is that these
have a fixed size, so Rust knows how much memory to reserve and just copies the value. But
for `String` it is different. `String` is a complex data structure that has a pointer to the heap, 
a length and a capacity. 
Rust doesn't know how much memory to reserve, so it just moves the ownership.

3. Once the variable is moved to `own_string`, it is no longer available in other scopes. This function
doesn't return the string, so it will be dropped when the function ends (see rule 3).

The solution is to pass a reference

~~~rust
fn main() {
	let s = String::from("A wonderful test string");
	own_string(&s);
	
	println!("The string is still available here: {}", s);
}

fn borrow_string(s: &String) {
	println!("I borrowed this string: {}", s);
}
~~~

## Borrowing

Borrowing in Rust comes in two form: immutable and mutable.
Have a look at this code:

~~~rust
fn main() {
	let mut v = vec![1, 2, 3, 4];

	own_vec(v); // Ownership is moved to own_vec; `v` can no longer be used in `main` after this point.

	// To continue using `v`, we need to reinitialize it as it was moved in the `own_vec` function.
	let mut v = vec![1, 2, 3, 4];

	borrow_vec_mutable(&mut v); // `v` is borrowed mutably, modified, then the borrow ends.
	println!("After mutable borrow: {:?}", v);  // `v` is accessible again, showing all changes.

	borrow_vec_immutable(&v); // `v` is borrowed immutably, it's read, but not modified.
	println!("After immutable borrow: {:?}", v);  // `v` is accessible again, unchanged.
}

fn own_vec(mut v: Vec<i32>) {
	v.push(5);
	println!("own_vec owns and modifies the vector: {:?}", v);
	// `v` goes out of scope here, and its memory is freed.
}

fn borrow_vec_mutable(v: &mut Vec<i32>) {
	v.push(6);
	println!("borrow_vec_mutable borrows and modifies the vector: {:?}", v);
	// Mutable borrow ends here.
}

fn borrow_vec_immutable(v: &Vec<i32>) {
	println!("borrow_vec_immutable borrows and accesses the vector: {:?}", v);
	// Immutable borrow ends here.
}
~~~

### Syntax for borrowing and moving in Rust

1. **Moving Ownership**

   - **Syntax:** `own_vec(v);`
   - **Behavior:** When `v` is passed to `own_vec`, ownership of v is transferred to the function. 
	 After the move, v is no longer accessible in the calling function (main in this case) because v 
	 has been moved and is now owned by own_vec.
   - **Use Case:** Use moving when the receiving function needs to take ownership of the value, 
	 often because it needs to outlive the scope of the calling function or be mutated freely 
	 without affecting the original.

2. **Mutable Borrowing**

   - **Syntax:** `borrow_vec_mutable(&mut v);`
   - **Behavior:** Passing `&mut v` allows borrow_vec_mutable to modify v. 
	 During the period v is borrowed mutably, no other parts of the code (including the caller) 
	 can access or borrow v, either mutably or immutably.
   - **Use Case:** Mutable borrowing is useful when a function needs to modify the borrowed data. 
	 It ensures that only one mutable reference exists at a time, preventing data races.

3. **Immutable Borrowing**

   - **Syntax:** `borrow_vec_immutable(&v)`;
   - **Behavior:** Passing &v provides borrow_vec_immutable read-only access to v. 
	 Multiple immutable borrows can coexist, but they cannot coexist with a mutable borrow.
   - **Use Case:** Immutable borrowing is used when the data only needs to be read. 
	 It allows multiple parts of your program to safely read the data without risk of it 
	 being changed elsewhere.
