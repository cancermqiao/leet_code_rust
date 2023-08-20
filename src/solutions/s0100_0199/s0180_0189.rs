pub struct Solution {}

impl Solution {
    // 189. 轮转数组
    #[allow(dead_code)]
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len() as i32;
        for i in 0..Self::gcd(n, k) {
            let mut next = (i + k) % n;
            while next != i {
                nums.swap(next as usize, i as usize);
                next = (next + k) % n;
            }
        }
    }

    // 欧几里得算法求最大公约数
    fn gcd(a: i32, b: i32) -> i32 {
        if a == 0 {
            return b;
        } else if b == 0 {
            return a;
        } else if a > b {
            return Solution::gcd(a % b, b);
        } else {
            return Solution::gcd(b % a, a);
        }
    }        
}

#[cfg(test)]
mod tests {
    use super::*;

    // 189. 轮转数组
    #[test]
    fn rotate() {
        let mut nums = vec![1,2,3,4,5,6,7];
        let k = 3;
        Solution::rotate(&mut nums, k);
        assert_eq!(nums, vec![5,6,7,1,2,3,4]);
    }
}