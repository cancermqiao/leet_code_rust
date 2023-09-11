pub struct Solution {}

impl Solution {
    /// 2651. 计算列车到站时间
    #[allow(dead_code)]
    pub fn find_delayed_arrival_time(arrival_time: i32, delayed_time: i32) -> i32 {
        (arrival_time + delayed_time) % 24
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
}