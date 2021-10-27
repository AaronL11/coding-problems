use std::io;
use std::io::BufRead;

#[allow(dead_code)]
fn main() -> io::Result<()> {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    stdin.read_line(&mut buf)?;
    let n: u8 = buf.trim().parse().unwrap();
    buf.clear();
    for _ in 0..n {
        stdin.read_line(&mut buf)?;
        buf.push('\n');
    }
    
    Ok(println!(
        "{}",
        &buf[..].split('\n')
                .map(|c| c.trim().parse::<u8>().unwrap())
                .fold(1, |acc, l| acc+l-1)
    ))
}
