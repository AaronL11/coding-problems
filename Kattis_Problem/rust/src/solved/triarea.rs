use std::io;
use std::io::BufRead;

#[allow(non_snake_case,dead_code)]
fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().lock().read_line(&mut buf)?;
    Ok(println!(
        "{}",
        &buf[..].trim()
                .split(' ')
                .map(|s| s.parse::<f64>().unwrap())
                .product::<f64>()/2f64
    ))
}
