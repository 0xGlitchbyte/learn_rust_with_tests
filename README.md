# Learn Rust with Tests

⚠️ **WIP**: **This project is an ongoing work in progress. Links may be broken and content may be missing. If you find something wrong, please open an issue and we'll get them fixed asap.**

## Why

* Explore the Rust language by writing tests
* **Get a grounding with TDD**. Rust is a great language for learning TDD because it is a simple language to learn and testing is built-in
* Be confident that you'll be able to start writing robust, well-tested systems in Rust
* [Watch a video, or read about why unit testing and TDD is important](why.md)

## Table of contents

### Rust fundamentals

1. [Install Rust](src/install_rust.md) - Set up environment for productivity.
2. [Hello, world]() - Declaring variables, constants, if/else statements, loops, functions, write your first Rust program and write your first test. Sub-test syntax and closures.
3. [Primitives]() - Further Explore function declaration syntax and learn new ways to improve the documentation of your code.
4. [Control Flow]() - Declaring if/else, loops, match, to control the flow of your program.
4. [Vectors]() - Learn about one of the most used data structures.
5. [Ownership and Borrow Checker]() - Learn about ownership, borrowing, and lifetimes.
6. [Structs]() - Learn about the three struct types: a classic C struct, a tuple struct, and a unit struct..
7. [Enums]() - Learn about how to define and use `enums`.
8. [Strings]() - Learn about the two string types, a string slice (&str) and an owned string (String).
9. [Modules]() - Learn how to use the module system.
10. [Mocking]() - Learn how to use the `Mockall` library to mocking.
11. [Hashmaps]() - Learn how to define and use `hashmaps`.
12. [Options]() - Learn how every Option is either Some and contains a value, or None, and does not.
13. [Error Handling]() - Learn how to properly handle errors.
14. [Generics]() - Learn how `generics` generalize types and functionalities to broader cases, reducing code duplication.
15. [Traits]() - Learn how `traits` define shared behaviors.
16. [Lifetimes]() - Learn how lifetimes tell the compiler how to check whether references live long enough to be valid in any given situation
17. [Iterators]() - Learn about the different ways to iterate.
18. [Smart Pointers]() - Learn about how smart pointers are variables that contain an address in memory and reference some other data.
19. [Threads]() - Learn about how programs can have independent parts that run simultaneously.
20. [Macros]() - Learn how to use and create `macros`.
21. [Conversions]() - Learn about the many ways to convert a value of a given type into another type.

### Build an application

Now that you have hopefully digested the _Rust Fundamentals_ section you have a solid grounding of a majority of Rust's language features and how to do TDD.

This next section will involve building an application.

Each chapter will iterate on the previous one, expanding the application's functionality as our product owner dictates.

New concepts will be introduced to help facilitate writing great code but most of the new material will be learning what can be accomplished from Rust's standard library.

By the end of this, you should have a strong grasp as to how to iteratively write an application in Rust, backed by tests.

* [HTTP server]() - We will create an application which listens to HTTP requests and responds to them.
* [JSON, routing and embedding]() - We will make our endpoints return JSON and explore how to do routing.
* [IO and sorting]() - We will persist and read our data from disk and we'll cover sorting data.
* [Command line & project structure]() - Support multiple applications from one code base and read input from command line.
* [Time]() - using the `time` package to schedule activities.
* [WebSockets]() - learn how to write and test a server that uses WebSockets.

### Testing fundamentals

Covering other subjects around testing.

* [Introduction to acceptance tests]() - Learn how to write acceptance tests for your code, with a real-world example for gracefully shutting down a HTTP server
* [Scaling acceptance tests]() - Learn techniques to manage the complexity of writing acceptance tests for non-trivial systems.
* [Working without mocks, stubs and spies]() - Learn about how to use fakes and contracts to create more realistic and maintainable tests.
* [Refactoring Checklist]() - Some discussion on what refactoring is, and some basic tips on how to do it.

### Questions and answers

I often run in to questions on the internets like

> How do I test my amazing function that does x, y and z

If you have such a question raise it as an issue on github and I'll try and find time to write a short chapter to tackle the issue. I feel like content like this is valuable as it is tackling people's _real_ questions around testing.

* [OS exec]() - An example of how we can reach out to the OS to execute commands to fetch data and keep our business logic testable/
* [Error types]() - Example of creating your own error types to improve your tests and make your code easier to work with.
* [Revisiting HTTP Handlers]() - Testing HTTP handlers seems to be the bane of many a developer's existence. This chapter explores the issues around designing handlers correctly.

### Meta / Discussion

* [Why unit tests and how to make them work for you]() - Watch a video, or read about why unit testing and TDD is important
* [Anti-patterns]() - A short chapter on TDD and unit testing anti-patterns

## Contributing

* _This project is work in progress_ If you would like to contribute, please do get in touch.
* Read [contributing.md](Contributing.md) for guidelines
* Any ideas? Create an issue

## Who this is for

* People who are interested in picking up Rust.
* People who already know some Rust, but want to explore testing with TDD.

## What you'll need

* A computer!
* [Installed Rust](https://rust-lang.org/)
* A text editor
* Some experience with programming. Understanding of concepts like `if`, variables, functions etc.
* Comfortable using the terminal

## Support me

I am proud to offer this resource for free, but if you wish to give some appreciation:

- [Tweet me @0xglitchbyte](https://twitter.com/0xglitchbyte)

## Feedback

* Add issues/submit PRs [here](https://github.com/0xGlitchbyte/learn_Rust_with_tests) or [tweet me @0xglitchbyte](https://twitter.com/0xglitchbyte)

[MIT license](LICENSE.md)

