pub struct Solution {}

impl Solution {
    // 3. 无重复字符的最长子串
    #[allow(dead_code)]
    pub fn length_of_longest_substring(s: String) -> i32 {
        let (mut start, mut max_length) = (0, 0);
        let mut nums_index = std::collections::HashMap::new();
        s.chars().enumerate().for_each(|(end, c)| {
            match nums_index.get(&c) {
                Some(index) => {
                    if start.le(index) {
                        start = index + 1
                    } else {
                        max_length = max_length.max(end - start + 1)
                    }
                }
                None => max_length = max_length.max(end - start + 1),
            }
            nums_index.insert(c, end);
        });
        max_length as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 3. 无重复字符的最长子串
    #[test]
    fn length_of_longest_substring() {
        let s = "abcabcbb".to_string();
        let res = Solution::length_of_longest_substring(s);
        assert_eq!(res, 3);
    }
}
