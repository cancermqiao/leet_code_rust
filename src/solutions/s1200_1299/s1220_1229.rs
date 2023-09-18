pub struct Solution {}

impl Solution {
    /// 1222. 可以攻击国王的皇后
    fn key(x: i32) -> i32 {
        if x > 0 {
            1
        } else if x < 0 {
            -1
        } else {
            0
        }
    }

    #[allow(dead_code)]
    pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: std::collections::HashMap<(i32, i32), (Vec<i32>, i32)> =
            std::collections::HashMap::new();
        for queen in queens {
            let (dx, dy) = (queen[0] - king[0], queen[1] - king[1]);
            if dx == 0 || dy == 0 || dx.abs() == dy.abs() {
                let key = (Self::key(dx), Self::key(dy));
                if let Some(value) = res.get(&key) {
                    if value.1 > dx.abs() + dy.abs() {
                        res.insert(key, (queen, dx.abs() + dy.abs()));
                    }
                } else {
                    res.insert(key, (queen, dx.abs() + dy.abs()));
                }
            }
        }
        res.into_iter().map(|x| x.1 .0).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1222. 可以攻击国王的皇后
    #[test]
    fn queens_attackthe_king() {
        let queens = vec![
            vec![0, 1],
            vec![1, 0],
            vec![4, 0],
            vec![0, 4],
            vec![3, 3],
            vec![2, 4],
        ];
        let king = vec![0, 0];
        let mut res = Solution::queens_attackthe_king(queens, king);
        res.sort();
        assert_eq!(res, vec![[0, 1], [1, 0], [3, 3]]);
    }
}
