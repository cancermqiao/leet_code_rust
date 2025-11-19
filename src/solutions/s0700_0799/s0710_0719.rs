pub struct Solution;

impl Solution {
    /// 717. 1 比特与 2 比特字符
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut res = true;
        for (index, bit) in bits.iter().rev().enumerate() {
            if index == 0 {
                if *bit == 1 {
                    return false;
                }
            } else if *bit == 1 {
                res = !res;
            } else {
                break
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_one_bit_character() {
        let bits = vec![1, 1, 1, 0];
        let res = Solution::is_one_bit_character(bits);
        assert!(!res);
    }
}
