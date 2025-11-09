pub struct Solution;

impl Solution {
    /// 40. 组合总和 II
    fn backtrace(
        freq: &Vec<(i32, usize)>,
        target: i32,
        buf: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
        pos: usize,
    ) {
        if target == 0 {
            res.push(buf.clone());
            return;
        }
        if pos == freq.len() {
            return;
        }
        Self::backtrace(freq, target, buf, res, pos + 1);
        let cnt = (target / freq[pos].0).min(freq[pos].1 as i32);
        for i in 1..=cnt {
            buf.push(freq[pos].0);
            Self::backtrace(freq, target - i * freq[pos].0, buf, res, pos + 1);
        }
        for _ in 1..=cnt {
            buf.pop();
        }
    }

    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let (mut buf, mut res) = (Vec::new(), Vec::new());
        if candidates.is_empty() {
            res
        } else {
            let (mut start, mut freq) = (0, Vec::new());
            candidates.sort();
            for i in 0..candidates.len() {
                if candidates[i] != candidates[start] {
                    freq.push((candidates[start], i - start));
                    start = i;
                }
            }
            freq.push((candidates[start], candidates.len() - start));
            Self::backtrace(&freq, target, &mut buf, &mut res, 0);
            res
        }
    }

    /// 41. 缺失的第一个正数
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut nums: Vec<i32> = nums
            .iter()
            .map(|&x| if x <= 0 { n as i32 + 1 } else { x })
            .collect();
        for i in 0..n {
            let num_abs = nums[i].unsigned_abs() as usize;
            if num_abs <= n {
                nums[num_abs - 1] = -nums[num_abs - 1].abs();
            }
        }
        for (i, &v) in nums.iter().enumerate() {
            if v > 0 {
                return i as i32 + 1;
            }
        }
        n as i32 + 1
    }

    /// 42. 接雨水
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut stack = Vec::new();
        let mut res = 0;
        for i in 0..height.len() {
            while !stack.is_empty() && height[i] > height[*stack.last().unwrap()] {
                let top = stack.pop().unwrap();
                if stack.is_empty() {
                    break;
                }
                let left = *stack.last().unwrap();
                res += (height[i].min(height[left]) - height[top]) * (i as i32 - left as i32 - 1);
            }
            stack.push(i);
        }
        res
    }

    /// 43. 字符串相乘
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == *"0" || num2 == *"0" {
            return "0".to_string();
        }
        let (m, n) = (num1.len(), num2.len());
        let (num1, num2) = (num1.as_bytes(), num2.as_bytes());
        let mut res = vec![0; m + n];
        for i in (0..num1.len()).rev() {
            for j in (0..num2.len()).rev() {
                let times = ((num1[i] - b'0') * (num2[j] - b'0')) as i32;
                res[i + j] += times / 10;
                res[i + j + 1] += times % 10;
            }
        }
        for i in (1..m + n).rev() {
            res[i - 1] += res[i] / 10;
            res[i] %= 10;
        }
        if res[0] == 0 {
            res.remove(0);
        }
        res.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    /// 44. 通配符匹配
    ///
    /// # 递归
    /// 超时
    fn is_match_recursion(s: &[u8], p: &[u8]) -> bool {
        match (s, p) {
            ([], []) | ([], [b'*']) => true,
            ([], [.., b'*']) => Self::is_match_recursion(s, &p[0..p.len() - 1]),
            ([], [.., _]) => false,
            ([.., _], []) => false,
            ([.., _], [.., b'?']) => {
                Self::is_match_recursion(&s[0..s.len() - 1], &p[0..p.len() - 1])
            }
            ([.., _], [.., b'*']) => {
                Self::is_match_recursion(&s[0..s.len() - 1], p)
                    || Self::is_match_recursion(s, &p[0..p.len() - 1])
            }
            ([.., a], [.., b]) => {
                a == b && Self::is_match_recursion(&s[0..s.len() - 1], &p[0..p.len() - 1])
            }
        }
    }

    /// # 动态规划
    fn is_match_dynamic(s: &[u8], p: &[u8]) -> bool {
        let (m, n) = (s.len(), p.len());
        let mut dp = vec![vec![false; n + 1]; m + 1];
        dp[0][0] = true;
        for i in 0..=m {
            for j in 1..=n {
                if i == 0 {
                    if p[j - 1] == b'*' {
                        dp[i][j] = true;
                    } else {
                        break;
                    }
                } else {
                    if p[j - 1] == b'?' || p[j - 1] == s[i - 1] {
                        dp[i][j] |= dp[i - 1][j - 1]
                    }
                    if p[j - 1] == b'*' {
                        dp[i][j] |= dp[i - 1][j] || dp[i][j - 1];
                    }
                }
            }
        }
        dp[m][n]
    }

    pub fn is_match(s: String, p: String, method_type: &str) -> bool {
        let (s, p) = (s.as_bytes(), p.as_bytes());
        match method_type {
            "recursion" => Self::is_match_recursion(s, p),
            "dynamic" => Self::is_match_dynamic(s, p),
            _ => Self::is_match_dynamic(s, p),
        }
    }

    /// 45. 跳跃游戏 II
    pub fn jump(nums: Vec<i32>) -> i32 {
        let (mut max_pos, mut step, mut end) = (0, 0, 0);
        for i in 0..nums.len() - 1 {
            max_pos = max_pos.max(i as i32 + nums[i]);
            if max_pos >= nums.len() as i32 - 1 {
                return step + 1;
            }
            if i as i32 == end {
                step += 1;
                end = max_pos;
            }
        }
        step
    }

    /// 46. 全排列
    fn dfs(nums: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, start: usize) {
        if start == nums.len() {
            res.push(nums.clone());
        } else {
            for i in start..nums.len() {
                nums.swap(i, start);
                Self::dfs(nums, res, start + 1);
                nums.swap(i, start);
            }
        }
    }

    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        Self::dfs(&mut nums, &mut res, 0);
        res
    }

    /// 47. 全排列 II
    fn dfs_unique(
        nums: &Vec<i32>,
        buf: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
        visited: &mut Vec<bool>,
    ) {
        if buf.len() == nums.len() {
            res.push(buf.clone());
        } else {
            for i in 0..nums.len() {
                if visited[i] || (i > 0 && nums[i] == nums[i - 1] && !visited[i - 1]) {
                    continue;
                }
                visited[i] = true;
                buf.push(nums[i]);
                Self::dfs_unique(nums, buf, res, visited);
                visited[i] = false;
                buf.pop();
            }
        }
    }

    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let (mut res, mut buf, mut visited) = (Vec::new(), Vec::new(), vec![false; nums.len()]);
        Self::dfs_unique(&nums, &mut buf, &mut res, &mut visited);
        res
    }

    /// 48. 旋转图像
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..(n / 2) {
            for j in i..(n - i - 1) {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[n - j - 1][i];
                matrix[n - j - 1][i] = matrix[n - i - 1][n - j - 1];
                matrix[n - i - 1][n - j - 1] = matrix[j][n - i - 1];
                matrix[j][n - i - 1] = tmp;
            }
        }
    }

    /// 49. 字母异分词
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut cache: std::collections::HashMap<Vec<usize>, Vec<String>> =
            std::collections::HashMap::new();
        let a = 'a' as usize;
        for str in strs {
            let mut count = vec![0; 26];
            str.chars().for_each(|c| {
                count[c as usize - a] += 1;
            });
            cache.entry(count).or_default().push(str);
        }
        cache.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 40. 组合总和 II
    #[test]
    fn combination_sum2() {
        let candidates = vec![10, 1, 2, 7, 6, 1, 5];
        let target = 8;
        let mut res = Solution::combination_sum2(candidates, target);
        res.sort();
        assert_eq!(
            res,
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        )
    }

    /// 41. 缺失的第一个正数
    #[test]
    fn first_missing_positive() {
        let nums = vec![1, 2, 0];
        let res = Solution::first_missing_positive(nums);
        assert_eq!(res, 3);
    }

    /// 42. 接雨水
    #[test]
    fn trap() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let res = Solution::trap(height);
        assert_eq!(res, 6);
    }

    /// 43. 字符串相乘
    #[test]
    fn multiply() {
        let num1 = "123".to_string();
        let num2 = "456".to_string();
        let res = Solution::multiply(num1, num2);
        assert_eq!(res, "56088".to_string());
    }

    /// 44. 通配符匹配
    #[test]
    fn is_match() {
        let s = "aa";
        let p = "*";
        let res = Solution::is_match(s.to_string(), p.to_string(), "recursion");
        assert!(res);
        let res = Solution::is_match(s.to_string(), p.to_string(), "dynamic");
        assert!(res);
    }

    /// 45. 跳跃游戏 II
    #[test]
    fn jump() {
        let nums = vec![2, 3, 1, 1, 4];
        let step = Solution::jump(nums);
        assert_eq!(step, 2);
    }

    /// 46. 全排列
    #[test]
    fn permute() {
        let nums = vec![1, 2, 3];
        let mut res = Solution::permute(nums);
        res.sort();
        assert_eq!(
            res,
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }

    /// 47. 全排列 II
    #[test]
    fn permute_unique() {
        let nums = vec![1, 1, 2];
        let mut res = Solution::permute_unique(nums);
        res.sort();
        assert_eq!(res, vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]);
    }

    /// 48. 旋转图像
    #[test]
    fn rotate() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }

    /// 49. 字母异分词
    #[test]
    fn group_anagrams() {
        let strs = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let mut res = Solution::group_anagrams(strs);
        for i in res.iter_mut() {
            i.sort();
        }
        res.sort();
        assert_eq!(
            res,
            vec![
                vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
                vec!["bat".to_string()],
                vec!["nat".to_string(), "tan".to_string()]
            ]
        )
    }
}
