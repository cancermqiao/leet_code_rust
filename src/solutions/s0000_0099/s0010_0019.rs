use std::{cmp::Ordering, collections::HashMap};

use crate::model::list::ListNode;

pub struct Solution;

impl Solution {
    /// 10. 正则表达式匹配
    ///
    /// # 递归
    fn is_match_recursion(s: &[u8], p: &[u8]) -> bool {
        match (p, s) {
            ([], _) => s.is_empty(),
            ([a, b'*', ..], _) => {
                Self::is_match_recursion(s, &p[2..])
                    || (!s.is_empty()
                        && (*a == b'.' || *a == s[0])
                        && Self::is_match_recursion(&s[1..], &p[1..]))
            }
            ([b'.', ..], _) => !s.is_empty() && Self::is_match_recursion(&s[1..], &p[1..]),
            ([a, ..], [b, ..]) => a == b && Self::is_match_recursion(&s[1..], &p[1..]),
            _ => false,
        }
    }

    /// # 动态规划
    fn is_match_dynamic(s: &[u8], p: &[u8]) -> bool {
        let (m, n) = (s.len(), p.len());
        let mut dp = vec![vec![false; n + 1]; m + 1];
        let matches = |i: usize, j: usize| -> bool {
            if i == 0 {
                false
            } else if p[j - 1] == b'.' {
                true
            } else {
                s[i - 1] == p[j - 1]
            }
        };
        dp[0][0] = true;
        for i in 0..m + 1 {
            for j in 1..n + 1 {
                if p[j - 1] == b'*' {
                    dp[i][j] |= dp[i][j - 2];
                    if matches(i, j - 1) {
                        dp[i][j] |= dp[i - 1][j];
                    }
                } else if matches(i, j) {
                    dp[i][j] |= dp[i - 1][j - 1];
                }
            }
        }
        dp[m][n]
    }

    pub fn is_match(s: String, p: String, method_type: &str) -> bool {
        let (s, p) = (s.as_bytes(), p.as_bytes());
        match method_type {
            "recursion" => Self::is_match_recursion(s, p),
            "dynamic" => Self::is_match_dynamic(s, p),
            _ => Self::is_match_recursion(s, p),
        }
    }

    /// 11. 盛最多水的容器
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
                match sum.cmp(&0) {
                    Ordering::Greater => right -= 1,
                    Ordering::Less => left += 1,
                    Ordering::Equal => {
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
        }
        res
    }

    /// 16. 最接近的三数之和
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let mut res = nums[0] + nums[1] + nums[2];
        for i in 0..nums.len() - 2 {
            let (mut l, mut r) = (i + 1, nums.len() - 1);
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if (sum - target).abs() < (res - target).abs() {
                    res = sum;
                }
                match sum.cmp(&target) {
                    Ordering::Equal => return sum,
                    Ordering::Greater => r -= 1,
                    Ordering::Less => l += 1,
                }
            }
        }
        res
    }

    /// 17. 电话号码的字母组合
    ///
    /// # 深度优先搜索
    fn dfs(s: &[u8], map: &HashMap<u8, &str>, buf: &mut String, res: &mut Vec<String>) {
        if s.is_empty() {
            res.push(buf.clone());
        } else {
            for c in map.get(&s[0]).unwrap().chars() {
                buf.push(c);
                Self::dfs(&s[1..], map, buf, res);
                buf.pop();
            }
        }
    }

    /// # 广度优先搜索
    fn bfs(s: &[u8], map: HashMap<u8, &str>, res: &mut Vec<String>) {
        res.push("".to_string());
        for key in s {
            for _ in 0..res.len() {
                let head = res.pop().unwrap();
                for c in map.get(key).unwrap().chars() {
                    res.insert(0, format!("{}{}", head, c));
                }
            }
        }
    }

    pub fn letter_combinations(digits: String, method_type: &str) -> Vec<String> {
        let map = HashMap::from([
            (b'2', "abc"),
            (b'3', "def"),
            (b'4', "ghi"),
            (b'5', "jkl"),
            (b'6', "mno"),
            (b'7', "pqrs"),
            (b'8', "tuv"),
            (b'9', "wxyz"),
        ]);
        let (mut buf, mut res) = ("".to_string(), Vec::new());
        if digits.is_empty() {
            return res;
        }
        match method_type {
            "dfs" => Self::dfs(digits.as_bytes(), &map, &mut buf, &mut res),
            "bfs" => Self::bfs(digits.as_bytes(), map, &mut res),
            _ => Self::dfs(digits.as_bytes(), &map, &mut buf, &mut res),
        }
        res
    }

    /// 18. 四数之和
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let n = nums.len();
        if n < 4 {
            return res;
        }
        let target = target as i64;
        nums.sort();
        for i in 0..n - 3 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            if nums[i] as i64 + nums[i + 1] as i64 + nums[i + 2] as i64 + nums[i + 3] as i64
                > target
            {
                break;
            }
            if (nums[i] as i64 + nums[n - 3] as i64 + nums[n - 2] as i64 + nums[n - 1] as i64)
                < target
            {
                continue;
            }
            for j in i + 1..n - 2 {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                if nums[i] as i64 + nums[j] as i64 + nums[j + 1] as i64 + nums[j + 2] as i64
                    > target
                {
                    break;
                }
                if (nums[i] as i64 + nums[j] as i64 + nums[n - 2] as i64 + nums[n - 1] as i64)
                    < target
                {
                    continue;
                }
                let (mut l, mut r) = (j + 1, n - 1);
                while l < r {
                    let sum = nums[i] as i64 + nums[j] as i64 + nums[l] as i64 + nums[r] as i64;
                    match sum.cmp(&target) {
                        Ordering::Equal => {
                            res.push(vec![nums[i], nums[j], nums[l], nums[r]]);
                            while l < r && nums[l] == nums[l + 1] {
                                l += 1;
                            }
                            l += 1;
                            while l < r && nums[r] == nums[r - 1] {
                                r -= 1;
                            }
                            r -= 1;
                        }
                        Ordering::Greater => r -= 1,
                        Ordering::Less => l += 1,
                    }
                }
            }
        }
        res
    }

    /// 19. 删除链表的倒数第 N 个结点
    pub fn remove_nth_from_end(
        head: Option<Box<ListNode<i32>>>,
        n: i32,
    ) -> Option<Box<ListNode<i32>>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut slow_head = &mut dummy;
        let mut fast_head = &slow_head.clone();
        for _ in 0..=n {
            if let Some(fast_node) = fast_head {
                fast_head = &fast_node.next;
            } else {
                return None;
            }
        }
        while fast_head.is_some() {
            fast_head = &fast_head.as_ref().unwrap().next;
            slow_head = &mut slow_head.as_mut().unwrap().next;
        }
        let remove_node = &mut slow_head.as_mut().unwrap().next;
        slow_head.as_mut().unwrap().next = remove_node.as_mut().unwrap().next.take();
        dummy.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use crate::model::list::List;

    use super::*;

    /// 10. 正则表达式匹配
    #[test]
    fn is_match() {
        let s = "aa";
        let p = "a";
        let res = Solution::is_match(s.to_string(), p.to_string(), "recursion");
        assert!(!res);
        let res = Solution::is_match(s.to_string(), p.to_string(), "dynamic");
        assert!(!res);
    }

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

    /// 16. 最接近的三数之和
    #[test]
    fn three_sum_closest() {
        let nums = vec![-1, 2, 1, -4];
        let target = 1;
        let res = Solution::three_sum_closest(nums, target);
        assert_eq!(res, 2);
    }

    /// 17. 电话号码的字母组合
    #[test]
    fn letter_combinations() {
        let digits = "23";
        let res = Solution::letter_combinations(digits.to_string(), "dfs");
        assert_eq!(
            res,
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
        let mut res = Solution::letter_combinations(digits.to_string(), "bfs");
        res.sort();
        assert_eq!(
            res,
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
    }

    /// 18. 四数之和
    #[test]
    fn four_sum() {
        let nums = vec![1, 0, -1, 0, -2, 2];
        let target = 0;
        let res = Solution::four_sum(nums, target);
        assert_eq!(
            res,
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        );
    }

    /// 19. 删除链表的倒数第 N 个结点
    #[test]
    fn remove_nth_from_end() {
        let vals = vec![1, 2, 3, 4, 5];
        let head = List::new(&vals);
        let n = 2;
        let res = Solution::remove_nth_from_end(head.head, n);
        let vectors = List { head: res }.as_vec();
        assert_eq!(vectors, vec![1, 2, 3, 5]);
    }
}
