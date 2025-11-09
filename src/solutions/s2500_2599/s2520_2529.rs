pub struct Solution;

impl Solution {
    /// 2525. 根据箱子的规则分类
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        let max_length = length.max(width).max(height);
        let vol = length as i64 * width as i64 * height as i64;
        let is_bulky = max_length >= 10000 || vol >= 1_000_000_000;
        let is_heavy = mass >= 100;
        match (is_bulky, is_heavy) {
            (true, false) => "Bulky",
            (false, true) => "Heavy",
            (true, true) => "Both",
            (false, false) => "Neither",
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 2525. 根据箱子的规则分类
    #[test]
    fn categorize_box() {
        let length = 1000;
        let width = 35;
        let height = 700;
        let mass = 300;
        let res = Solution::categorize_box(length, width, height, mass);
        assert_eq!(res, "Heavy".to_string());
    }
}
