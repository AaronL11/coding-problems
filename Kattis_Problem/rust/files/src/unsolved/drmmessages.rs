use std::io;
use io::{Read,Write};

fn main() -> io::Result<()> {
    let mut buf: [u8;15_001] = [0;15_001];
    let n = io::stdin().lock().read(&mut buf)?-1;
    for x in buf[..n].iter_mut() {
        *x -= 65;
    }
    let (l,r) = buf[..n].split_at_mut(n/2);
    let (sl,sr) = (
        l.iter().fold(0u16,|p,c| p+*c as u16),
        r.iter().fold(0u16,|p,c| p+*c as u16)
        );
    for i in 0..n/2 {
        l[i] = (
            ((l[i] as u16+sl)%26) as u8
            +
            ((r[i] as u16+sr)%26) as u8
        )%26+65
    }
    io::stdout().lock().write_all(&l)
}


