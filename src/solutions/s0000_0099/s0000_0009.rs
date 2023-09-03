use std::ops::Div;

pub struct Solution {}

impl Solution {
    /// 3. 无重复字符的最长子串
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

    ///6. N字形变换
    #[allow(dead_code)] 
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows < 2 {
            return s;
        }
        let mut res = vec!["".to_string(); num_rows as usize];
        let (mut i, mut direction) = (0, -1);
        for c in s.chars() {
            res.get_mut(i as usize).unwrap().push(c);
            if i == 0 || i == num_rows as i32 - 1 {
                direction = -direction;
            }
            i += direction;
        }
        res.join("")
    }
    
    /// 9. 回文数
    #[allow(dead_code)]
    pub fn is_palindrome(x: i32) -> bool {
        // 0
        if x == 0 {
            return true;
        }
        // 负数和个位为0
        if x < 0 || x % 10 == 0 {
            return false;
        }
        let mut reverted_number = 0;
        let mut x = x;
        while x > reverted_number {
            let (quotient, remaind) = (x / 10, x % 10);
            reverted_number = reverted_number * 10 + remaind;
            x = quotient;
        }
        x == reverted_number || x == reverted_number / 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 3. 无重复字符的最长子串
    #[test]
    fn length_of_longest_substring() {
        let s = "abcabcbb".to_string();
        let res = Solution::length_of_longest_substring(s);
        assert_eq!(res, 3);
    }

    ///6. N字形变换
    #[test] 
    fn convert() {
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 3;
        let res = Solution::convert(s, num_rows);
        assert_eq!(res, "PAHNAPLSIIGYIR".to_string())
    }

    /// 9. 回文数
    #[test]
    fn is_palindrome() {
        let x = 121;
        let res = Solution::is_palindrome(x);
        assert_eq!(res, true);
    }
}
