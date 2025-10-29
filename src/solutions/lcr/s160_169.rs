pub struct Solution;

impl Solution {
    /// LCR 160. 珠宝的最高价值
    #[allow(dead_code)]
    pub fn jewellery_value(mut frame: Vec<Vec<i32>>) -> i32 {
        for i in 0..frame.len() {
            for j in 0..frame[0].len() {
                if i == 0 && j == 0 {
                    continue;
                }
                if i == 0 {
                    frame[i][j] += frame[i][j - 1];
                } else if j == 0 {
                    frame[i][j] += frame[i - 1][j];
                } else {
                    frame[i][j] += frame[i - 1][j].max(frame[i][j - 1]);
                }
            }
        }
        frame.last().unwrap().last().unwrap().to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// LCR 160. 珠宝的最高价值
    #[test]
    fn jewellery_value() {
        let frame = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        let res = Solution::jewellery_value(frame);
        assert_eq!(res, 12);
    }
}
