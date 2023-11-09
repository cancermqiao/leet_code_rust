pub struct Solution {}

impl Solution {
    /// 2258. 逃离火灾
    #[allow(dead_code)]
    pub fn maximum_minutes(grid: Vec<Vec<i32>>) -> i32 {
        let (nr, nc) = (grid.len(), grid[0].len());
        let bfs = |pos: &mut Vec<(usize, usize)>| -> (i32, i32, i32) {
            let mut times = vec![vec![-1; nc]; nr];
            for &(x, y) in pos.iter() {
                times[x][y] = 0;
            }
            let mut t = 1;
            while !pos.is_empty() {
                let n = pos.len();
                for _ in 0..n {
                    let (i, j) = pos.pop().unwrap();
                    for (x, y) in [
                        (i.overflowing_add(1), j.overflowing_add(0)),
                        (i.overflowing_sub(1), j.overflowing_add(0)),
                        (i.overflowing_add(0), j.overflowing_add(1)),
                        (i.overflowing_add(0), j.overflowing_sub(1)),
                    ] {
                        if !x.1
                            && !y.1
                            && x.0 < nr
                            && y.0 < nc
                            && grid[x.0][y.0] == 0
                            && times[x.0][y.0] == -1
                        {
                            times[x.0][y.0] = t;
                            pos.insert(0, (x.0, y.0))
                        }
                    }
                }
                t += 1;
            }
            (
                times[nr - 1][nc - 1],
                times[nr - 1][nc - 2],
                times[nr - 2][nc - 1],
            )
        };
        let (man_time_end, man_time_left, man_time_top) = bfs(&mut vec![(0, 0)]);
        // 人无法到达安全屋
        if man_time_end == -1 {
            return -1;
        }
        let mut fire_pos = Vec::new();
        for (i, row) in grid.iter().enumerate() {
            for (j, &x) in row.iter().enumerate() {
                if x == 1 {
                    fire_pos.push((i, j))
                }
            }
        }
        let (fire_time_end, fire_time_left, fire_time_top) = bfs(&mut fire_pos);
        // 火烧不到安全屋
        if fire_time_end == -1 {
            return 1_000_000_000;
        }

        // 最大等待时间
        let stay_time = fire_time_end - man_time_end;

        if stay_time < 0 {
            // 火比人先到安全屋
            -1
        } else if (man_time_left != -1 && fire_time_left > man_time_left + stay_time)
            || (man_time_top != -1 && fire_time_top > man_time_top + stay_time)
        {
            stay_time
        } else {
            stay_time - 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 2258. 逃离火灾
    #[test]
    fn maximum_minutes() {
        let grid = vec![
            vec![0, 2, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 2, 2, 1, 0],
            vec![0, 2, 0, 0, 1, 2, 0],
            vec![0, 0, 2, 2, 2, 0, 2],
            vec![0, 0, 0, 0, 0, 0, 0],
        ];
        let res = Solution::maximum_minutes(grid);
        assert_eq!(res, 3);
    }
}
