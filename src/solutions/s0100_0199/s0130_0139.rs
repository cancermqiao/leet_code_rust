use std::ops::Deref;

pub struct Solution {}

impl Solution {
    /// 134. 加油站
    #[allow(dead_code)]
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let (mut i, n) = (0, gas.len());
        while i < n {
            let (mut cnt, mut sum_of_gas, mut sum_of_cost) = (0, 0, 0);
            while cnt < n {
                let index = (i + cnt) % n;
                sum_of_gas += gas[index];
                sum_of_cost += cost[index];
                if sum_of_gas < sum_of_cost {
                    break;
                }
                cnt += 1;
            }
            
            if cnt == n {
                return i as i32;
            } else {
                i = i + cnt + 1;
            }
        }
        -1
    }

    /// 136. 只出现一次的数字
    #[allow(dead_code)]
    pub fn single_number_v1(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, num| acc ^ num)
    }

    /// 137. 只出现一次的数字 II
    #[allow(dead_code)]
    pub fn single_number_v2(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..32 {
            let mut count = 0;
            for &num in nums.iter() {
                count += (num >> i) & 1;
            }
            if count % 3 != 0 {
                res |= 1 << i;
            }
        }
        res
    }

    /// 139. 单次拆分
    #[allow(dead_code)]
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let n = word_dict.iter().map(|x| x.len()).max().unwrap();
        let mut dp = vec![0];
        for i in 0..s.len() {
            if dp.is_empty() {
                return false;
            }
            for &j in dp.iter() {
                if word_dict.contains(&s[j..i+1].to_string()) {
                    dp.insert(0, i+1);
                    break;
                }
            }
            if dp.last().unwrap() + n - 1 <= i {
                dp.pop();
            }
        }
        if let Some(&n) = dp.first() {
            n == s.len()
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 134. 加油站
    #[test]
    fn can_complete_circuit() {
        let gas = vec![1,2,3,4,5];
        let cost = vec![3,4,5,1,2];
        let res = Solution::can_complete_circuit(gas, cost);
        assert_eq!(res, 3);
    }

    /// 136. 只出现一次的数字
    #[test]
    fn single_number_v1() {
        let nums = vec![2,2,1];
        let res = Solution::single_number_v1(nums);
        assert_eq!(res, 1);
    }

    /// 137. 只出现一次的数字 II
    #[test]
    fn single_number_v2() {
        let nums = vec![2,2,3,2];
        let res = Solution::single_number_v2(nums);
        assert_eq!(res, 3);
    }

    /// 139. 单次拆分
    #[test]
    fn word_break() {
        let s = "leetcode".to_string();
        let word_dict = vec!["leet".to_string(), "code".to_string()];
        let res = Solution::word_break(s, word_dict);
        assert_eq!(res, true);
    }
}