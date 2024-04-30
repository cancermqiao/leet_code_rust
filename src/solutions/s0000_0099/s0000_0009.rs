use crate::model::list::ListNode;

enum State {
    Blank,
    Sign,
    Begin,
}

pub struct Solution;

impl Solution {
    /// 1. 两数之和
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut cache = std::collections::HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            if let Some(&v) = cache.get(num) {
                return vec![v, i as i32];
            } else {
                cache.insert(target - num, i as i32);
            }
        }
        vec![-1, -1]
    }

    /// 2. 两数相加
    fn carried(
        l1: Option<Box<ListNode<i32>>>,
        l2: Option<Box<ListNode<i32>>>,
        mut carry: i32,
    ) -> Option<Box<ListNode<i32>>> {
        if l1.is_none() && l2.is_none() && carry == 0 {
            None
        } else {
            Some(Box::new(ListNode {
                next: Self::carried(
                    l1.and_then(|x| {
                        carry += x.val;
                        x.next
                    }),
                    l2.and_then(|x| {
                        carry += x.val;
                        x.next
                    }),
                    carry / 10,
                ),
                val: carry % 10,
            }))
        }
    }

    #[allow(dead_code)]
    pub fn add_two_numbers(
        l1: Option<Box<ListNode<i32>>>,
        l2: Option<Box<ListNode<i32>>>,
    ) -> Option<Box<ListNode<i32>>> {
        Self::carried(l1, l2, 0)
    }

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

    /// 4. 寻找两个正序数组的中位数
    #[allow(dead_code)]
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mid = (nums1.len() + nums2.len()) / 2;
        let (mut pre, mut cur) = (0, 0);
        let (mut i, mut j) = (0, 0);
        for _ in 0..=mid {
            pre = cur;
            if i >= nums1.len() {
                cur = nums2[j];
                j += 1;
            } else if j >= nums2.len() {
                cur = nums1[i];
                i += 1;
            } else if nums1[i] > nums2[j] {
                cur = nums2[j];
                j += 1;
            } else {
                cur = nums1[i];
                i += 1;
            }
        }
        if (nums1.len() + nums2.len()) % 2 == 0 {
            (pre + cur) as f64 / 2.0
        } else {
            cur as f64
        }
    }

    /// 5. 最长回文子串
    fn expand_string(s: &str, mut l: i32, mut r: i32) -> (i32, i32) {
        while l >= 0
            && r < s.len() as i32
            && s[l as usize..l as usize + 1] == s[r as usize..r as usize + 1]
        {
            l -= 1;
            r += 1;
        }
        (l + 1, r - 1)
    }

    #[allow(dead_code)]
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return s;
        }
        let (mut max_length, mut l, mut r) = (1, 0, 0);
        for i in 1..s.len() {
            let (l1, r1) = Self::expand_string(&s, i as i32, i as i32);
            let (l2, r2) = Self::expand_string(&s, i as i32 - 1, i as i32);
            if r1 - l1 + 1 > max_length {
                l = l1;
                r = r1;
                max_length = r1 - l1 + 1;
            }
            if r2 - l2 + 1 > max_length {
                l = l2;
                r = r2;
                max_length = r2 - l2 + 1;
            }
        }
        s[l as usize..r as usize + 1].to_string()
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
            if i == 0 || i == num_rows - 1 {
                direction = -direction;
            }
            i += direction;
        }
        res.join("")
    }

    /// 7. 整数反转
    #[allow(dead_code)]
    pub fn reverse(mut x: i32) -> i32 {
        let mut res = 0;
        while x != 0 {
            if !(i32::MIN / 10..=i32::MAX / 10).contains(&res) {
                return 0;
            }
            res = res * 10 + x % 10;
            x /= 10;
        }
        res
    }

    /// 8. 字符串转换整数（atoi）
    #[allow(dead_code)]
    pub fn my_atoi(s: String) -> i32 {
        let mut state = State::Blank;
        let mut res = 0_i64;
        let mut sign = 1;
        for c in s.chars() {
            match state {
                State::Blank => {
                    if c == ' ' {
                        continue;
                    } else if c == '-' {
                        state = State::Sign;
                        sign = -1;
                    } else if c == '+' {
                        state = State::Sign;
                        sign = 1;
                    } else if c.is_ascii_digit() {
                        res = res * 10 + c.to_digit(10).unwrap() as i64;
                        state = State::Begin;
                    } else {
                        break;
                    }
                }
                State::Sign | State::Begin => {
                    if c.is_ascii_digit() {
                        res = res * 10 + c.to_digit(10).unwrap() as i64;
                        if sign == 1 {
                            res = res.min(i32::MAX as i64);
                        } else {
                            res = res.min(-(i32::MIN as i64));
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        res as i32 * sign
    }

    /// 9. 回文数
    #[allow(dead_code)]
    pub fn is_palindrome(mut x: i32) -> bool {
        // 0
        if x == 0 {
            return true;
        }
        // 负数和个位为0
        if x < 0 || x % 10 == 0 {
            return false;
        }
        let mut reverted_number = 0;
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
    use crate::model::list::List;

    use super::*;

    /// 1. 两数之和
    #[test]
    fn two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let res = Solution::two_sum(nums, target);
        assert_eq!(res, vec![0, 1]);
    }

    /// 2. 两数相加
    #[test]
    fn add_two_numbers() {
        let l1 = vec![2, 4, 3];
        let l2 = vec![5, 6, 4];
        let l1 = List::new(&l1).head;
        let l2 = List::new(&l2).head;
        let res = List {
            head: Solution::add_two_numbers(l1, l2),
        }
        .as_vec();
        assert_eq!(res, vec![7, 0, 8]);
    }

    /// 3. 无重复字符的最长子串
    #[test]
    fn length_of_longest_substring() {
        let s = "abcabcbb".to_string();
        let res = Solution::length_of_longest_substring(s);
        assert_eq!(res, 3);
    }

    /// 4. 寻找两个正序数组的中位数
    #[test]
    fn find_median_sorted_arrays() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        let res = Solution::find_median_sorted_arrays(nums1, nums2);
        assert_eq!(res, 2.0);
    }

    /// 5. 最长回文子串
    #[test]
    fn longest_palindrome() {
        let s = "babad".to_string();
        let res = Solution::longest_palindrome(s);
        assert_eq!(res, "bab".to_string());
    }

    /// 6. N字形变换
    #[test]
    fn convert() {
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 3;
        let res = Solution::convert(s, num_rows);
        assert_eq!(res, "PAHNAPLSIIGYIR".to_string())
    }

    /// 7. 整数反转
    #[test]
    fn reverse() {
        let x = i32::MAX;
        let res = Solution::reverse(x);
        assert_eq!(res, 0);
    }

    /// 8. 字符串转换整数（atoi）
    #[test]
    fn my_atoi() {
        let s = "   -42".to_string();
        let res = Solution::my_atoi(s);
        assert_eq!(res, -42);
    }

    /// 9. 回文数
    #[test]
    fn is_palindrome() {
        let x = 121;
        let res = Solution::is_palindrome(x);
        assert!(res);
    }
}
