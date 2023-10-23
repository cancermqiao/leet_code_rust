pub struct Solution {}

impl Solution {
    /// 2678. 老人的数目
    #[allow(dead_code)]
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details
            .iter()
            .filter(|d| d.get(11..13).unwrap() > "60")
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 2678. 老人的数目
    #[test]
    fn count_seniors() {
        let details = vec![
            "7868190130M7522".to_string(),
            "5303914400F9211".to_string(),
            "9273338290F4010".to_string(),
        ];
        let res = Solution::count_seniors(details);
        assert_eq!(res, 2);
    }
}
