trait Solution {
    fn roman_to_int(s: String) -> i32;
}

struct SolutionImpl;
impl Solution for SolutionImpl {
    fn roman_to_int(s: String) -> i32 {
        let mut ret = 0;
        let chars = s.chars().collect::<Vec<char>>();
        std::iter::once(&' ')
            .chain(chars.iter())
            .zip(chars.iter())
            .for_each(|(prev, cur)| match (prev, cur) {
                ('C', 'M') => ret += 800,
                ('C', 'D') => ret += 300,
                ('X', 'C') => ret += 80,
                ('X', 'L') => ret += 30,
                ('I', 'X') => ret += 8,
                ('I', 'V') => ret += 3,
                (_, 'M') => ret += 1000,
                (_, 'D') => ret += 500,
                (_, 'C') => ret += 100,
                (_, 'L') => ret += 50,
                (_, 'X') => ret += 10,
                (_, 'V') => ret += 5,
                (_, 'I') => ret += 1,
                _ => (unreachable!()),
            });
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!(SolutionImpl::roman_to_int("III".to_string()), 3);
        assert_eq!(SolutionImpl::roman_to_int("IV".to_string()), 4);
        assert_eq!(SolutionImpl::roman_to_int("IX".to_string()), 9);
        assert_eq!(SolutionImpl::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(SolutionImpl::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
