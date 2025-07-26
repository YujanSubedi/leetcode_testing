struct UsizeHashSet {
    hash_bitboard: Vec<u128>,
}

impl UsizeHashSet {
    fn new(size_of_hash_map: usize) -> Self {
        Self {
            hash_bitboard: vec![0; size_of_hash_map],
        }
    }

    fn insert(&mut self, value: usize) {
        let index: usize = value >> 7;
        let bit_position: usize = value & 127;
        let full_bitmask: u128 = 1u128 << bit_position;
        self.hash_bitboard[index] |= full_bitmask;
    }

    fn remove(&mut self, value: usize) {
        let index: usize = value >> 7;
        let bit_position: usize = value & 127;
        let full_bitmask: u128 = 1u128 << bit_position;
        self.hash_bitboard[index] &= !full_bitmask;
    }

    fn contains(&self, value: usize) -> bool {
        let index: usize = value >> 7;
        let bit_position: usize = value & 127;
        let full_bitmask: u128 = 1u128 << bit_position;
        (self.hash_bitboard[index] & full_bitmask) != 0
    }

    fn get_values(&self) -> Vec<usize> {
        let mut all_values: Vec<usize> = Vec::with_capacity(self.hash_bitboard.len() * 128);
        for (index, val) in self.hash_bitboard.iter().enumerate() {
            for i in 0..128usize {
                if (1u128 << i) & val != 0 {
                    all_values.push(index * 128 + i);
                }
            }
        }
        all_values
    }
}

pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
    let hashset_size = graph.len() / 128 + 1;

    let mut safe_nodes_hashset: UsizeHashSet = UsizeHashSet::new(hashset_size);
    let mut unsafe_nodes_hashset: UsizeHashSet = UsizeHashSet::new(hashset_size);

    for (node, paths) in graph.iter().enumerate() {
        if paths.is_empty() {
            safe_nodes_hashset.insert(node);
        } else {
            unsafe_nodes_hashset.insert(node);
        }
    }

    let mut new_added = true;
    while new_added {
        new_added = false;
        for node in unsafe_nodes_hashset.get_values() {
            let mut all_safe = true;
            for path in &graph[node] {
                if unsafe_nodes_hashset.contains(*path as usize) {
                    all_safe = false;
                    break;
                }
            }
            if all_safe {
                safe_nodes_hashset.insert(node);
                unsafe_nodes_hashset.remove(node);
                new_added = true;
            }
        }
    }

    let safe_nodes_vec = safe_nodes_hashset.get_values();
    let mut res: Vec<i32> = Vec::with_capacity(safe_nodes_vec.len());
    for node in safe_nodes_vec {
        res.push(node as i32);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let graph = vec![
            vec![1, 2],
            vec![2, 3],
            vec![5],
            vec![0],
            vec![5],
            vec![],
            vec![],
        ];
        let ouptut_val = vec![2, 4, 5, 6];
        assert_eq!(eventual_safe_nodes(graph), ouptut_val);
    }

    #[test]
    fn test_02() {
        let graph = vec![vec![1, 2, 3, 4], vec![1, 2], vec![3, 4], vec![0, 4], vec![]];
        let ouptut_val = vec![4];
        assert_eq!(eventual_safe_nodes(graph), ouptut_val);
    }
}
