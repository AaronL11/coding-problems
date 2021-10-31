use std::io;
use std::io::BufRead;

#[allow(non_snake_case,dead_code)]
fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().lock().read_line(&mut buf)?;
    let n = buf.trim().parse::<u8>().unwrap();
    buf.clear();
    for _ in 0..n {
        io::stdin().lock().read_line(&mut buf)?;
        println!("{}",test_fn(&buf[..].trim()));
        buf.clear();
    }
    Ok(())
}

#[allow(non_snake_case,dead_code)]
fn test_fn<'a,'b>(input: &'a str) -> &'b str {
    let rec = &input.split(' ')
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()[..];
    let (r,e,c) = (rec[0],rec[1],rec[2]);
    if e-c > r {
        "advertise"
    } else if e-c == r {
        "does not matter"
    } else {
        "do not advertise"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nastyhacks_test1() {
        assert_eq!(
            test_fn("0 100 70"),
            "advertise"
        )
    }

    #[test]
    fn nastyhacks_test2() {
        assert_eq!(
            test_fn("100 130 30"),
            "does not matter"
        )
    }

    #[test]
    fn nastyhacks_test3() {
        assert_eq!(
            test_fn("-100 -70 40"),
            "do not advertise"
        )
    }
}
