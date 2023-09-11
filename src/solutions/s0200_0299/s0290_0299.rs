pub struct Solution {}

impl Solution {
    /// 290. 单词规律
    #[allow(dead_code)]
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let words = s.split(' ').collect::<Vec<&str>>();
        if pattern.len() != words.len() {
            return false;
        }
        let mut c2w = std::collections::HashMap::new();
        let mut w2c = std::collections::HashMap::new();
        for i in 0..pattern.len() {
            let c = pattern.chars().nth(i).unwrap();
            let word = words[i];
            if let Some(&v) = c2w.get(&c) {
                if v != word {
                    return false;
                }
            } else {
                c2w.insert(c, word);
            }
            if let Some(&v) = w2c.get(word) {
                if v != c {
                    return false;
                }
            } else {
                w2c.insert(word, c);
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 290. 单词规律
    #[test]
    fn word_pattern() {
        let pattern = "abba".to_string();
        let s = "dog cat cat dog".to_string();
        let res = Solution::word_pattern(pattern, s);
        assert_eq!(res, true);
    }
}