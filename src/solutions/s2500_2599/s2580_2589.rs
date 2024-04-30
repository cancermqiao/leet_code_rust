trait Vowel {
    fn is_vowel(&self) -> bool;
}

impl Vowel for char {
    fn is_vowel(&self) -> bool {
        matches!(*self, 'a' | 'e' | 'i' | 'o' | 'u')
    }
}

impl Vowel for String {
    fn is_vowel(&self) -> bool {
        let mut chars = self.chars();
        let first = chars.next().unwrap();
        if first.is_vowel() {
            if let Some(last) = chars.last() {
                last.is_vowel()
            } else {
                true
            }
        } else {
            false
        }
    }
}

pub struct Solution;

impl Solution {
    /// 2582. 递枕头
    #[allow(dead_code)]
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        if time / (n - 1) % 2 == 0 {
            1 + time % (n - 1)
        } else {
            n - time % (n - 1)
        }
    }

    /// 2586. 统计范围内的元音字符串数
    #[allow(dead_code)]
    pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        words
            .iter()
            .take(right as usize + 1)
            .skip(left as usize)
            .map(|word| word.is_vowel() as i32)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 2582. 递枕头
    #[test]
    fn pass_the_pillow() {
        let n = 4;
        let time = 5;
        let res = Solution::pass_the_pillow(n, time);
        assert_eq!(res, 2);
    }

    /// 2586. 统计范围内的元音字符串数
    #[test]
    fn vowel_strings() {
        let words = vec![
            "hey".to_string(),
            "aeo".to_string(),
            "mu".to_string(),
            "ooo".to_string(),
            "artro".to_string(),
        ];
        let left = 1;
        let right = 4;
        let res = Solution::vowel_strings(words, left, right);
        assert_eq!(res, 3);
    }
}
