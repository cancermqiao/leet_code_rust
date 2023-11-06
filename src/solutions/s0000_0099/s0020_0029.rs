use crate::data::list::ListNode;

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

    /// 21. 合并两个有序链表
    #[allow(dead_code)]
    pub fn merge_two_lists(
        list1: Option<Box<ListNode<i32>>>,
        list2: Option<Box<ListNode<i32>>>,
    ) -> Option<Box<ListNode<i32>>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(a), None) | (None, Some(a)) => Some(a),
            (Some(mut a), Some(mut b)) => {
                if a.val > b.val {
                    b.next = Self::merge_two_lists(Some(a), b.next);
                    Some(b)
                } else {
                    a.next = Self::merge_two_lists(a.next, Some(b));
                    Some(a)
                }
            }
        }
    }

    /// 22. 括号生成
    fn dfs(l: i32, r: i32, buf: &mut String, res: &mut Vec<String>) {
        if l == 0 {
            res.push(format!("{}{}", buf, vec![")"; r as usize].concat()));
        } else {
            buf.push('(');
            Self::dfs(l - 1, r + 1, buf, res);
            buf.pop();
            if r > 0 {
                buf.push(')');
                Self::dfs(l, r - 1, buf, res);
                buf.pop();
            }
        }
    }

    #[allow(dead_code)]
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = Vec::new();
        let mut buf = "".to_string();
        Self::dfs(n, 0, &mut buf, &mut res);
        res
    }

    /// 23. 合并k个升序列表
    #[allow(dead_code)]
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode<i32>>>>) -> Option<Box<ListNode<i32>>> {
        let n = lists.len();
        if n == 0 {
            None
        } else if n == 1 {
            lists[0].clone()
        } else if n == 2 {
            Self::merge_two_lists(lists[0].clone(), lists[1].clone())
        } else {
            Self::merge_k_lists(vec![
                Self::merge_k_lists(lists[0..n / 2].to_vec()),
                Self::merge_k_lists(lists[n / 2..].to_vec()),
            ])
        }
    }

    /// 24. 两两交换链表中的节点
    #[allow(dead_code)]
    pub fn swap_pairs(head: Option<Box<ListNode<i32>>>) -> Option<Box<ListNode<i32>>> {
        if let Some(mut node1) = head {
            if let Some(mut node2) = node1.next.take() {
                node2.next.insert(node1).next = Self::swap_pairs(node2.next.take());
                Some(node2)
            } else {
                Some(node1)
            }
        } else {
            None
        }
    }

    /// 25. K 个一组翻转链表
    fn reverse_one(
        mut head: Option<Box<ListNode<i32>>>,
        mut next_head: Option<Box<ListNode<i32>>>,
    ) -> Option<Box<ListNode<i32>>> {
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = next_head;
            next_head = Some(node);
        }
        next_head
    }

    #[allow(dead_code)]
    pub fn reverse_k_group(
        mut head: Option<Box<ListNode<i32>>>,
        k: i32,
    ) -> Option<Box<ListNode<i32>>> {
        let mut next_head = &mut head;
        for _ in 0..k {
            if let Some(node) = next_head.as_mut() {
                next_head = &mut node.next;
            } else {
                return head;
            }
        }
        let next_head = Self::reverse_k_group(next_head.take(), k);
        Self::reverse_one(head, next_head)
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

    /// 29. 两数相除
    fn quick_add(dividend: i32, mut divisor: i32, mut quotient: i32) -> bool {
        let mut res = 0;
        while quotient > 0 {
            if quotient & 1 == 1 {
                if res < dividend - divisor {
                    return false;
                }
                res += divisor;
            }
            if quotient > 1 {
                if divisor < dividend.wrapping_sub(divisor) {
                    return false;
                }
                divisor += divisor;
            }
            quotient >>= 1;
        }
        true
    }

    #[allow(dead_code)]
    pub fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
        if dividend == 0 {
            return 0;
        }
        if dividend == i32::MIN {
            if divisor == 1 {
                return i32::MIN;
            }
            if divisor == -1 {
                return i32::MAX;
            }
            if divisor == i32::MIN {
                return 1;
            }
        }
        if divisor == i32::MIN {
            return 0;
        }
        let mut sign = 1;
        if dividend > 0 {
            dividend = -dividend;
            sign = -sign;
        }
        if divisor > 0 {
            divisor = -divisor;
            sign = -sign;
        }

        let (mut l, mut r, mut res) = (1, i32::MAX, 0);
        while l <= r {
            let mid = l + ((r - l) >> 1);
            let check = Self::quick_add(dividend, divisor, mid);
            if check {
                res = mid;
                if mid == i32::MAX {
                    break;
                }
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        if sign < 0 {
            -res
        } else {
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::list::List;

    use super::*;

    /// 20. 有效括号
    #[test]
    fn is_valid() {
        let s = "()".to_string();
        let res = Solution::is_valid(s);
        assert!(res);
    }

    /// 21. 合并两个有序链表
    #[test]
    fn merge_two_lists() {
        let l1 = vec![1, 2, 4];
        let l2 = vec![1, 3, 4];
        let list1 = List::new(&l1);
        let list2 = List::new(&l2);
        let res = Solution::merge_two_lists(list1.head, list2.head);
        let vectors = List { head: res }.as_vec();
        assert_eq!(vectors, vec![1, 1, 2, 3, 4, 4]);
    }

    /// 22. 括号生成
    #[test]
    fn generate_parenthesis() {
        let n = 3;
        let res = Solution::generate_parenthesis(n);
        assert_eq!(res, vec!["((()))", "(()())", "(())()", "()(())", "()()()"]);
    }

    /// 23. 合并k个升序列表
    #[test]
    fn merge_k_lists() {
        let vals = vec![vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]];
        let lists: Vec<Option<Box<ListNode<i32>>>> =
            vals.iter().map(|x| List::new(x).head).collect();
        let res = Solution::merge_k_lists(lists);
        let vectors = List { head: res }.as_vec();
        assert_eq!(vectors, vec![1, 1, 2, 3, 4, 4, 5, 6]);
    }

    /// 24. 两两交换链表中的节点
    #[test]
    fn swap_pairs() {
        let vals = vec![1, 2, 3, 4];
        let head = List::new(&vals);
        let res = Solution::swap_pairs(head.head);
        let vectors = List { head: res }.as_vec();
        assert_eq!(vectors, vec![2, 1, 4, 3]);
    }

    /// 25. K 个一组翻转链表
    #[test]
    fn reverse_k_group() {
        let vals = vec![1, 2, 3, 4, 5];
        let head = List::new(&vals);
        let k = 2;
        let res = Solution::reverse_k_group(head.head, k);
        let vectors = List { head: res }.as_vec();
        assert_eq!(vectors, vec![2, 1, 4, 3, 5]);
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

    /// 29. 两数相除
    #[test]
    fn divide() {
        let dividend = 10;
        let divisor = 3;
        let res = Solution::divide(dividend, divisor);
        assert_eq!(res, 3);
    }
}
