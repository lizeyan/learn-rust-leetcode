trait Solution {
    fn is_palindrome(x: i32) -> bool;
}

struct SolutionImpl;
impl Solution for SolutionImpl {
    fn is_palindrome(mut x: i32) -> bool {
        let s = x.to_string();
        s.chars().eq(s.chars().rev())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!(SolutionImpl::is_palindrome(121), true);
        assert_eq!(SolutionImpl::is_palindrome(-121), false);
        assert_eq!(SolutionImpl::is_palindrome(10), false);
    }
}