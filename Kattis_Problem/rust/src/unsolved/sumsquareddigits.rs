use std::io;
use std::io::BufRead;

#[allow(non_snake_case,dead_code)]
fn main() -> io::Result<()> {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    stdin.read_line(&mut buf)?;
    let n = buf.parse::<u16>().unwrap();
    buf.clear();
    for _ in 0..n {
        stdin.read_line(&mut buf)?;
        let ins = &buf[..].trim().split(' ').collect::<Vec<&str>>();
        let (K,b,n) = (ins[0],ins[1].parse::<u8>().unwrap(),ins[2].parse::<u32>().unwrap());
        println!(
            "{} {}",
            K,
            ssd(b as u32, n)
            )
    }
    Ok(())
}

#[allow(non_snake_case,dead_code)]
fn ssd(b: u32, n: u32 )-> u32 {
    let (mut rem,mut x) = (0,n);
    let mut vec: Vec<u32> = Vec::new();
    while x > 0 {
        rem += n%b;
        x /= b;
        vec.push(rem);
    }
    vec.iter().fold(0, |acc,i| acc+i*i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sumsquareddigits_test2() {
        assert_eq!(
            ssd(10, 1234),
            30
        )
    }

    #[test]
    fn sumsquareddigits_test3() {
        assert_eq!(
            ssd(3, 98765),
            19
        )
    }

    #[test]
    fn sumsquareddigits_test1() {
        assert_eq!(
            ssd(16, 987654321),
            696
        )
    }
}
