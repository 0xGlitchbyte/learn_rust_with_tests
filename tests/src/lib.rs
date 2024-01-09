pub fn hello_world() -> String {
    let greeting = String::from("Hello, World!");
    greeting
}

pub fn greeting(name: String) -> String {
    let result = String::from("Hello, ");
    let greet = format!("{result} {name}");
    greet
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        let want = String::from("Hello, World!");
        let result = hello_world();
        assert_eq!(result, want);
    }

    #[test]
    fn greeting_test() {
        let name = String::from("Rusty");
        let result = greeting(name);
        assert_eq!(result, name)
    }
}
