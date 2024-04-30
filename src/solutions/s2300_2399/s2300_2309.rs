use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    /// 2300. 咒语和药水的成功对数
    fn binary_search(potions: &[i32], spell: i64, success: i64) -> i32 {
        let (mut l, mut r) = (0, potions.len() as i32 - 1);
        while l <= r {
            let p = (l + r) / 2;
            match (potions[p as usize] as i64 * spell).cmp(&success) {
                Ordering::Greater | Ordering::Equal => {
                    r = p - 1;
                }
                Ordering::Less => {
                    l = p + 1;
                }
            }
        }
        potions.len() as i32 - l
    }

    #[allow(dead_code)]
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        potions.sort_unstable();
        spells
            .iter()
            .map(|&s| Self::binary_search(&potions, s as i64, success))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 2300. 咒语和药水的成功对数
    #[test]
    fn successful_pairs() {
        let spells = vec![5, 1, 3];
        let potions = vec![1, 2, 3, 4, 5];
        let success = 7;
        let res = Solution::successful_pairs(spells, potions, success);
        assert_eq!(res, vec![4, 0, 3]);
    }
}
