fn get_message() -> &'static str {
    "Hello, world!"
}

fn main() {
    println!("{}", get_message());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_message() {
        assert_eq!(get_message(), "Hello, world!");
    }
}
