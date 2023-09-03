pub struct Solution {}

impl Solution {
    /// 20. 有效括号
    #[allow(dead_code)]
    pub fn is_valid(s: String) -> bool {
        let sets_map = std::collections::HashMap::from([(')', '('), (']', '['), ('}', '{')]);
        let mut stack = Vec::new();
        for c in s.chars() {
            if sets_map.contains_key(&c) {
                match stack.pop() {
                    Some(l) => {
                        if Some(&l) != sets_map.get(&c) {
                            return false;
                        }
                    }
                    None => return false,
                }
            } else {
                stack.push(c);
            }
        }
        stack.is_empty()
    }

    /// 26. 删除有序数组中的重复项
    #[allow(dead_code)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut p = 0;
        for i in 1..nums.len() {
            if nums[i] != nums[p] {
                nums[p + 1] = nums[i];
                p += 1;
            }
        }
        (p + 1) as i32
    }

    /// 27. 移除元素
    #[allow(dead_code)]
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut p = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[p] = nums[i];
                p += 1;
            }
        }
        p as i32
    }

    /// 28. 找出字符串中第一个匹配项的下标
    #[allow(dead_code)]
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if haystack.len() < needle.len() {
            return -1;
        }
        for i in 0..(haystack.len() - needle.len() + 1) {
            let mut j = 0;
            while j < needle.len() && haystack.chars().nth(i + j) == needle.chars().nth(j) {
                j += 1;
            }
            if j == needle.len() {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 20. 有效括号
    #[test]
    fn is_valid() {
        let s = "()".to_string();
        let res = Solution::is_valid(s);
        assert_eq!(res, true);
    }

    /// 26. 移除元素
    #[test]
    fn remove_duplicates() {
        let mut nums = vec![1, 1, 2];
        let res = Solution::remove_duplicates(&mut nums);
        assert_eq!(&nums[0..res as usize], [1, 2]);
    }

    /// 27. 移除元素
    #[test]
    fn remove_element() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        let res = Solution::remove_element(&mut nums, val);
        assert_eq!(&nums[0..res as usize], [2, 2]);
    }

    /// 28. 找出字符串中第一个匹配项的下标
    #[test]
    fn str_str() {
        let haystack = "sadbutsad".to_string();
        let needle = "sad".to_string();
        let res = Solution::str_str(haystack, needle);
        assert_eq!(res, 0);
    }
}
