use std::collections::HashMap;


trait Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>;
}

struct SolutionImpl;

impl Solution for SolutionImpl {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let map: HashMap<i32, usize> = nums.iter().enumerate().map(|(i, &val)| (val, i)).collect();

        for (i, item) in nums.iter().enumerate() {
            match map.get(&(target - item)) {
                Some(&j) => {
                    if j != i {
                        return vec![i.try_into().unwrap(), j.try_into().unwrap()];
                    }
                }
                _ => {}
            }
        }
        return vec![0, 1];
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!(SolutionImpl::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(SolutionImpl::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(SolutionImpl::two_sum(vec![3,3], 6), vec![0, 1]);
    }
}