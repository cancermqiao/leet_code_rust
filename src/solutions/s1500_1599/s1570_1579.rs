pub struct Solution;

impl Solution {
    /// 1578. 使绳子变成彩色的最短时间
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut pre = 'a';
        let (mut res, mut time_max) = (0, 0);
        for (i, c) in colors.chars().enumerate() {
            if i == 0 {
                pre = c;
                time_max = needed_time[i];
                continue;
            }
            if c == pre {
                if needed_time[i] > time_max {
                    res += time_max;
                    time_max = needed_time[i];
                } else {
                    res += needed_time[i];
                }
            } else {
                pre = c;
                time_max = needed_time[i];
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1578. 使绳子变成彩色的最短时间
    #[test]
    fn min_cost() {
        let colors = "abaac".to_string();
        let needed_time = vec![1, 2, 3, 4, 5];
        let res = Solution::min_cost(colors, needed_time);
        assert_eq!(res, 3);
    }
}