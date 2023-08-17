pub struct MerkleTree {
    total: u16,
    max_depth: u32,
    nodes: Vec<u16>,
    current_depth: u16,
    current_index: u16,
}

impl MerkleTree {
    pub fn new(total: u16) -> Self {
        let max_depth = (total as f64).log2().ceil() as u32;
        Self {
            total,
            max_depth,
            nodes: (),
            current_depth: (),
            current_index: (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
