pub struct Solution;

impl Solution {
    /// 3318. 计算子数组的 x-sum I
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut res = Vec::new();
        let mut freq: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        for i in 0..nums.len() {
            freq.entry(nums[i]).and_modify(|cnt| *cnt += 1).or_insert(1);

            if i + 1 >= k as usize {
                let mut freq_vec: Vec<(i32, i32)> = freq.clone().into_iter().collect();
                freq_vec.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));
                freq_vec.truncate(x as usize);
                res.push(freq_vec.iter().fold(0, |acc, e| acc + e.0 * e.1));
                freq.entry(nums[i + 1 - k as usize])
                    .and_modify(|cnt| *cnt -= 1);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 3318. 计算子数组的 x-sum I
    #[test]
    fn find_x_sum() {
        let nums = vec![1, 1, 2, 2, 3, 4, 2, 3];
        let k = 6;
        let x = 2;
        let res = Solution::find_x_sum(nums, k, x);
        assert_eq!(res, vec![6, 10, 12]);
    }
}
