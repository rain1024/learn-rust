// Reference 
// https://exercism.org/tracks/rust/exercises/hello-world/

// &'static is a "lifetime specifier", something you'll learn more about later
pub fn hello() -> &'static str {
  "Good bye"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        assert_eq!("Hello, World!", hello());
    }
}