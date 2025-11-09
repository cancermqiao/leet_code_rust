pub struct Solution;

impl Solution {
    /// 318. 最大单词长度乘积
    fn word_to_mask(word: &str) -> u32 {
        word.as_bytes()
            .iter()
            .fold(0, |acc, e| acc | (1 << (e - b'a')))
    }

    pub fn max_product(words: Vec<String>) -> i32 {
        let mask_lens: Vec<(u32, usize)> = words
            .iter()
            .map(|word| (Self::word_to_mask(word), word.len()))
            .collect();
        let mut res = 0;
        mask_lens.iter().for_each(|(m1, l1)| {
            mask_lens.iter().for_each(|(m2, l2)| {
                if m1 & m2 == 0 {
                    res = res.max(l1 * l2);
                }
            })
        });
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 318. 最大单词长度乘积
    #[test]
    fn max_product() {
        let words = vec![
            "abcw".to_string(),
            "baz".to_string(),
            "foo".to_string(),
            "bar".to_string(),
            "xtfn".to_string(),
            "abcdef".to_string(),
        ];
        let res = Solution::max_product(words);
        assert_eq!(res, 16);
    }
}
