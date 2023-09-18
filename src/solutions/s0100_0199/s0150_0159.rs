pub struct Solution {}

impl Solution {
    /// 150. 逆波兰表达式求值
    #[allow(dead_code)]
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        tokens.iter().for_each(|token| {
            let num = match token.as_str() {
                "+" => stack.pop().unwrap() + stack.pop().unwrap(),
                "-" => -stack.pop().unwrap() + stack.pop().unwrap(),
                "*" => stack.pop().unwrap() * stack.pop().unwrap(),
                "/" => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    b / a
                }
                _ => token.parse().unwrap(),
            };
            stack.push(num);
        });
        stack.pop().unwrap()
    }

    /// 151. 反转字符串中的单词
    #[allow(dead_code)]
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }

    /// 152. 乘积最大子数组
    #[allow(dead_code)]
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut dp_max = vec![nums[0]];
        let mut dp_min = vec![nums[0]];
        for i in 1..nums.len() {
            dp_max.push(
                nums[i]
                    .max(nums[i] * dp_max[i - 1])
                    .max(nums[i] * dp_min[i - 1]),
            );
            dp_min.push(
                nums[i]
                    .min(nums[i] * dp_max[i - 1])
                    .min(nums[i] * dp_min[i - 1]),
            );
        }
        *dp_max.iter().max().unwrap()
    }

    /// 153. 寻找旋转排序数组中的最小值
    #[allow(dead_code)]
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        if nums[r] > nums[l] {
            return nums[l];
        }
        while l < r {
            let p = (l + r) / 2;
            if nums[p] >= nums[0] {
                l = p + 1;
            } else {
                r = p;
            }
        }
        nums[l]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 150. 逆波兰表达式求值
    #[test]
    fn eval_rpn() {
        let tokens = vec![
            "2".to_string(),
            "1".to_string(),
            "+".to_string(),
            "3".to_string(),
            "*".to_string(),
        ];
        let res = Solution::eval_rpn(tokens);
        assert_eq!(res, 9);
    }

    // 151. 反转字符串中的单词
    #[test]
    fn reverse_words() {
        let s = "  hello world  ".to_string();
        let res = Solution::reverse_words(s);
        assert_eq!(res, "world hello".to_string());
    }

    /// 152. 乘积最大子数组
    #[test]
    fn max_product() {
        let nums = vec![2, 3, -2, 4];
        let res = Solution::max_product(nums);
        assert_eq!(res, 6);
    }

    /// 153. 寻找旋转排序数组中的最小值
    #[test]
    fn find_min() {
        let nums = vec![3, 4, 5, 1, 2];
        let res = Solution::find_min(nums);
        assert_eq!(res, 1);
    }
}
