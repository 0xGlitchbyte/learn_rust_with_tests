fn main() {
    // ANCHOR: print;
    println!("Hello, world!");
    // ANCHOR: print;

    let name = String::from("Rusty");
    greeting(name);
}

fn hello_world() -> String {
    let greeting = String::from("Hello, World!");
    greeting
}

fn greeting(name: String) -> String {
    let hello = String::from("Hello, ");
    let greeting = format!("{hello}{name}!");
    greeting
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_test() {
        let want = String::from("Hello, World!");
        let result = hello_world();
        assert_eq!(result, want);
    }

    #[test]
    fn greeting_test() {
        let want = String::from("Hello, Rusty!");
        let name = String::from("Rusty");
        let result = greeting(name);
        assert_eq!(want, result);
    }
}
