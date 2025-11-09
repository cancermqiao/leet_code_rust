pub struct Solution;

impl Solution {
    /// 2651. 计算列车到站时间
    pub fn find_delayed_arrival_time(arrival_time: i32, delayed_time: i32) -> i32 {
        (arrival_time + delayed_time) % 24
    }

    /// 2652. 倍数求和
    pub fn sum_of_multiples(n: i32) -> i32 {
        let mut res = 0;
        for i in 1..=n {
            if i % 3 == 0 || i % 5 == 0 || i % 7 == 0 {
                res += i;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 2651. 计算列车到站时间
    #[test]
    fn find_delayed_arrival_time() {
        let arrival_time = 15;
        let delayed_time = 5;
        let res = Solution::find_delayed_arrival_time(arrival_time, delayed_time);
        assert_eq!(res, 20);
    }

    /// 2652. 倍数求和
    #[test]
    fn sum_of_multiples() {
        let n = 7;
        let res = Solution::sum_of_multiples(n);
        assert_eq!(res, 21);
    }
}
