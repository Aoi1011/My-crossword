pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = 1;
        let b = 1;

        assert_eq!(2, add(a, b));
    }
}
