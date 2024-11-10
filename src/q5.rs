use std::cmp::min;

trait Solution {
    fn longest_palindrome(s: String) -> String;
}

struct SolutionImpl;

impl Solution for SolutionImpl {
    fn longest_palindrome(s: String) -> String {
        let mut chars = Vec::with_capacity(s.len() * 2 + 1);
        chars.push('#');
        for char in s.chars() {
            chars.push(char);
            chars.push('#');
        }
        let mut rightmost = 0;
        let mut rightmost_center = 0;
        let mut radius = Vec::with_capacity(chars.len());
        for i in 0..chars.len() {
            let mut r = {
                if i < rightmost {
                    min(radius[2 * rightmost_center - i], (rightmost - i))
                } else { 0 }
            };
            while i + r < chars.len() && i >= r && chars[i + r] == chars[i - r] {
                r += 1
            };
            radius.push(r - 1);
            if i + r > rightmost {
                rightmost = i + r;
                rightmost_center = i;
            }
        }

        let c = radius.iter().enumerate().max_by_key(|(_, &r)| r).map(|(i, &r)| i)
            .unwrap();
        let start = (c - radius[c]) / 2;
        let end = (c + radius[c]) / 2;
        s[start..end].chars().collect::<String>()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn cases() {
        assert_eq!(SolutionImpl::longest_palindrome("babad".to_string()), "aba");
        assert_eq!(SolutionImpl::longest_palindrome("cbbd".to_string()), "bb");
        assert_eq!(SolutionImpl::longest_palindrome("a".to_string()), "a");
        assert_eq!(SolutionImpl::longest_palindrome("ac".to_string()), "c");
        assert_eq!(SolutionImpl::longest_palindrome("".to_string()), "");
    }
}