use elliptic_curves::helper::merkle_parent;
use num_traits::ToPrimitive;

#[derive(Debug)]
pub struct MerkleTree {
    total: u16,
    max_depth: f64,
    nodes: Vec<Vec<Option<Vec<u8>>>>,
    current_depth: f64,
    current_index: f64,
}

impl MerkleTree {
    pub fn new(total: u16) -> Self {
        let max_depth = (f64::log2(total as f64)).ceil() as u32;

        let mut nodes = Vec::new();
        for depth in 0..max_depth + 1 {
            let num_items =
                ((total as f64) / 2.0_f64.powi((max_depth - depth) as i32)).ceil() as usize;
            let level_hashes = vec![None; num_items];
            nodes.push(level_hashes);
        }

        Self {
            total,
            max_depth: max_depth as f64,
            nodes,
            current_depth: 0_f64,
            current_index: 0_f64,
        }
    }

    pub fn up(&mut self) {
        self.current_depth -= 1_f64;
        self.current_index /= 2_f64;
    }

    pub fn left(&mut self) {
        self.current_depth += 1_f64;
        self.current_index *= 2_f64;
    }

    pub fn right(&mut self) {
        self.current_depth += 1_f64;
        self.current_index = self.current_index * 2_f64 + 1_f64;
    }

    pub fn root(&self) -> &Option<Vec<u8>> {
        &self.nodes[0][0]
    }

    pub fn set_current_node(&mut self, value: Option<Vec<u8>>) {
        self.nodes[self.current_depth as usize][self.current_index as usize] = value;
    }

    pub fn get_current_node(&self) -> &Option<Vec<u8>> {
        let current_dep = self.current_depth.to_usize();
        let current_idx = self.current_index.to_usize();

        match (current_dep, current_idx) {
            (Some(dep), Some(idx)) => &self.nodes[dep][idx],
            _ => panic!("error to get current node"),
        }
    }

    pub fn get_left_node(&self) -> &Option<Vec<u8>> {
        let current_dep = (self.current_depth + 1_f64).to_usize();
        let current_idx = (self.current_index * 2_f64).to_usize();

        match (current_dep, current_idx) {
            (Some(dep), Some(idx)) => &self.nodes[dep][idx],
            _ => panic!("error to get left node"),
        }
    }

    pub fn get_right_node(&self) -> &Option<Vec<u8>> {
        let current_dep = (self.current_depth + 1_f64).to_usize();
        let current_idx = (self.current_index * 2_f64 + 1_f64).to_usize();

        match (current_dep, current_idx) {
            (Some(dep), Some(idx)) => &self.nodes[dep][idx],
            _ => panic!("error to get right node"),
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.current_depth == self.max_depth
    }

    pub fn right_exists(&self) -> bool {
        let current_dep = (self.current_depth + 1_f64).to_usize();
        let current_idx = (self.current_index * 2_f64 + 1_f64).to_usize();

        match (current_dep, current_idx) {
            (Some(dep), Some(idx)) => self.nodes[dep].len() > idx,
            _ => panic!("error to evaluate whether right node exists"),
        }
    }

    pub fn populate_tree(&mut self, flag_bits: &mut Vec<u8>, hashes: &mut Vec<Option<Vec<u8>>>) {
        while self.root().is_none() {
            if self.is_leaf() {
                flag_bits.remove(0);
                self.set_current_node(hashes.remove(0));
                self.up();
            } else {
                let left_hash = self.get_left_node();
                if left_hash.is_none() {
                    if flag_bits.remove(0) == 0 {
                        self.set_current_node(hashes.remove(0));
                        self.up();
                    } else {
                        self.left();
                    }
                } else if self.right_exists() {
                    let right_hash = self.get_right_node();
                    if right_hash.is_none() {
                        self.right();
                    } else {
                        let parent_hash =
                            merkle_parent(&mut left_hash.clone(), &mut right_hash.clone());
                        self.set_current_node(Some(parent_hash));
                        self.up();
                    }
                } else {
                    let parent_hash = merkle_parent(&mut left_hash.clone(), &mut left_hash.clone());
                    self.set_current_node(Some(parent_hash));
                    self.up();
                }
            }
        }

        if hashes.len() != 0 {
            panic!("hashes not all consumed {}", hashes.len());
        }

        for flag_bit in flag_bits {
            if *flag_bit != 0 {
                panic!("flag bit not all consumed");
            }
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
    use elliptic_curves::helper::{merkle_parent, merkle_parent_level};
    use hex::{FromHex, ToHex};

    use super::*;

    #[test]
    fn test_init() {
        let tree = MerkleTree::new(9);

        assert_eq!(tree.nodes[0].len(), 1);
        assert_eq!(tree.nodes[1].len(), 2);
        assert_eq!(tree.nodes[2].len(), 3);
        assert_eq!(tree.nodes[3].len(), 5);
        assert_eq!(tree.nodes[4].len(), 9);

        println!("{}", tree);
    }

    #[test]
    fn test_display() {
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
        let mut tree = MerkleTree::new(hex_hashes.len() as u16);

        let hashes: Vec<Option<Vec<u8>>> = hex_hashes
            .iter()
            .map(|h| Some(Vec::from_hex(h).unwrap()))
            .collect();

        tree.nodes[4] = hashes;
        tree.nodes[3] = merkle_parent_level(&mut tree.nodes[4]);
        tree.nodes[2] = merkle_parent_level(&mut tree.nodes[3]);
        tree.nodes[1] = merkle_parent_level(&mut tree.nodes[2]);
        tree.nodes[0] = merkle_parent_level(&mut tree.nodes[1]);

        println!("{}", tree);
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
        let mut tree = MerkleTree::new(hex_hashes.len() as u16);
        let hashes: Vec<Option<Vec<u8>>> = hex_hashes
            .iter()
            .map(|h| Some(Vec::from_hex(h).unwrap()))
            .collect();

        tree.nodes[4] = hashes;

        while tree.root().is_none() {
            if tree.is_leaf() {
                tree.up();
            } else {
                let left_hash = tree.get_left_node();
                let right_hash = tree.get_right_node();

                if left_hash.is_none() {
                    tree.left();
                } else if right_hash.is_none() {
                    tree.right();
                } else {
                    tree.set_current_node(Some(merkle_parent(
                        &mut left_hash.clone(),
                        &mut right_hash.clone(),
                    )));
                    tree.up();
                }
            }
        }

        println!("{}", tree);
    }

    #[test]
    fn test_populate_tree_1() {
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
        let mut tree = MerkleTree::new(hex_hashes.len() as u16);

        let mut hashes: Vec<Option<Vec<u8>>> = hex_hashes
            .iter()
            .map(|h| Some(Vec::from_hex(h).unwrap()))
            .collect();

        tree.populate_tree(&mut vec![1; 31], &mut hashes);

        let root = "597c4bafe3832b17cbbabe56f878f4fc2ad0f6a402cee7fa851a9cb205f87ed1";

        assert_eq!(
            tree.root().clone().unwrap().encode_hex::<String>(),
            root.to_string()
        );
    }
}
