use std::cmp::Ordering;

pub struct Solution {}

impl Solution {
    /// 823. 带因子的二叉树
    #[allow(dead_code)]
    pub fn num_factored_binary_trees(arr: Vec<i32>) -> i32 {
        let mut arr: Vec<i64> = arr.iter().map(|&x| x as i64).collect();
        arr.sort_unstable();
        let mut dp = vec![1_i64; arr.len()];
        let mod_num = 10_i64.pow(9) + 7;
        for i in 1..arr.len() {
            let (mut left, mut right) = (0, i as i32 - 1);
            while left <= right {
                let mult = arr[left as usize] * arr[right as usize];
                match mult.cmp(&arr[i]) {
                    Ordering::Equal => {
                        dp[i] +=
                            (dp[left as usize] * dp[right as usize] * (1 + (left != right) as i64))
                                % mod_num;
                        dp[i] %= mod_num;
                        left += 1;
                        right -= 1;
                    }
                    Ordering::Greater => right -= 1,
                    Ordering::Less => left += 1,
                }
            }
        }
        dp.iter().fold(0_i64, |acc, num| (acc + num) % mod_num) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 823. 带因子的二叉树
    #[test]
    fn num_factored_binary_trees() {
        let arr = vec![18865777, 36451879, 36878647];
        let res = Solution::num_factored_binary_trees(arr);
        assert_eq!(res, 3)
    }
}
