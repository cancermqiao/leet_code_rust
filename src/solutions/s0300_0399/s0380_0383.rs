pub struct Solution {}

impl Solution {
    // 383. 赎金信
    #[allow(dead_code)]
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false
        }
        let mut chars_nums = vec![0; 26];
        magazine.chars().for_each(|x| chars_nums[x as usize - 'a' as usize] += 1);
        for c in ransom_note.chars() {
            chars_nums[c as usize -'a' as usize ] -= 1;
            if chars_nums[c as usize -'a' as usize ] < 0 {
                return false
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 383. 赎金信
    #[test]
    fn can_construct() {
        let ransom_note = "a".to_string();
        let magazine = "b".to_string();
        let res = Solution::can_construct(ransom_note, magazine);
        assert_eq!(res, false);
    }
}