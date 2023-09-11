use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    /// 11. 盛最多水的容器
    #[allow(dead_code)]
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, height.len() - 1);
        let mut max_area = 0;
        while l < r {
            max_area = max_area.max((r - l) as i32 * height[l].min(height[r]));
            if height[l] > height[r] {
                r -= 1;
            } else {
                l += 1;
            }
        }
        max_area
    }

    /// 12. 整数转罗马数字
    #[allow(dead_code)]
    pub fn int_to_roman(mut num: i32) -> String {
        let value_roman_vec = vec![
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];
        let mut res = "".to_string();
        for (value, roman) in value_roman_vec {
            while num >= value {
                num -= value;
                res.push_str(roman);
            }
        }
        res
    }

    /// 13. 罗马数字转整数
    #[allow(dead_code)]
    pub fn roman_to_int(s: String) -> i32 {
        let roman_int_map = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
        let (mut res, mut pre) = (0, 0);
        for c in s.chars() {
            let num = roman_int_map.get(&c).unwrap();
            if *num > pre {
                res -= pre;
            } else {
                res += pre;
            }
            pre = *num;
        }
        res + pre
    }

    /// 14. 最长公共前缀
    #[allow(dead_code)]
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }
        for (i, c) in strs[0].chars().enumerate() {
            for s in &strs[1..] {
                if s.len() < i + 1 || s.chars().nth(i).unwrap() != c {
                    return strs[0][0..i].to_string();
                }
            }
        }
        strs[0].clone()
    }

    /// 15. 三数之和
    #[allow(dead_code)]
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut res = Vec::new();
        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let (mut left, mut right) = (i + 1, nums.len() - 1);
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum > 0 {
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    res.push(vec![nums[i], nums[left], nums[right]]);
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
                    left += 1;
                    right -= 1;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 11. 盛最多水的容器
    #[test]
    fn max_area() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let res = Solution::max_area(height);
        assert_eq!(res, 49);
    }

    /// 12. 整数转罗马数字
    #[test]
    fn int_to_roman() {
        let num = 3;
        let res = Solution::int_to_roman(num);
        assert_eq!(res, "III");
    }

    /// 13. 罗马数字转整数
    #[test]
    fn roman_to_int() {
        let s = "III".to_string();
        let res = Solution::roman_to_int(s);
        assert_eq!(res, 3);
    }

    /// 14. 最长公共前缀
    #[test]
    fn longest_common_prefix() {
        let strs = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        let res = Solution::longest_common_prefix(strs);
        assert_eq!(res, "fl".to_string());
    }

    /// 15. 三数之和
    #[test]
    fn three_sum() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let res = Solution::three_sum(nums);
        assert_eq!(res, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }
}
