pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise1() {
        let prime = 223;

        let vec: Vec<(i32, i32)> = vec![(192, 105), (17, 56), (200, 119), (1, 193), (42, 99)];

        for point in vec {
            let x = point.0.pow(2) % prime;
            let y = (point.1.pow(3) + 7) % prime;

            println!("{} Point ({}, {})", x == y,  point.0, point.1);
        }
    }
}
