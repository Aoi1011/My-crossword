use hex::ToHex;

#[derive(Debug)]
pub struct MerkleTree {
    total: u16,
    max_depth: u32,
    nodes: Vec<Vec<Option<String>>>,
    current_depth: u16,
    current_index: u16,
}

impl MerkleTree {
    pub fn new(total: u16) -> Self {
        let max_depth = (total as f64).log2().ceil() as u32;

        let mut nodes = Vec::new();
        for depth in 0..max_depth + 1 {
            let num_items =
                ((total as f64) / 2.0_f64.powi((max_depth - depth) as i32)).ceil() as usize;
            let level_hashes = vec![None; num_items];
            nodes.push(level_hashes);
        }

        Self {
            total,
            max_depth,
            nodes,
            current_depth: 0,
            current_index: 0,
        }
    }
}

impl std::fmt::Display for MerkleTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = Vec::new();

        for (depth, level) in self.nodes.iter().enumerate() {
            let mut items = Vec::new();

            for (idx, h) in level.iter().enumerate() {
                let short = match h {
                    Some(hash) => format!("{}...", hex::encode(&hash[..8])),
                    None => "None".to_string(),
                };

                if depth == self.current_depth as usize && idx == self.current_index as usize {
                    items.push(format!("*{}*", &short[..short.len() - 2]));
                } else {
                    items.push(short);
                }
            }
            result.push(items.join(", "));
        }

        write!(f, "{}", result.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let tree = MerkleTree::new(9);

        println!("{}", tree);

        assert_eq!(tree.nodes[0].len(), 1);
        assert_eq!(tree.nodes[1].len(), 2);
        assert_eq!(tree.nodes[2].len(), 3);
        assert_eq!(tree.nodes[3].len(), 5);
        assert_eq!(tree.nodes[4].len(), 9);
    }
}
