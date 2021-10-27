use std::io;
use std::io::Read;

#[allow(dead_code)]
fn main() {
    let mut buf: [u8;7] = [0;7];
    io::stdin().read(&mut buf).unwrap();
    println!(
        "{}",
        if buf.starts_with("555".as_bytes()){
            1
        } else {
            0
        }
    )
}
