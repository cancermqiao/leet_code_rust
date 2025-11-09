use crate::model::list::ListNode;

pub struct Solution;

impl Solution {
    /// 3217. 从链表中移除在数组中存在的节点
    pub fn modified_list(
        nums: Vec<i32>,
        head: Option<Box<ListNode<i32>>>,
    ) -> Option<Box<ListNode<i32>>> {
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;
        let mut cur = dummy.as_mut();
        let nums = nums.into_iter().collect::<std::collections::HashSet<_>>();
        while let Some(next) = cur.next.as_mut() {
            if nums.contains(&next.val) {
                cur.next = next.next.take();
            } else {
                cur = cur.next.as_mut().unwrap();
            }
        }
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::list::List;

    /// 3217. 从链表中移除在数组中存在的节点
    #[test]
    fn modified_list() {
        let nums = vec![1, 2, 3];
        let head = List::new(&[1, 2, 3, 4, 5]);
        let res = List {
            head: Solution::modified_list(nums, head.head),
        }
        .as_vec();
        assert_eq!(res, vec![4, 5]);
    }
}
