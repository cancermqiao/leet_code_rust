pub struct List<T>
where
    T: Copy,
{
    pub head: Link<T>,
}

type Link<T> = Option<Box<ListNode<T>>>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Link<T>,
}

impl<T> ListNode<T> {
    /// 创建ListNode
    #[inline]
    pub fn new(val: T) -> Self {
        ListNode { next: None, val }
    }
}

impl<T> List<T>
where
    T: Copy,
{
    #[inline]
    pub fn new(vals: &Vec<T>) -> Self {
        if vals.is_empty() {
            List { head: None }
        } else {
            let mut head = List { head: None };
            for i in (0..vals.len()).rev() {
                head.push(vals[i]);
            }
            head
        }
    }

    /// 向链表的头部插入节点
    pub fn push(&mut self, val: T) {
        let new_node = Box::new(ListNode {
            val: val,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    /// 抛出链表的头结点，返回对应的值
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }

    pub fn to_vec(&mut self) -> Vec<T> {
        let mut vectors = Vec::new();
        while let Some(val) = self.pop() {
            vectors.push(val);
        }
        vectors
    }
}

// impl<T> Drop for List<T> where T: Copy {
//     fn drop(&mut self) {
//         let mut cur_link = self.head.take();
//         while let Some(mut boxed_node) = cur_link {
//             cur_link = boxed_node.next.take();
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list() {
        let vals = vec![1, 2, 3];
        let mut list = List::new(&vals);
        list.push(4);
        let val = list.pop().unwrap();
        assert_eq!(val, 4);
        let vectors = list.to_vec();
        assert_eq!(vectors, vals);
    }
}
