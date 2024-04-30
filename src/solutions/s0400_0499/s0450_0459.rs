pub struct Solution;

impl Solution {
    /// 452. 用最少数量的箭引爆气球
    #[allow(dead_code)]
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        if points.is_empty() {
            return 0;
        }
        points.sort();
        let (mut arrow, mut r) = (1, i32::MAX);
        for point in points {
            if point[0] > r {
                arrow += 1;
                r = point[1];
            } else {
                r = r.min(point[1]);
            }
        }
        arrow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 452. 用最少数量的箭引爆气球
    #[test]
    fn find_min_arrow_shots() {
        let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
        let res = Solution::find_min_arrow_shots(points);
        assert_eq!(res, 2);
    }
}
