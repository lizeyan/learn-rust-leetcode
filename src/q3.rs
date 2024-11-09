use std::cmp::{max, min};
use std::collections::HashMap;

trait Solution {
    fn length_of_longest_substring(s: String) -> i32;
}

struct SolutionImpl;
impl Solution for SolutionImpl {
    fn length_of_longest_substring(s: String) -> i32 {
        let mut last_occurrence = HashMap::new();
        let mut prev_longest = 0;
        let mut longest = 0;
        for (i, char) in s.chars().enumerate() {
            match last_occurrence.get(&char) {
                Some(_) => {
                    prev_longest = min(prev_longest + 1, i - last_occurrence[&char])
                }
                None => {
                    prev_longest += 1
                }
            }
            last_occurrence.insert(char, i);
            longest = max(longest, prev_longest);
        }
        longest as i32
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!(SolutionImpl::length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(SolutionImpl::length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(SolutionImpl::length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(SolutionImpl::length_of_longest_substring("dvdf".to_string()), 3);
        assert_eq!(SolutionImpl::length_of_longest_substring("".to_string()), 0);
    }
}