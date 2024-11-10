use std::cmp::min;

trait Solution {
    fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64;
}

struct SolutionImpl;


impl Solution for SolutionImpl {
    fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        fn find_k_smallest(arr1: &[i32], arr2: &[i32], k: usize) -> i32 {
            if arr1.is_empty() {
                return arr2[k - 1];
            } else if arr2.is_empty() { return arr1[k - 1] } else if k == 1 {
                return min(arr1[0], arr2[0]);
            }
            let idx1 = min(k / 2, arr1.len());
            let idx2 = min(k / 2, arr2.len());
            if arr1[idx1 - 1] < arr2[idx2 - 1] {
                find_k_smallest(&arr1[idx1..], arr2, k - idx1)
            } else {
                find_k_smallest(arr1, &arr2[idx2..], k - idx2)
            }
        }
        let total_len = nums1.len() + nums2.len();
        if total_len % 2 == 0 {
            let k = total_len / 2;
            let k1 = find_k_smallest(&nums1, &nums2, k);
            let k2 = find_k_smallest(&nums1, &nums2, k + 1);
            (k1 + k2) as f64 / 2.0
        } else {
            let k = total_len / 2 + 1;
            find_k_smallest(&nums1, &nums2, k) as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!(SolutionImpl::find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
        assert_eq!(SolutionImpl::find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    }
}