use rand::rngs::StdRng;
use rand::{self, Rng, SeedableRng};

fn rand7() -> i32 {
    let mut rng = StdRng::seed_from_u64(11);
    rng.random_range(1..=7)
}

pub struct Solution;

impl Solution {
    /// 470. 用 Rand7() 实现 Rand10()
    ///      平均调用次数
    ///      2 + 1 * 9/49 + 1 * 9/49 * 3/63 + 2 * 9/49 * 3/63 * 1/21 + 1 * 9/49 * 3/63 * 1/21 * 9/49 + ....
    ///      = (2 + 9/49 + 9/49 * 3/63) / (1 - 9/49 * 3/63 * 1/21)
    ///      = 2.193333
    pub fn rand10() -> i32 {
        let mut base = 1;
        let mut rand_num = 1;
        loop {
            while base < 10 {
                base *= 7;
                rand_num = (rand_num - 1) * 7 + rand7();
            }
            let quotient = base / 10;
            if rand_num <= quotient * 10 {
                return rand_num % 10 + 1;
            } else {
                rand_num %= 10;
                base %= 10;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 470. 用 Rand7() 实现 Rand10()
    #[test]
    fn rand10() {
        let res = Solution::rand10();
        assert_eq!(res, 10);
    }
}
