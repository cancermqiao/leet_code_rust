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

    /// 2512. 奖励最顶尖的 K 名学生
    #[allow(dead_code)]
    pub fn top_students(
        positive_feedback: Vec<String>,
        negative_feedback: Vec<String>,
        report: Vec<String>,
        student_id: Vec<i32>,
        k: i32,
    ) -> Vec<i32> {
        let positive_map: std::collections::HashMap<&str, i32> =
            positive_feedback.iter().map(|p| (p.as_str(), 3)).collect();
        let negative_map: std::collections::HashMap<&str, i32> =
            negative_feedback.iter().map(|p| (p.as_str(), -1)).collect();
        let mut student_scores = Vec::new();
        for i in 0..report.len() {
            let score: i32 = report[i]
                .split(' ')
                .map(|word| {
                    positive_map.get(word).unwrap_or(&0) + negative_map.get(word).unwrap_or(&0)
                })
                .sum();
            student_scores.push((-score, student_id[i]));
        }
        student_scores.sort();
        student_scores.truncate(k as usize);
        student_scores.iter().map(|s: &(i32, i32)| s.1).collect()
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

    /// 2512. 奖励最顶尖的 K 名学生
    #[test]
    fn top_students() {
        let positive_feedback = vec![
            "smart".to_string(),
            "brilliant".to_string(),
            "studious".to_string(),
        ];
        let negative_feedback = vec!["not".to_string()];
        let report = vec![
            "this student is studious".to_string(),
            "the student is smart".to_string(),
        ];
        let student_id = vec![1, 2];
        let k = 2;
        let res =
            Solution::top_students(positive_feedback, negative_feedback, report, student_id, k);
        assert_eq!(res, vec![1, 2]);
    }
}
