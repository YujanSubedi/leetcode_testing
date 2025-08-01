pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let len1 = nums1.len();
    let len2 = nums2.len();
    let total_len = len1 + len2;
    let mut half_len = (total_len - 1) >> 1;

    let mut indx1: usize = 0;
    let mut indx2: usize = 0;

    while half_len > 0 {
        match (indx1 < len1, indx2 < len2) {
            (true, true) => {
                if nums1[indx1] < nums2[indx2] {
                    indx1 += 1;
                } else {
                    indx2 += 1;
                }
            }
            (true, false) => {
                indx1 += half_len;
                half_len = 1;
            }
            (false, true) => {
                indx2 += half_len;
                half_len = 1;
            }
            (false, false) => {}
        }
        half_len -= 1;
    }

    let left_val = match (indx1 < len1, indx2 < len2) {
        (true, true) => {
            if nums1[indx1] < nums2[indx2] {
                indx1 += 1;
                nums1[indx1 - 1] as f64
            } else {
                indx2 += 1;
                nums2[indx2 - 1] as f64
            }
        }
        (true, false) => {
            indx1 += 1;
            nums1[indx1 - 1] as f64
        }
        (false, true) => {
            indx2 += 1;
            nums2[indx2 - 1] as f64
        }
        (false, false) => 0.0,
    };

    if total_len & 1 == 1 {
        return left_val;
    }

    let right_val = match (indx1 < len1, indx2 < len2) {
        (true, true) => {
            if nums1[indx1] < nums2[indx2] {
                nums1[indx1] as f64
            } else {
                nums2[indx2] as f64
            }
        }
        (true, false) => nums1[indx1] as f64,
        (false, true) => nums2[indx2] as f64,
        (false, false) => 0.0,
    };

    (left_val + right_val) / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        let output_val = 2.00000;
        assert_eq!(find_median_sorted_arrays(nums1, nums2), output_val);
    }

    #[test]
    fn test_02() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        let output_val = 2.50000;

        assert_eq!(find_median_sorted_arrays(nums1, nums2), output_val);
    }

    #[test]
    fn test_03() {
        let nums1 = vec![];
        let nums2 = vec![1];
        let output_val = 1.0;

        assert_eq!(find_median_sorted_arrays(nums1, nums2), output_val);
    }

    #[test]
    fn test_04() {
        let nums1 = vec![1, 2, 3, 4, 5];
        let nums2 = vec![6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
        let output_val = 1.0;

        assert_eq!(find_median_sorted_arrays(nums1, nums2), output_val);
    }
}
