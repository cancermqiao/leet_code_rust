pub struct Solution;

impl Solution {
    /// 242. 有效的字母异位词
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut count = [0; 26];
        let a = 'a' as usize;
        s.chars().zip(t.chars()).for_each(|(s_char, t_char)| {
            count[s_char as usize - a] += 1;
            count[t_char as usize - a] -= 1;
        });
        count.iter().all(|&x| x == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 242. 有效的字母异位词
    #[test]
    fn is_anagram() {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();
        let res = Solution::is_anagram(s, t);
        assert!(res);
    }
}
