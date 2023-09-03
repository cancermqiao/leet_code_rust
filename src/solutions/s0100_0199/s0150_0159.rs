pub struct Solution {}

impl Solution {
    /// 151. 反转字符串中的单词
    #[allow(dead_code)]
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 151. 反转字符串中的单词
    #[test]
    fn reverse_words() {
        let s = "  hello world  ".to_string();
        let res = Solution::reverse_words(s);
        assert_eq!(res, "world hello".to_string());
    }
}
