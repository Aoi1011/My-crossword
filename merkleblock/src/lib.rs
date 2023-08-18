use hex::ToHex;

#[derive(Debug)]
pub struct MerkleTree {
    total: u16,
    max_depth: u32,
    nodes: Vec<Option<Vec<u8>>>,
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
    use hex::FromHex;

    use super::*;

    #[test]
    fn test_init() {
        let tree = MerkleTree::new(9);

        assert_eq!(tree.nodes[0].len(), 1);
        assert_eq!(tree.nodes[1].len(), 2);
        assert_eq!(tree.nodes[2].len(), 3);
        assert_eq!(tree.nodes[3].len(), 5);
        assert_eq!(tree.nodes[4].len(), 9);
    }

    #[test]
    fn test_print() {
        let hex_hashes = vec![
            "9745f7173ef14ee4155722d1cbf13304339fd00d900b759c6f9d58579b5765fb",
            "5573c8ede34936c29cdfdfe743f7f5fdfbd4f54ba0705259e62f39917065cb9b",
            "82a02ecbb6623b4274dfcab82b336dc017a27136e08521091e443e62582e8f05",
            "507ccae5ed9b340363a0e6d765af148be9cb1c8766ccc922f83e4ae681658308",
            "a7a4aec28e7162e1e9ef33dfa30f0bc0526e6cf4b11a576f6c5de58593898330",
            "bb6267664bd833fd9fc82582853ab144fece26b7a8a5bf328f8a059445b59add",
            "ea6d7ac1ee77fbacee58fc717b990c4fcccf1b19af43103c090f601677fd8836",
            "457743861de496c429912558a106b810b0507975a49773228aa788df40730d41",
            "7688029288efc9e9a0011c960a6ed9e5466581abf3e3a6c26ee317461add619a",
            "b1ae7f15836cb2286cdd4e2c37bf9bb7da0a2846d06867a429f654b2e7f383c9",
            "9b74f89fa3f93e71ff2c241f32945d877281a6a50a6bf94adac002980aafe5ab",
            "b3a92b5b255019bdaf754875633c2de9fec2ab03e6b8ce669d07cb5b18804638",
            "b5c0b915312b9bdaedd2b86aa2d0f8feffc73a2d37668fd9010179261e25e263",
            "c9d52c5cb1e557b92c84c52e7c4bfbce859408bedffc8a5560fd6e35e10b8800",
            "c555bc5fc3bc096df0a0c9532f07640bfb76bfe4fc1ace214b8b228a1297a4c2",
            "f9dbfafc3af3400954975da24eb325e326960a25b87fffe23eef3e7ed2fb610e",
        ];
        let tree = MerkleTree::new(hex_hashes.len() as u16);

        let hashes: Vec<Option<Vec<u8>>> = hex_hashes
            .iter()
            .map(|h| Some(Vec::from_hex(h).unwrap()))
            .collect();

        tree.nodes[4] = hashes;
    }
}
