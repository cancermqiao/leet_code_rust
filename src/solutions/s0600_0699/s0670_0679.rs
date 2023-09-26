pub struct Solution {}

impl Solution {
    /// 678. 有效的括号字符串
    #[allow(dead_code)]
    pub fn check_valid_string(s: String) -> bool {
        let (mut stack1, mut stack2) = (Vec::new(), Vec::new());
        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                stack1.push(i);
            } else if c == '*' {
                stack2.push(i);
            } else {
                if !(stack1.pop().is_some() || stack2.pop().is_some()) {
                    return false;
                }
            }
        }
        while !stack1.is_empty() && !stack2.is_empty() {
            if stack1.pop().unwrap() > stack2.pop().unwrap() {
                return false;
            }
        }
        stack1.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 678. 有效的括号字符串
    #[test]
    fn check_valid_string() {
        let s = "(*))".to_string();
        let res = Solution::check_valid_string(s);
        assert_eq!(res, true);
    }
}
