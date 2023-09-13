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

    /// 2596. 检查骑士巡视方案
    #[allow(dead_code)]
    pub fn check_valid_grid(grid: Vec<Vec<i32>>) -> bool {
        if grid[0][0] != 0 {
            return false;
        }
        let (nr, nc) = (grid.len() as i32, grid[0].len() as i32);
        let (mut x, mut y) = (0, 0);
        for step in 1..nr * nc {
            let mut exist = false;
            for (nx, ny) in [
                (x + 2, y + 1),
                (x + 2, y - 1),
                (x - 2, y + 1),
                (x - 2, y - 1),
                (x + 1, y + 2),
                (x - 1, y + 2),
                (x + 1, y - 2),
                (x - 1, y - 2),
            ] {
                if nx >= 0 && nx < nr && ny >= 0 && ny < nc {
                    if grid[nx as usize][ny as usize] == step {
                        x = nx;
                        y = ny;
                        exist = true;
                        break;
                    }
                }
            }
            if !exist {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 2594. 修车的最少时间
    #[test]
    fn repair_cars() {
        let ranks = vec![4, 2, 3, 1];
        let cars = 10;
        let res = Solution::repair_cars(ranks, cars);
        assert_eq!(res, 16);
    }

    /// 2596. 检查骑士巡视方案
    #[test]
    fn check_valid_grid() {
        let grid = vec![
            vec![0, 11, 16, 5, 20],
            vec![17, 4, 19, 10, 15],
            vec![12, 1, 8, 21, 6],
            vec![3, 18, 23, 14, 9],
            vec![24, 13, 2, 7, 22],
        ];
        let res = Solution::check_valid_grid(grid);
        assert_eq!(res, true);
    }
}
