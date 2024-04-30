pub struct Solution;

impl Solution {
    /// 2798. 满足目标工作时长的员工数目
    #[allow(dead_code)]
    pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
        hours.iter().filter(|&x| *x >= target).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_employees_who_met_target() {
        let hours = vec![0, 1, 2, 3, 4];
        let target = 2;
        let res = Solution::number_of_employees_who_met_target(hours, target);
        assert_eq!(res, 3);
    }
}
