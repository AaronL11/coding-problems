impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n)
            .map(|i| {
                if i % 15 == 0 {
                    String::from("FizzBuzz")
                } else if i % 3 == 0 {
                    String::from("Fizz")
                } else if i % 5 == 0 {
                    String::from("Buzz")
                } else {
                    format!("{}", i)
                }
            })
            .collect::<Vec<_>>()
    }
}
