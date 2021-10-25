use std::io;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().lock().read_line(&mut buf)?;
    Ok(println!(
            "{}",
            &buf[..].as_bytes()
                .windows(3)
                .map(|w| match w {
                    [a,_,b] if a==b => 0,
                    [_,a,b] if a==b => 0,
                    [a,b,_] if a==b => 0,
                    _ => 1
                })
                .product::<u8>()
    ))
}
