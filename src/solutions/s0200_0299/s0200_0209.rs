pub struct Solution;

impl Solution {
    /// 200. 岛屿的数量
    #[allow(dead_code)]
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let (nr, nc) = (grid.len() as i32, grid[0].len() as i32);
        let mut num_islands = 0;
        for i in 0..nr {
            for j in 0..nc {
                if grid[i as usize][j as usize] == '1' {
                    num_islands += 1;
                    grid[i as usize][j as usize] = '0';
                    let mut stack = vec![(i, j)];
                    while let Some((x, y)) = stack.pop() {
                        for (x, y) in [(x + 1, y), (x - 1, y), (x, y - 1), (x, y + 1)] {
                            if x >= 0
                                && x < nr
                                && y >= 0
                                && y < nc
                                && grid[x as usize][y as usize] == '1'
                            {
                                stack.push((x, y));
                                grid[x as usize][y as usize] = '0';
                            }
                        }
                    }
                }
            }
        }
        num_islands
    }

    /// 202. 快乐数
    #[allow(dead_code)]
    pub fn is_happy(n: i32) -> bool {
        let get_next = |mut num: i32| {
            let mut res = 0;
            while num > 0 {
                res += (num % 10).pow(2);
                num /= 10;
            }
            res
        };
        let mut slow_runner = n;
        let mut fast_runner = get_next(n);
        while fast_runner != 1 && fast_runner != slow_runner {
            slow_runner = get_next(slow_runner);
            fast_runner = get_next(get_next(fast_runner));
        }
        fast_runner == 1
    }

    /// 205. 同构字符串
    #[allow(dead_code)]
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut s2t: std::collections::HashMap<char, char> = std::collections::HashMap::new();
        let mut t2s: std::collections::HashMap<char, char> = std::collections::HashMap::new();
        for i in 0..s.len() {
            let s_char = s.chars().nth(i).unwrap();
            let t_char = t.chars().nth(i).unwrap();
            if let Some(&v) = s2t.get(&s_char) {
                if v != t_char {
                    return false;
                }
            } else {
                s2t.insert(s_char, t_char);
            }
            if let Some(&v) = t2s.get(&t_char) {
                if v != s_char {
                    return false;
                }
            } else {
                t2s.insert(t_char, s_char);
            }
        }
        true
    }

    /// 209. 长度最小的子数组
    #[allow(dead_code)]
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut i, mut sum, mut res) = (0, 0, target + 1);
        for j in 0..nums.len() {
            sum += nums[j];
            while sum >= target {
                res = res.min((j - i + 1) as i32);
                sum -= nums[i];
                i += 1;
            }
        }
        if res == target + 1 {
            0
        } else {
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 200. 岛屿的数量
    #[test]
    fn num_islands() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        let res = Solution::num_islands(grid);
        assert_eq!(res, 1);
    }

    /// 202. 快乐数
    #[test]
    fn is_happy() {
        let n = 19;
        let res = Solution::is_happy(n);
        assert!(res);
    }

    /// 205. 同构字符串
    #[test]
    fn is_isomorphic() {
        let s = "egg".to_string();
        let t = "add".to_string();
        let res = Solution::is_isomorphic(s, t);
        assert!(res);
    }

    /// 209. 长度最小的子数组
    #[test]
    fn min_sub_array_len() {
        let target = 7;
        let nums = vec![2, 3, 1, 2, 4, 3];
        let res = Solution::min_sub_array_len(target, nums);
        assert_eq!(res, 2);
    }
}
