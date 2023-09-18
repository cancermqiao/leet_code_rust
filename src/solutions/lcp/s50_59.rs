pub struct Solution {}

impl Solution {
    /// LCP 50. 宝石补给
    #[allow(dead_code)]
    pub fn give_gem(mut gem: Vec<i32>, operations: Vec<Vec<i32>>) -> i32 {
        for operation in operations {
            gem[operation[1] as usize] += gem[operation[0] as usize] / 2;
            gem[operation[0] as usize] -= gem[operation[0] as usize] / 2;
        }
        gem.iter().max().unwrap() - gem.iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // LCP 50. 宝石补给
    #[test]
    fn give_gem() {
        let gem = vec![3, 1, 2];
        let operations = vec![vec![0, 2], vec![2, 1], vec![2, 0]];
        let res = Solution::give_gem(gem, operations);
        assert_eq!(res, 2);
    }
}
