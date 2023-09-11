pub struct Solution {}

impl Solution {
    /// 2594. 修车的最少时间
    #[allow(dead_code)]
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let (mut l, mut r) = (0, ranks[0] as i64 * cars as i64 * cars as i64);
        while l <= r {
            let p = (l + r) / 2;
            let mut num_cars = 0i64;
            for &rank in &ranks {
                num_cars += ((p / rank as i64) as f64).sqrt() as i64;
            }
            if num_cars < cars as i64 {
                l = p + 1;
            } else {
                r = p - 1;
            }
        }
        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 2594. 修车的最少时间
    #[test]
    fn repair_cars() {
        let ranks = vec![4,2,3,1];
        let cars = 10;
        let res = Solution::repair_cars(ranks, cars);
        assert_eq!(res, 16);
    }
}