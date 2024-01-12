fn main() {
    // ANCHOR: print
    println!("Hello, world!");
    // ANCHOR_END: print

    let hello = hello_world();
    println!("{}", hello);

    let name = String::from("Rusty");
    greeting(name);
}

// ANCHOR: hello_world
fn hello_world() -> String {
    let greeting = String::from("Hello, World!");
    greeting
}
// ANCHOR_END: hello_world

// ANCHOR: greeting
fn greeting(name: String) -> String {
    let hello = String::from("Hello, ");
    let greeting = format!("{hello}{name}!");
    greeting
}
// ANCHOR_END: greeting

// ANCHOR: test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_test() {
        let want = String::from("Hello, World!");
        let result = hello_world();
        assert_eq!(want, result);
    }
    // ANCHOR_END: test

    // ANCHOR: greeting_test
    #[test]
    fn greeting_test() {
        let want = String::from("Hello, Rusty!");
        let name = String::from("Rusty");
        let result = greeting(name);
        assert_eq!(want, result);
    }
    // ANCHOR_END: greeting_test
}
