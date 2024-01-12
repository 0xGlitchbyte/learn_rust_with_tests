# Hello World

## Start new project

To start a new project, we use the `cargo new` command. This will generate everything we need to get our program running.  

`cargo new hello_world`

The structure of the new directory looks like this: 
```markdown
hello_world
├── Cargo.toml
└── src
    └── main.rs
```
With:

- `hello_world` as our project name.
- `Cargo.toml` is the file that manages our project.
- `src` is the directory all our code goes in.
- `main.rs` is the file that holds our code.

If we open `main.rs`, we will see the traditional "Hello, World!" code.

```rust
fn main() {
  println!("Hello, World!");
}
```

Every program requires a `main()` function to run, since this is where every program we write begins at.

To run our code, we can type `cargo run` in our terminal.

This will:

- Build our code into a binary file.
- Run our code after the build is finished.

Once `cargo run` finishes running, we should see "Hello, World!" in our terminal.

## Hello, World!

Every `cargo new` project starts with "Hello, World!" using the `println!` macro.
```rust
{{#rustdoc_include ../examples/01_hello_world/hello_world/src/main.rs:print}}
```

However, when we write tests, we want to separate our "domain" code from the outside world (side effects). The outside world is dealing with file input/output, printing to the terminal, interfacing with databases, so on and so forth.

To be testable, we need to turn our "Hello, World!" into a function and print the result of the function to the screen separately.

First, we write the function.

```rust
{{#rustdoc_include ../examples/01_hello_world/hello_world/src/main.rs:hello_world}}
```

- The `fn` keyword is how we define our function.
- `hello_world` is the name of our function.
- `()` is where would would define our arguments if we have any.
- `-> String` is the return type we expect.
- `let greeting =` is an immutable variable assignment.
- `String::from("Hello, World!")` creates a "Hello, World!" in the `String` type.
- `;` semi-colons denote the end of a line.
- `greeting` returns the value. We could also write `return greeting` and it would be the same thing.

In order to run `hello_world()`, we have to call it from the `main()`.

```rust,ignore
fn main() {
  let hello = hello_world();
  println!("{}", hello);
}
```

Running `cargo run` again builds and runs our code.

This prints "Hello, World!" to the screen the same way `println!("Hello, World!")` does, but is now ready to be tested.

### Writing our first test

Now that we have separated the domain code from any side effects, we write our first test.

```rust
{{#rustdoc_include ../examples/01_hello_world/hello_world/src/main.rs:test}}
```

- `#[cfg(test)]` is a macro that says "this block of code will be our tests". This is how `cargo` knows which code to run when you use `cargo test`.
- `mod tests` is the name of the block.
- `use super::*` pulls all the code above the test block into scope.
- `[#test]` is a macro we put on each function we want to count as a test.
-  `fn hello_world_test()` is our first test. Note: no return value is given, since tests aren't supposed to return anything; they're supposed to test code. 
- `let want = String::from("Hello, World!")` is our desired outcome for the test
- `let result = hello_world()` is the result we get back from the function we're testing.
- `assert_eq!(want, result)` asserts the `want` and `result` are the same value.


We can now run our tests with `cargo test`.

We should see the following output:

```bash,ignore 
Compiling hello_world v0.1.0 (/Users/glitch/projects/learn_rust_with_tests/examples/01_
hello_world/hello_world)
    Finished test [unoptimized + debuginfo] target(s) in 0.65s
     Running unittests src/main.rs (target/debug/deps/hello_world-9e795269315caeb3)

running 1 tests
test tests::greeting_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished i
n 0.00s
```
### Taking in a name

The purpose of tests is to define a desired outcome and write it in code. By first defining what outcome we wish to achieve, we can iterate on our code to match the test.

Our new requirement is to take in a name and print that out with "Hello". For example, the outcome should be "Hello, Rusty!"

Since this is Test-Driven Development, we write our test first. 

```rust
{{#rustdoc_include ../examples/01_hello_world/hello_world/src/main.rs:greeting_test}}
```
- `fn greeting_test()` is defined.
- `let want = String::from("Hello, Rusty!");` is the outcome we want.
- `let name = String::from("Rusty!")` is the name we pass an an argument to `greeting()`.
- `let result = greeting(name)` calls `greeting` with our `name` variable and saves the return value to `result`.
- `assert_eq!(want, result)` asserts `want` and `result` are the same.

If we run `cargo test` now, we will get an error:

```bash,ignore
$ cargo test
   Compiling hello_world v0.1.0 (/Users/rt/projects/learn_rust_with_tests/examples/01_
hello_world/hello_world)
error[E0425]: cannot find function `greeting` in this scope
  --> src/main.rs:46:22
   |
46 |         let result = greeting(name);
   |                      ^^^^^^^^ not found in this scope

For more information about this error, try `rustc --explain E0425`.
error: could not compile `hello_world` (bin "hello_world" test) due to 1 previous erro
r
```

That's because `greeting()` doesnt exist yet.

We can create a placeholder using the `unimplemented!()` macro. When `cargo test` is ran, it wont fail:

```rust
fn greeting() {
  unimplemented!()
}
```

Now we need to write the code to fulfill our test. 

```rust
{{#rustdoc_include ../examples/01_hello_world/hello_world/src/main.rs:greeting}}
```

- `fn greeting(name: String) -> String` creates function `greeting()` that takes a `String` called `name` as an argument. The return type is `String`.
- `let hello = String::from("Hello,");` creates the `String` "Hello,".
- `let greeting = format!("{hello} {name}!")` formats the value of `hello` and our `name` argument into a single `String` using the `format!()` macro.

We've written our function to fulfill the requirements we set in our test, so now we try running `cargo test`.

Running `cargo test` will give us "passed" on both tests.

```bash
$ cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests src/main.rs (target/debug/deps/hello_world-9e795269315caeb3)

running 2 tests
test tests::greeting_test ... ok
test tests::hello_world_test ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Starting in the next chapter, we will start writing our tests first, fulfilling them, and refactoring as necessary.
