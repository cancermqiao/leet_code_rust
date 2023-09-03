pub struct Solution {}

impl Solution {
    /// 2511. 最多可以摧毁的敌人城堡数
    #[allow(dead_code)]
    pub fn capture_forts(forts: Vec<i32>) -> i32 {
        let (mut res, mut pre) = (0_i32, -1_i32);
        for (i, &fort) in forts.iter().enumerate() {
            if fort != 0 {
                if pre >= 0 && fort != forts[pre as usize] {
                    res = res.max(i as i32 - pre - 1);
                }
                pre = i as i32
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 2511. 最多可以摧毁的敌人城堡数
    #[test]
    fn capture_forts() {
        let forts = vec![0, -1, -1, 0, -1];
        let res = Solution::capture_forts(forts);
        assert_eq!(res, 0);
    }
}
