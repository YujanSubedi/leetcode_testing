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

    fn contains_range(&self, lower_value: usize, higher_value: usize) -> bool {
        let lower_index: usize = lower_value >> 7;
        let higher_index: usize = higher_value >> 7;
        let lower_bit_position: usize = lower_value & 127;
        let higher_bit_position: usize = higher_value & 127;

        let lower_mask: u128 = 1u128 << lower_bit_position;
        let higher_mask: u128 = 1u128 << higher_bit_position;
        let full_lower_mask: u128 = lower_mask | (!(lower_mask | (lower_mask - 1)));
        let full_higher_mask: u128 = higher_mask | (higher_mask - 1);

        if lower_index == higher_index {
            if full_lower_mask & full_higher_mask & self.hash_bitboard[higher_index] != 0 {
                return true;
            }
            return false;
        }

        let full_set_mask: u128 = u128::MAX;
        if self.hash_bitboard[lower_index] & full_lower_mask != 0
            || self.hash_bitboard[higher_index] & full_higher_mask != 0
        {
            return true;
        }
        for i in lower_index + 1..higher_index {
            if self.hash_bitboard[i] & full_set_mask != 0 {
                return true;
            }
        }

        false
    }
}

pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, index_diff: i32, value_diff: i32) -> bool {
    let mut custom_usize_hashmap: UsizeHashSet = UsizeHashSet::new(15_625_001);

    let length: usize = nums.len();
    let ind_dif_usize: usize = index_diff as usize;
    let val_dif_usize: usize = value_diff as usize;

    let mut value: usize;
    let mut j: usize = 0;
    while j <= ind_dif_usize && j < length {
        value = (nums[j] + 1000000000) as usize;
        if custom_usize_hashmap.contains(value) {
            return true;
        }
        custom_usize_hashmap.insert(value);
        j += 1;
    }

    let mut lower_value: usize;
    let mut higher_value: usize;
    for i in 0..length {
        value = (nums[i] + 1000000000) as usize;
        custom_usize_hashmap.remove(value);
        if val_dif_usize > value {
            lower_value = 0;
        } else {
            lower_value = value - val_dif_usize;
        }
        if value + val_dif_usize > 2000000000 {
            higher_value = 2000000000;
        } else {
            higher_value = value + val_dif_usize;
        }
        if custom_usize_hashmap.contains_range(lower_value, higher_value) {
            return true;
        }
        if j < length {
            value = (nums[j] + 1000000000) as usize;
            if custom_usize_hashmap.contains(value) {
                return true;
            }
            custom_usize_hashmap.insert(value);
            j += 1;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let nums_vec: Vec<i32> = vec![-3, 3];
        let indexdiff: i32 = 1;
        let valuediff: i32 = 0;

        assert_eq!(
            contains_nearby_almost_duplicate(nums_vec, indexdiff, valuediff),
            false
        );
    }

    #[test]
    fn test_02() {
        let nums_vec: Vec<i32> = vec![1, 5, 9, 1, 5, 9];
        let indexdiff: i32 = 1;
        let valuediff: i32 = 0;

        assert_eq!(
            contains_nearby_almost_duplicate(nums_vec, indexdiff, valuediff),
            false
        );
    }

    #[test]
    fn test_03() {
        let nums_vec: Vec<i32> = vec![1, 5, 9, 1, 5, 9, 1000000000, -1000000000, 123];
        let indexdiff: i32 = 1;
        let valuediff: i32 = 0;

        assert_eq!(
            contains_nearby_almost_duplicate(nums_vec, indexdiff, valuediff),
            false
        );
    }

    #[test]
    fn test_04() {
        let nums_vec: Vec<i32> = vec![1, 2, 3, 1];
        let indexdiff: i32 = 3;
        let valuediff: i32 = 0;

        assert_eq!(
            contains_nearby_almost_duplicate(nums_vec, indexdiff, valuediff),
            true
        );
    }

    #[test]
    fn test_05() {
        let nums_vec: Vec<i32> = vec![1, 5, 9, 1, 5, 9, 1000000000, -1000000000, 123];
        let indexdiff: i32 = 1;
        let valuediff: i32 = 4;

        assert_eq!(
            contains_nearby_almost_duplicate(nums_vec, indexdiff, valuediff),
            true
        );
    }
}
