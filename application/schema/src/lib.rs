pub fn test() -> i32{
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        test();
        assert_eq!(1, 1);
    }
}