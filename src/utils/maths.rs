/// 欧几里得算法求最大公约数
/// 
/// # Example
/// ```
/// use leet_code_rust::utils::maths::gcd;
/// let gcd = gcd(4, 6);
/// assert_eq!(gcd, 2);
/// ```
pub fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 {
        return b;
    } else if b == 0 {
        return a;
    } else if a > b {
        return gcd(a % b, b);
    } else {
        return gcd(b % a, a);
    }
}

/// 最小公倍数
/// 
/// # Example
/// ```
/// use leet_code_rust::utils::maths::lcm;
/// let lcm = lcm(4, 6);
/// assert_eq!(lcm, 12);
/// ```
pub fn lcm(a: i32, b: i32) -> i32 {
    a * b / gcd(a, b)
}