use crate::model::list::ListNode;

pub struct Solution;

impl Solution {
    /// 80. 删除有序数组中的重复项 II
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let k = 2;
        if nums.len() <= k {
            return nums.len() as i32;
        }

        let mut p = k;
        for i in k..nums.len() {
            if nums[i] != nums[p - k] {
                nums[p] = nums[i];
                p += 1;
            }
        }
        p as i32
    }

    /// 82. 删除排序链表中的重复元素 II
    pub fn delete_duplicates_ii(head: Option<Box<ListNode<i32>>>) -> Option<Box<ListNode<i32>>> {
        if let Some(mut head_node) = head {
            let val = head_node.val;
            let cur = &mut head_node.next.take();
            let mut duplicate_count = 0;
            while let Some(node) = cur {
                if node.val == val {
                    *cur = node.next.take();
                    duplicate_count += 1;
                } else {
                    break;
                }
            }
            if duplicate_count > 0 {
                return Self::delete_duplicates_ii(cur.take());
            } else {
                head_node.next = Self::delete_duplicates_ii(cur.take());
                return Some(head_node);
            }
        }
        head
    }

    /// 83. 删除排序链表中的重复元素
    pub fn delete_duplicates(head: Option<Box<ListNode<i32>>>) -> Option<Box<ListNode<i32>>> {
        if let Some(mut head_node) = head {
            let val = head_node.val;
            let cur = &mut head_node.next.take();
            while let Some(node) = cur {
                if node.val == val {
                    *cur = node.next.take();
                } else {
                    break;
                }
            }
            head_node.next = Self::delete_duplicates(cur.take());
            return Some(head_node);
        }
        None
    }

    /// 88. 合并两个有序数组
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m - 1;
        let mut j = n - 1;
        while i >= 0 || j >= 0 {
            if i < 0 {
                nums1[(i + j + 1) as usize] = nums2[j as usize];
                j -= 1;
            } else if j < 0 {
                nums1[(i + j + 1) as usize] = nums1[i as usize];
                i -= 1;
            } else if nums1[i as usize] < nums2[j as usize] {
                nums1[(i + j + 1) as usize] = nums2[j as usize];
                j -= 1;
            } else {
                nums1[(i + j + 1) as usize] = nums1[i as usize];
                i -= 1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::list::List;

    use super::*;

    /// 80. 删除有序数组中的重复项 II
    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let res = Solution::remove_duplicates(&mut nums);
        assert_eq!(&nums[0..res as usize], vec![1, 1, 2, 2, 3]);
    }

    /// 82. 删除排序链表中的重复元素 II
    #[test]
    fn test_delete_duplicates_ii() {
        let nums = vec![1, 2, 3, 3, 4, 4, 5];
        let head = List::new(&nums).head;
        let res = List {
            head: Solution::delete_duplicates_ii(head),
        }
        .as_vec();
        assert_eq!(res, vec![1, 2, 5]);
    }

    /// 83. 删除排序链表中的重复元素
    #[test]
    fn test_delete_duplicates() {
        let nums = vec![1, 1, 2];
        let head = List::new(&nums).head;
        let res = List {
            head: Solution::delete_duplicates(head),
        }
        .as_vec();
        assert_eq!(res, vec![1, 2]);
    }

    /// 88. 合并两个有序数组
    #[test]
    fn test_merge() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }
}
