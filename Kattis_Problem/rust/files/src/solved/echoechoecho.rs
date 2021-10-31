use std::io;
use std::io::BufRead;

#[allow(dead_code)]
fn main() {
    let mut buf = String::new();
    io::stdin().lock().read_line(&mut buf).unwrap();
    buf.pop();
    buf.push(' ');
    println!(
        "{}",
        &buf[..].repeat(3).trim_end()
        )
}
