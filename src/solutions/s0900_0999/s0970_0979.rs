pub struct Solution;

impl Solution {
    /// 978. 最长湍流子数组
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }

        let mut max_size = 1;
        let mut cur_size = 1;
        let mut gap = 0;
        for i in 1..n {
            let cur_gap = (arr[i] - arr[i - 1] > 0) as i32 - (arr[i] - arr[i - 1] < 0) as i32;
            if gap == 0 {
                if cur_gap != 0 {
                    cur_size += 1;
                }
            } else if cur_gap * gap < 0 {
                cur_size += 1;
            } else if cur_gap * gap > 0 {
                cur_size = 2;
            } else {
                cur_size = 1;
            }
            max_size = max_size.max(cur_size);
            gap = cur_gap;
        }
        max_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 978. 最长湍流子数组
    #[test]
    fn max_turbulence_size() {
        let arr = vec![9, 4, 2, 10, 7, 8, 8, 1, 9];
        let res = Solution::max_turbulence_size(arr);
        assert_eq!(res, 5)
    }
}
