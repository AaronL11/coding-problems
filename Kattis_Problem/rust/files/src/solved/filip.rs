use std::io;
use std::io::BufRead;
use std::cmp::max;


#[allow(non_snake_case,dead_code)]
fn main() -> io::Result<()>{
    let mut buf = String::new();
    io::stdin().lock().read_line(&mut buf)?;
    Ok(println!(
        "{}",
        test_fn(&buf[..].trim())
        ))
}

#[allow(non_snake_case,dead_code)]
fn test_fn(input: &str) -> u16 {
    input.split(' ')
        .map(
            |s| s.chars()
                .rev()
                .collect::<String>()
                .parse::<u16>()
                .unwrap()
            )
        .fold(0, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filips_test1() {
        assert_eq!(
                test_fn("734 893"),
                437
            )
    }

    #[test]
    fn filips_test2() {
        assert_eq!(
            test_fn("221 231"),
            132
            )
    }
}
