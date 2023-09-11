pub struct Solution {}

impl Solution {
    /// 45. 跳跃游戏 II
    #[allow(dead_code)]
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
        return step;
    }

    /// 48. 旋转图像
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut cache: std::collections::HashMap<Vec<usize>, Vec<String>> =
            std::collections::HashMap::new();
        let a = 'a' as usize;
        for str in strs {
            let mut count = vec![0; 26];
            str.chars().for_each(|c| {
                count[c as usize - a] += 1;
            });
            cache.entry(count).or_insert(vec![]).push(str);
        }
        cache.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 45. 跳跃游戏 II
    #[test]
    fn jump() {
        let nums = vec![2, 3, 1, 1, 4];
        let step = Solution::jump(nums);
        assert_eq!(step, 2);
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
