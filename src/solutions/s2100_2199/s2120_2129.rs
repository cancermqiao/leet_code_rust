pub struct Solution;

impl Solution {
    /// 2125. 银行中的激光束数量
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut res = 0;
        let device_array = bank
            .iter()
            .map(|s| {
                s.chars()
                    .map(|c| if c.eq(&'1') { 1 } else { 0 })
                    .sum::<i32>()
            })
            .filter(|&x| x > 0)
            .collect::<Vec<i32>>();
        if device_array.len() <= 1 {
            return 0;
        }
        for i in 0..(device_array.len() - 1) {
            res += device_array[i] * device_array[i + 1]
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 2125. 银行中的激光束数量
    #[test]
    pub fn number_of_beams() {
        let bank = vec![
            "011001".to_string(),
            "000000".to_string(),
            "010100".to_string(),
            "001000".to_string(),
        ];
        let res = Solution::number_of_beams(bank);
        assert_eq!(res, 8);
    }
}
