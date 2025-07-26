fn min_swaps_helper(first_vec: Vec<i32>, second_vec: Vec<i32>) -> i32 {
    let l1 = first_vec.len();
    let l2 = second_vec.len();
    let mut i1: usize = 0;
    let mut i2: usize = 0;

    let mut f_val = first_vec[0];
    let mut s_val = second_vec[0];
    let mut no_swap = true;
    let mut no_of_oprations = 0;

    println!();
    println!("{first_vec:?}, {second_vec:?}");
    while i2 < l1 || i2 < l2 {
        println!("{f_val}, {s_val}, {no_swap}");
        if f_val == s_val {
            let shift_oprations: i64 = ((f_val as i64) * (f_val as i64 - 1)) >> 1;
            no_of_oprations += shift_oprations as i32;

            i1 += 1;
            i2 += 1;
            f_val = if i1 < l1 { first_vec[i1] } else { 0 };
            s_val = if i2 < l2 { second_vec[i2] } else { 0 };
        } else if no_swap {
            if f_val > s_val {
                let a = (f_val - s_val) as i64;
                let b = (f_val - 1) as i64;
                let shift_oprations: i64 = ((a + b) * (b - a + 1)) >> 1;
                no_of_oprations += shift_oprations as i32;

                i1 += 1;
                i2 += 1;
                f_val -= s_val;
                f_val += if i1 < l1 { first_vec[i1] } else { 0 };
                s_val = if i2 < l2 { second_vec[i2] } else { 0 };
            } else {
                no_swap = false;
                let shift_oprations: i64 = ((f_val as i64) * (f_val as i64 - 1)) >> 1;
                no_of_oprations += shift_oprations as i32;

                i1 += 1;
                s_val -= f_val - 1;
                f_val = if i1 < l1 { first_vec[i1] } else { 0 };
            }
        } else if s_val > f_val {
            let a = (s_val - f_val) as i64;
            let b = (s_val - 1) as i64;
            let shift_oprations: i64 = ((a + b) * (b - a + 1)) >> 1;
            no_of_oprations += shift_oprations as i32;

            i1 += 1;
            i2 += 1;
            s_val -= f_val;
            f_val = if i1 < l1 { first_vec[i1] } else { 0 };
            s_val += if i2 < l2 { second_vec[i2] } else { 0 };
        } else {
            no_swap = true;
            let shift_oprations: i64 = ((s_val as i64) * (s_val as i64 - 1)) >> 1;
            no_of_oprations += shift_oprations as i32;

            i2 += 1;
            f_val -= s_val - 1;
            s_val = if i2 < l2 { second_vec[i2] } else { 0 };
        }
        println!("{no_of_oprations}");
    }

    println!("{no_of_oprations}");
    no_of_oprations
}

pub fn min_swaps(nums: Vec<i32>) -> i32 {
    println!("{nums:?}");
    let (odd_vec, even_vec, odd_count, even_count) = {
        let len_nums = nums.len();
        if len_nums == 1 {
            return 0;
        }

        let mut odd_count: i32 = 0;
        let mut even_count: i32 = 0;
        let mut odd_vecs: Vec<i32> = Vec::with_capacity(len_nums);
        let mut even_vecs: Vec<i32> = Vec::with_capacity(len_nums);
        let mut index: usize = 0;
        let mut count: i32;

        while index < len_nums {
            count = 0;
            while index < len_nums && nums[index] & 1 == 0 {
                even_count += 1;
                count += 1;
                index += 1;
            }
            if count > 0 {
                even_vecs.push(count);
            }

            count = 0;
            while index < len_nums && nums[index] & 1 != 0 {
                odd_count += 1;
                count += 1;
                index += 1;
            }
            if count > 0 {
                odd_vecs.push(count);
            }
        }
        if (odd_count - even_count).abs() > 1 {
            return -1;
        }

        (odd_vecs, even_vecs, odd_count, even_count)
    };

    println!("{odd_vec:?}, {even_vec:?}");

    let odd_first_res = {
        if even_count > odd_count {
            i32::MAX
        } else if nums[0] & 1 == 0 {
            let mut odd_vec_clone = odd_vec.clone();
            odd_vec_clone[0] -= 1;
            even_vec[0] + min_swaps_helper(even_vec.clone(), odd_vec_clone)
        } else {
            min_swaps_helper(odd_vec.clone(), even_vec.clone())
        }
    };

    let even_first_res = {
        if odd_count > even_count {
            i32::MAX
        } else if nums[0] & 1 == 1 {
            let mut even_vec_clone = even_vec.clone();
            even_vec_clone[0] -= 1;
            odd_vec[0] + min_swaps_helper(odd_vec, even_vec_clone)
        } else {
            min_swaps_helper(even_vec, odd_vec)
        }
    };

    println!();
    println!("{odd_first_res}, {even_first_res}");
    if odd_first_res < even_first_res {
        odd_first_res
    } else {
        even_first_res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let nums = vec![2, 4, 6, 5, 7];
        let output_val = 3;
        assert_eq!(min_swaps(nums), output_val);
    }

    #[test]
    fn test_02() {
        let nums = vec![2, 4, 5, 7];
        let output_val = 1;
        assert_eq!(min_swaps(nums), output_val);
    }

    #[test]
    fn test_03() {
        let nums = vec![1, 2, 3];
        let output_val = 0;
        assert_eq!(min_swaps(nums), output_val);
    }

    #[test]
    fn test_04() {
        let nums = vec![4, 5, 6, 8];
        let output_val = -1;
        assert_eq!(min_swaps(nums), output_val);
    }

    #[test]
    fn test_05() {
        let nums = vec![67, 172, 310, 333];
        let output_val = 1;
        assert_eq!(min_swaps(nums), output_val);
    }

    #[test]
    fn test_06() {
        let nums = vec![139, 494, 546, 268, 97, 523];
        let output_val = 2;
        assert_eq!(min_swaps(nums), output_val);
    }

    #[test]
    fn test_07() {
        let nums = vec![1, 1, 2, 2, 2, 1, 1, 1, 2, 2];
        let output_val = 3;
        assert_eq!(min_swaps(nums), output_val);
    }

    #[test]
    fn test_08() {
        let nums = vec![
            128106918, 766862377, 272670667, 471997775, 525240106, 677329976, 445577800,
        ];
        let output_val = 3;
        assert_eq!(min_swaps(nums), output_val);
    }
}
