pub struct Solution;

impl Solution {}

/// 2043. 简易银行系统
struct Bank {
    balance: Vec<i64>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        Self { balance }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        if account1 == account2 {
            if let Some(balance) = self.balance.get(account1 as usize - 1) {
                return *balance >= money;
            } else {
                return false;
            }
        }
        if let Ok([balance1, balance2]) = self
            .balance
            .get_disjoint_mut([account1 as usize - 1, account2 as usize - 1])
        {
            if *balance1 >= money {
                *balance1 -= money;
                *balance2 += money;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        if let Some(balance) = self.balance.get_mut(account as usize - 1) {
            *balance += money;
            true
        } else {
            false
        }
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        if let Some(balance) = self.balance.get_mut(account as usize - 1) {
            if *balance >= money {
                *balance -= money;
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bank() {
        let mut bank = Bank::new(vec![10, 100, 20, 50, 30]);
        let res = vec![
            bank.withdraw(3, 10),
            bank.transfer(5, 1, 20),
            bank.deposit(5, 20),
            bank.transfer(3, 4, 15),
            bank.withdraw(10, 50),
        ];
        assert_eq!(res, vec![true, true, true, false, false]);
    }
}
