pub struct Solution;

impl Solution {
    /// 274. H指数
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, citations.len());
        while left < right {
            let mid = (left + right + 1) >> 1;
            let cnt = citations.iter().filter(|&v| *v >= mid as i32).count();
            if cnt >= mid {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        left as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 274. H指数
    #[test]
    fn h_index() {
        let citations = vec![3, 0, 6, 1, 5];
        let res = Solution::h_index(citations);
        assert_eq!(res, 3);
    }
}
