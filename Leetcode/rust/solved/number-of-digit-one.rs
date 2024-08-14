use std::collections::HashMap;

struct DP {
    map: HashMap<i32, i32>,
}

impl DP {
    fn f(&mut self, n: i32) -> i32 {
        if let Some(&k) = self.map.get(&n) {
            k
        } else {
            let res = match n {
                0 => 0,
                1..=9 => 1,
                _ => {
                    let exp = (n as f64).log10().floor() as u32;
                    let m = 10i32.pow(exp);
                    let (top, bot) = (n / m, n % m);
                    self.f(bot) + self.f(top * m - 1) + if top == 1 { n - m + 1 } else { 0 }
                }
            };
            self.map.insert(n, res);
            res
        }
    }
}

impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        DP {
            map: HashMap::new(),
        }
        .f(n)
    }
}
