pub struct Solution;

impl Solution {
    /// 1921. 消灭怪物的最大数量
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut arrive_times = dist
            .iter()
            .zip(speed.iter())
            .map(|(a, b)| (a + b - 1) / b)
            .collect::<Vec<i32>>();
        arrive_times.sort();
        for (attack_time, &arrive_time) in arrive_times.iter().enumerate() {
            if arrive_time <= attack_time as i32 {
                return attack_time as i32;
            }
        }
        arrive_times.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1921. 消灭怪物的最大数量
    #[test]
    fn eliminate_maximum() {
        let dist = vec![1, 3, 4];
        let speed = vec![1, 1, 1];
        let res = Solution::eliminate_maximum(dist, speed);
        assert_eq!(res, 3);
    }
}
